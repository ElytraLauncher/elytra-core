//! 启动器更新下载配置（纯数据，无逻辑）。

use serde::{Deserialize, Serialize};

/// persist section `launcher_update`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherUpdateConfig {
    /// 下载目录（空 = 使用平台默认目录）
    pub download_dir: String,
}
