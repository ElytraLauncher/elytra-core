//! 宿主注入：身份/路径覆盖必须在任何持久化或 UA 使用前生效。

use elytra_domain::constants;
use elytra_domain::host::{self, Config, Identity, Paths};
use elytra_domain::util::config_dir;

#[test]
fn host_overrides_identity_and_paths() {
    let dir = tempfile::tempdir().expect("tempdir");
    let config_root = dir.path().join("custom-config");
    host::init(Config {
        identity: Identity {
            app_version: "9.9.9".into(),
            curseforge_key: Some("test-key".into()),
            config_dir_name: "CustomElytra".into(),
            ..Default::default()
        },
        paths: Paths {
            config_root: Some(config_root.clone()),
            ..Default::default()
        },
    })
    .expect("首次初始化");

    assert_eq!(constants::app_version(), "9.9.9");
    assert_eq!(constants::ua_download(), "Elytra Launcher/9.9.9");
    assert_eq!(constants::curseforge_key(), Some("test-key"));
    assert_eq!(config_dir(), config_root);
}
