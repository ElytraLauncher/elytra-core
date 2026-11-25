//! JDK 相关契约（`jdk/`）。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 本机 JDK 信息。
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export_to = "jdk/")]
pub struct JdkInfo {
    pub path: String,
    pub version: String,
    pub vendor: String,
    pub bits: u8,
}

/// JDK 选择配置（`jdk` section）：当前选中的 JDK。
#[derive(Debug, Serialize, Deserialize, Default, TS)]
#[ts(export_to = "jdk/")]
pub struct JdkOptions {
    #[serde(default)]
    pub selection: JdkSelection,
}

/// 本机 JDK 扫描缓存（`jdk_scanned` section）。
#[derive(Debug, Serialize, Deserialize, Default, TS)]
#[ts(export_to = "jdk/")]
pub struct JdkScanCache {
    pub jdks: Vec<JdkInfo>,
}

/// jdk 选择模式：手动 + 自动，json 扁平化处理。
#[derive(Debug, Serialize, Deserialize, Default, Clone, TS)]
#[ts(export_to = "jdk/")]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum JdkSelection {
    #[default]
    Auto,
    Manual {
        path: String,
    },
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    JdkInfo::export().unwrap();
    JdkOptions::export().unwrap();
    JdkScanCache::export().unwrap();
    JdkSelection::export().unwrap();
}
