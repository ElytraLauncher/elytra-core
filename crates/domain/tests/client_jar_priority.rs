//! 阶段 C 顺序回归：client.jar 完成前不得启动库/资源下载。
//!
//! 慢网下 64 路资源会抢走带宽，加载器等 client.jar 120s 无进度会判 `vanillaClientTimeout`。

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use elytra_domain::config::downloader::{DownloaderConfig, NetworkMode};
use elytra_domain::downloader::hash::sha1_hex;
use elytra_domain::persist;
use elytra_domain::task::model::{DownloadTask, TaskStatus};
use elytra_domain::task::runner::run_task;
use elytra_domain::task::{self, EventSink};
use parking_lot::Mutex;
use serde_json::json;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

#[derive(Default)]
struct RecordingSink {
    events: Mutex<Vec<&'static str>>,
}

impl EventSink for RecordingSink {
    fn send(&self, event: &'static str, _payload: serde_json::Value) {
        self.events.lock().push(event);
    }
}

fn step_specs() -> Vec<task::StepSpec> {
    vec![task::StepSpec {
        name: "测试步骤".into(),
        kind: task::StepKind::File,
    }]
}

fn response(status: &str, content_type: &str, body: &[u8]) -> Vec<u8> {
    let mut out = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    out.extend_from_slice(body);
    out
}

async fn read_request(socket: &mut tokio::net::TcpStream) -> String {
    let mut buf = vec![0u8; 8192];
    let n = socket.read(&mut buf).await.unwrap_or(0);
    String::from_utf8_lossy(&buf[..n]).into_owned()
}

#[tokio::test(flavor = "multi_thread")]
async fn stage_c_waits_for_client_jar_before_assets() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base = format!("http://{addr}");

    let client_body = vec![0x5a_u8; 256 * 1024];
    let library_body = b"test-library".to_vec();
    let index_body = br#"{"objects":{}}"#.to_vec();

    let client_sha1 = sha1_hex(&client_body);
    let library_sha1 = sha1_hex(&library_body);
    let index_sha1 = sha1_hex(&index_body);
    let version = json!({
        "assetIndex": {
            "id": "test",
            "sha1": index_sha1,
            "size": index_body.len(),
            "totalSize": 0,
            "url": format!("{base}/assets.json")
        },
        "assets": "test",
        "complianceLevel": 1,
        "downloads": {
            "client": {
                "url": format!("{base}/client.jar"),
                "sha1": client_sha1,
                "size": client_body.len()
            }
        },
        "id": "test",
        "javaVersion": { "majorVersion": 17 },
        "libraries": [{
            "name": "test:lib:1",
            "downloads": {
                "artifact": {
                    "path": "test/lib/1/lib-1.jar",
                    "url": format!("{base}/test/lib/1/lib-1.jar"),
                    "sha1": library_sha1,
                    "size": library_body.len()
                }
            },
            "rules": null
        }],
        "mainClass": "test.Main",
        "minimumLauncherVersion": 0,
        "releaseTime": "2024-01-01T00:00:00+00:00",
        "time": "2024-01-01T00:00:00+00:00",
        "type": "release"
    });
    let version_body = serde_json::to_vec(&version).unwrap();

    let client_done = Arc::new(AtomicBool::new(false));
    let library_before_client_done = Arc::new(AtomicBool::new(false));
    let server_client_done = Arc::clone(&client_done);
    let server_library_early = Arc::clone(&library_before_client_done);

    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                break;
            };
            let version_body = version_body.clone();
            let index_body = index_body.clone();
            let client_body = client_body.clone();
            let library_body = library_body.clone();
            let client_done = Arc::clone(&server_client_done);
            let library_before_client_done = Arc::clone(&server_library_early);
            tokio::spawn(async move {
                let request = read_request(&mut socket).await;
                let path = request
                    .lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .unwrap_or("/");
                let bytes = match path {
                    "/version.json" => response("200 OK", "application/json", &version_body),
                    "/assets.json" => response("200 OK", "application/json", &index_body),
                    "/client.jar" => {
                        let head = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            client_body.len()
                        );
                        socket.write_all(head.as_bytes()).await.ok();
                        let chunks = client_body.chunks(64 * 1024).count();
                        for (index, chunk) in client_body.chunks(64 * 1024).enumerate() {
                            socket.write_all(chunk).await.ok();
                            socket.flush().await.ok();
                            if index + 1 < chunks {
                                tokio::time::sleep(Duration::from_millis(120)).await;
                            }
                        }
                        client_done.store(true, Ordering::SeqCst);
                        let _ = socket.shutdown().await;
                        return;
                    }
                    "/test/lib/1/lib-1.jar" => {
                        if !client_done.load(Ordering::SeqCst) {
                            library_before_client_done.store(true, Ordering::SeqCst);
                        }
                        response("200 OK", "application/java-archive", &library_body)
                    }
                    _ => response("404 Not Found", "text/plain", b"missing"),
                };
                socket.write_all(&bytes).await.ok();
                let _ = socket.shutdown().await;
            });
        }
    });

    let previous_config = persist::downloader_config();
    let game_dir = tempfile::tempdir().unwrap();
    let mut config = DownloaderConfig::default();
    config.minecraft_dir = game_dir.path().display().to_string();
    config.network_mode = NetworkMode::Direct;
    config.concurrency = 4;
    persist::save_downloader_config(&config).unwrap();

    let sink: Arc<dyn EventSink> = Arc::new(RecordingSink::default());
    let task = DownloadTask {
        version_id: "test".into(),
        version_url: format!("{base}/version.json"),
        mc_version: "test".into(),
        loader: None,
        optifine_patch: None,
        version_dir_name: Some("test-priority".into()),
        fabric_api: None,
        display_name: None,
    };
    let (_task_id, cancel, progress) = task::global().create(
        "client-jar-priority",
        &task::TaskText::default(),
        &step_specs(),
    );

    run_task(sink, &cancel, &progress, &task).await;

    persist::save_downloader_config(&previous_config).unwrap();

    assert_eq!(
        progress.lock().status,
        TaskStatus::Done,
        "顺序测试任务应成功，error={:?}",
        progress.lock().error
    );
    assert!(
        !library_before_client_done.load(Ordering::SeqCst),
        "client.jar 完成前不应发起库下载"
    );
    assert!(client_done.load(Ordering::SeqCst), "client.jar 应完整下载");
}
