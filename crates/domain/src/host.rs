//! 宿主注入：应用身份、路径覆盖与运行时句柄。
//!
//! 一个进程只允许一次 [`init`]；CLI/TUI 必须在任何持久化/下载/认证调用前完成初始化。
//! 未初始化时使用 Elytra 默认值，便于 core 自身测试与独立二进制。

use std::path::PathBuf;
use std::sync::OnceLock;

use crate::constants;
use crate::error::CoreError;

#[derive(Clone, Debug)]
pub struct Identity {
    pub app_name: String,
    pub app_version: String,
    pub launcher_brand: String,
    pub client_id: String,
    pub config_dir_name: String,
    pub keychain_service: String,
    pub curseforge_key: Option<String>,
}

impl Default for Identity {
    fn default() -> Self {
        Self {
            app_name: constants::APP_NAME.to_string(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            launcher_brand: constants::LAUNCHER_BRAND.to_string(),
            client_id: constants::MS_CLIENT_ID.to_string(),
            config_dir_name: constants::CONFIG_DIR.to_string(),
            keychain_service: constants::KEYCHAIN_SERVICE.to_string(),
            curseforge_key: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Paths {
    /// 精确配置目录；None = 平台数据目录 + `identity.config_dir_name`。
    pub config_root: Option<PathBuf>,
    /// 默认游戏目录基准；None = 现有便携逻辑。
    pub game_dir_base: Option<PathBuf>,
    /// 默认下载目录；None = 平台下载目录。
    pub downloads_dir: Option<PathBuf>,
}

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub identity: Identity,
    pub paths: Paths,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init(config: Config) -> Result<(), CoreError> {
    CONFIG.set(config).map_err(|_| {
        CoreError::new("host 已初始化，不能重复设置身份/路径")
            .with_kind(crate::error::ErrorKind::Config)
    })
}

pub fn config() -> &'static Config {
    CONFIG.get_or_init(Config::default)
}
