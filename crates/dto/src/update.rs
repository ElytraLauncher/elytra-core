//! 启动器更新契约（`update/`）。

use serde::Serialize;
use ts_rs::TS;

/// Release 中的一个可下载文件。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "update/")]
pub struct LauncherUpdateAsset {
    pub name: String,
    pub url: String,
    #[ts(type = "number")]
    pub size: u64,
    /// windows / macos / linux / other
    pub platform: String,
    /// x64 / arm64 / universal / x86 / other
    pub arch: String,
    /// setup / msi / portable / dmg / appimage / deb / rpm / archive / other
    pub kind: String,
    /// 当前平台+架构下的首选文件
    pub recommended: bool,
}

/// 下载更新对话框的完整数据：平台信息、文件清单与下载目录。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "update/")]
pub struct LauncherUpdatePlan {
    pub version: String,
    pub release_url: String,
    pub platform: String,
    pub arch: String,
    /// 生效目录（配置值或平台默认值）
    pub download_dir: String,
    /// 平台默认目录（用于「恢复默认」）
    pub default_download_dir: String,
    pub assets: Vec<LauncherUpdateAsset>,
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    LauncherUpdateAsset::export().unwrap();
    LauncherUpdatePlan::export().unwrap();
}
