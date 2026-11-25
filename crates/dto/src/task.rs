//! 任务/下载相关契约（`task/`）。

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 任务状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default, TS)]
#[ts(export_to = "task/")]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    #[default]
    Pending,
    Running,
    Done,
    Error,
    Cancelled,
    /// 用户暂停（任务从内存移除，快照保留可恢复）。
    Paused,
}

/// 步骤状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default, TS)]
#[ts(export_to = "task/")]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    #[default]
    Pending,
    Running,
    Done,
}

/// 步骤种类：前端按枚举翻译步骤名（`File` 仍显示文件名）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export_to = "task/")]
#[serde(rename_all = "snake_case")]
pub enum StepKind {
    /// 下载原版 version json。
    VersionJson,
    /// client.jar。
    ClientJar,
    Libraries,
    Assets,
    /// native classifier 下载/解压。
    Natives,
    /// ModLoader 配置（合并 json 写入）。
    LoaderConfig,
    /// ModLoader 本体。
    LoaderLibs,
    #[serde(rename = "optifine")]
    OptiFine,
    FabricApi,
    /// managed JDK 运行时安装。
    JavaRuntime,
    /// 修复任务：客户端主体。
    RepairClient,
    /// 修复任务：支持库。
    RepairLibraries,
    /// 修复任务：资源文件。
    RepairAssets,
    /// 修复任务：原生库。
    RepairNatives,
    /// 整合包安装：下载包体。
    ModpackDownload,
    /// 整合包安装：解析清单。
    ModpackParse,
    /// 整合包安装：版本清单。
    ModpackVersionJson,
    /// 整合包安装：游戏主文件。
    ModpackGame,
    /// 整合包安装：依赖库。
    ModpackLibraries,
    /// 整合包安装：资源文件。
    ModpackAssets,
    /// 整合包安装：原生库。
    ModpackNatives,
    /// 整合包安装：ModLoader 配置。
    ModpackLoaderConfig,
    /// 整合包安装：ModLoader 文件。
    ModpackLoaderFiles,
    /// 整合包安装：解压 overrides。
    ModpackOverrides,
    /// 整合包导出：扫描/匹配在线文件。
    ExportScan,
    /// 整合包导出：写入 zip。
    ExportPack,
    /// 批量文件下载的单个文件步骤。
    File,
}

/// 任务进度快照（task-progress 事件推送体）。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "task/")]
pub struct TaskProgress {
    pub task_id: String,
    pub name: String,
    /// 任务名的 i18n key；旧快照/未迁移任务为 None，前端回退 name。
    pub name_key: Option<String>,
    pub name_params: Option<BTreeMap<String, String>>,
    pub status: TaskStatus,
    pub progress: f64,
    pub speed_bps: f64,
    /// 预计剩余秒数；停滞或速度过低时 None。
    #[ts(type = "number | null")]
    pub eta_seconds: Option<u64>,
    pub completed_files: u32,
    pub total_files: u32,
    #[ts(type = "number")]
    pub completed_bytes: u64,
    #[ts(type = "number")]
    pub total_bytes: u64,
    pub current_phase: String,
    /// 阶段的 i18n key。
    pub current_phase_key: Option<String>,
    pub current_phase_params: Option<BTreeMap<String, String>>,
    pub steps: Vec<StepInfo>,
    pub error: Option<String>,
}

/// 单个步骤信息。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "task/")]
pub struct StepInfo {
    pub name: String,
    /// 步骤枚举；None 时前端显示 name。
    pub kind: Option<StepKind>,
    pub status: StepStatus,
    pub progress: f64,
}

/// ModLoader 种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export_to = "task/")]
pub enum LoaderKind {
    #[serde(rename = "fabric")]
    Fabric,
    #[serde(rename = "forge")]
    Forge,
    #[serde(rename = "neoforge")]
    NeoForge,
    #[serde(rename = "quilt")]
    Quilt,
}

/// ModLoader 选择（种类 + 版本）。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "task/")]
pub struct LoaderChoice {
    pub kind: LoaderKind,
    pub version: String,
}

/// Fabric API 下载信息。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "task/")]
pub struct FabricApi {
    pub url: String,
    pub version: String,
}

/// 下载游戏本体任务（原版，或带 ModLoader/OptiFine/Fabric API）。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "task/")]
pub struct DownloadTask {
    pub version_id: String,
    pub version_url: String,
    pub mc_version: String,
    #[serde(default)]
    pub loader: Option<LoaderChoice>,
    #[serde(default)]
    pub optifine_patch: Option<String>,
    #[serde(default)]
    pub version_dir_name: Option<String>,
    #[serde(default)]
    pub fabric_api: Option<FabricApi>,
    #[serde(default)]
    pub display_name: Option<String>,
}

/// 单文件下载种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export_to = "task/")]
pub enum DownloadFileKind {
    #[serde(rename = "mod")]
    Mod,
    #[serde(rename = "resourcepack")]
    ResourcePack,
    #[serde(rename = "datapack")]
    DataPack,
    #[serde(rename = "shader")]
    Shader,
    #[serde(rename = "world")]
    World,
}

/// task-waiting-user 事件 payload。
#[derive(Serialize, Clone, TS)]
#[ts(export_to = "task/")]
pub struct WaitingUserPayload {
    pub task_id: String,
    pub file_name: String,
    pub error: String,
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    TaskStatus::export().unwrap();
    TaskProgress::export().unwrap();
    StepStatus::export().unwrap();
    StepKind::export().unwrap();
    StepInfo::export().unwrap();
    LoaderKind::export().unwrap();
    LoaderChoice::export().unwrap();
    FabricApi::export().unwrap();
    DownloadTask::export().unwrap();
    DownloadFileKind::export().unwrap();
    WaitingUserPayload::export().unwrap();
}
