//! 契约漂移回归：core 的序列化 JSON 必须能被 DTO 原样解析并重新序列化。

use serde::de::DeserializeOwned;
use serde::Serialize;

fn roundtrip<C, D>(core: &C)
where
    C: Serialize,
    D: DeserializeOwned + Serialize,
{
    let json = serde_json::to_value(core).expect("core 序列化");
    let dto: D = serde_json::from_value(json.clone()).expect("DTO 反序列化");
    assert_eq!(serde_json::to_value(&dto).expect("DTO 序列化"), json);
}

#[test]
fn defaults_match_core() {
    roundtrip::<_, elytra_dto::config::DownloaderConfig>(
        &elytra_domain::config::downloader::DownloaderConfig::default(),
    );
    roundtrip::<_, elytra_dto::config::GameDirsConfig>(
        &elytra_domain::config::game_dirs::GameDirsConfig::default(),
    );
    roundtrip::<_, elytra_dto::config::LauncherUpdateConfig>(
        &elytra_domain::config::launcher_update::LauncherUpdateConfig::default(),
    );
    roundtrip::<_, elytra_dto::config::OnlineConfig>(
        &elytra_domain::config::online::OnlineConfig::default(),
    );
    roundtrip::<_, elytra_dto::config::ToolDlConfig>(
        &elytra_domain::config::tooldl::ToolDlConfig::default(),
    );
    roundtrip::<_, elytra_dto::jdk::JdkOptions>(&elytra_domain::jdk::model::JdkOptions::default());
    roundtrip::<_, elytra_dto::jdk::JdkScanCache>(
        &elytra_domain::jdk::model::JdkScanCache::default(),
    );
    roundtrip::<_, elytra_dto::launcher::AutoTasks>(
        &elytra_domain::launcher::auto_tasks::model::AutoTasks::default(),
    );
    roundtrip::<_, elytra_dto::launcher::LaunchOptions>(
        &elytra_domain::launcher::model::LaunchOptions::default(),
    );
    roundtrip::<_, elytra_dto::launcher::InstanceLaunchOptions>(
        &elytra_domain::launcher::model::InstanceLaunchOptions::from_launch_options(
            &elytra_domain::launcher::model::LaunchOptions::default(),
            elytra_domain::jdk::model::JdkSelection::default(),
        ),
    );
}

#[test]
fn enum_variants_match_core() {
    use elytra_domain::config::downloader::{
        DownloadSourceMode as CoreSourceMode, JdkSourceMode as CoreJdkMode,
        NetworkMode as CoreNetworkMode,
    };
    use elytra_dto::config::{
        DownloadSourceMode as DtoSourceMode, JdkSourceMode as DtoJdkMode,
        NetworkMode as DtoNetworkMode,
    };

    roundtrip::<_, DtoNetworkMode>(&CoreNetworkMode::Auto);
    roundtrip::<_, DtoNetworkMode>(&CoreNetworkMode::System);
    roundtrip::<_, DtoNetworkMode>(&CoreNetworkMode::Direct);
    roundtrip::<_, DtoSourceMode>(&CoreSourceMode::OfficialOnly);
    roundtrip::<_, DtoSourceMode>(&CoreSourceMode::MirrorOnly);
    roundtrip::<_, DtoSourceMode>(&CoreSourceMode::Auto);
    roundtrip::<_, DtoJdkMode>(&CoreJdkMode::OfficialOnly);
    roundtrip::<_, DtoJdkMode>(&CoreJdkMode::MirrorOnly);
    roundtrip::<_, DtoJdkMode>(&CoreJdkMode::Auto);
}
