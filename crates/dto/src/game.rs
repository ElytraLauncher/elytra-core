//! 游戏/版本相关契约（`game/`、`community/`）。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// CurseForge Key 来源（只暴露状态，不返回 key 本身）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[ts(export_to = "community/")]
#[serde(rename_all = "camelCase")]
pub enum CurseForgeKeySource {
    /// 用户在设置中填写的 key（优先）。
    User,
    /// 构建期内置 key。
    BuiltIn,
    /// 两者都没有。
    None,
}

/// 单个 Mod 的升级兼容状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export_to = "game/")]
pub enum UpgradeModStatus {
    Compatible,
    /// 当前文件不兼容，但 Modrinth 上存在目标 MC / Loader 的版本。
    Update,
    Incompatible,
    Unknown,
}

/// 单个 Mod 的兼容性结果。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "game/")]
pub struct UpgradeModReport {
    pub file_name: String,
    pub mod_id: Option<String>,
    /// 当前文件在 Modrinth 上的版本号（未识别为 None）。
    pub current_version: Option<String>,
    /// 目标版本可用的 Modrinth 版本号（仅 status=update）。
    pub compatible_version: Option<String>,
    pub status: UpgradeModStatus,
    /// 语言无关的原因码。
    pub reason: Option<String>,
    /// 原因参数（声明的版本范围 / 支持的 Loader 列表等）。
    pub reason_detail: Option<String>,
}

/// 兼容性报告。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "game/")]
pub struct UpgradeReport {
    pub mods: Vec<UpgradeModReport>,
    pub compatible: u32,
    pub update: u32,
    pub incompatible: u32,
    pub unknown: u32,
    /// Modrinth 查询失败过（结果可能降级为本地元数据）。
    pub network_error: bool,
}

/// 已安装版本概要。
#[derive(Debug, Clone, Serialize, TS, PartialEq)]
#[ts(export_to = "game/")]
pub struct InstalledVersionInfo {
    pub name: String,
    /// 从版本 json 推断的 loader；json 读取失败时为空串
    pub loader: String,
}

/// 升级备份元数据（持久化在 `versions/.elytra-upgrade-<旧名>/backup.json`）。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "game/")]
pub struct UpgradeBackupMeta {
    pub from_version: String,
    /// 提交时写入的最终目录名；未提交为 None。
    pub to_version: Option<String>,
    /// 旧目录名是否由 MC / Loader 自动生成；自定义实例名保持原名。
    #[serde(default = "default_true")]
    pub rename_to_target: bool,
    pub target_mc: String,
    /// `vanilla` / `fabric` / `forge` / `neoforge` / `quilt`。
    pub target_loader: String,
    pub target_loader_version: Option<String>,
    /// 升级完成后要改名为 `.disabled` 的 Mod 文件名。
    pub disable_mods: Vec<String>,
    #[ts(type = "number")]
    pub created_at: u64,
    /// 安装任务已成功结束（由 `mark_ready` 写入 ready 标记）。
    #[serde(default)]
    pub install_done: bool,
}

/// 实例升级页的当前状态。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "game/")]
pub struct InstanceUpgradeInfo {
    pub mc_version: String,
    pub loader: String,
    pub loader_version: Option<String>,
    pub mod_count: u32,
    /// 该版本目录正被下载/升级任务占用。
    pub busy: bool,
    pub pending: Option<UpgradeBackupMeta>,
}

/// 升级提交结果。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "game/")]
pub struct UpgradeCommitResult {
    pub old_version: String,
    pub new_version: String,
}

/// 升级回滚结果。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "game/")]
pub struct UpgradeRollbackResult {
    pub restored_version: String,
}

/// 版本列表变化事件。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "game/")]
pub struct VersionsChanged {
    /// 激活游戏目录绝对路径（前端比对，丢弃切目录前发出的迟到事件）。
    pub dir: String,
    /// 最新全量列表。
    pub versions: Vec<InstalledVersionInfo>,
}

/// 聚焦版本目录内资源子目录变化事件。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "game/")]
pub struct VersionDirChanged {
    pub version: String,
    /// 发生变化的一级子目录名（`saves`/`mods`/…）。
    pub kinds: Vec<String>,
}

/// 激活目录 + 版本列表一次返回。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "game/")]
pub struct VersionState {
    pub active_dir: String,
    pub versions: Vec<InstalledVersionInfo>,
}

/// 目标实例的 MC 版本与 ModLoader（Mod 下载前校验）。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "game/")]
pub struct InstanceModTarget {
    pub mc_version: String,
    pub loader: String,
}

fn default_true() -> bool {
    true
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    CurseForgeKeySource::export().unwrap();
    UpgradeModStatus::export().unwrap();
    UpgradeModReport::export().unwrap();
    UpgradeReport::export().unwrap();
    InstalledVersionInfo::export().unwrap();
    VersionState::export().unwrap();
    InstanceModTarget::export().unwrap();
    InstanceUpgradeInfo::export().unwrap();
    UpgradeBackupMeta::export().unwrap();
    UpgradeCommitResult::export().unwrap();
    UpgradeRollbackResult::export().unwrap();
    VersionsChanged::export().unwrap();
    VersionDirChanged::export().unwrap();
}
