//! 启动器/实例/自动任务契约（`launcher/`）。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::jdk::JdkSelection;

/// 备份范围：全部世界 / 指定世界（`saves/` 下的目录名）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export_to = "launcher/")]
pub enum BackupScope {
    All,
    Selected { worlds: Vec<String> },
}

/// 单个自动任务动作：内置（Rust 实现）或自定义（系统命令行）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export_to = "launcher/")]
pub enum AutoTaskAction {
    /// 备份存档：排除 lock/tmp，按时间戳目录保留 `keep` 份。
    BackupSaves { scope: BackupScope, keep: u32 },
    /// 记录 mod 清单 JSON。
    RecordMods,
    /// 崩溃日志打包（含脱敏）。
    PackCrashLogs,
    /// 自定义命令行：Windows 走 `cmd /C`，其余走 `sh -c`。
    Command { command: String, timeout_secs: u32 },
}

/// 单个自动任务步骤（可单独停用而不删除）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export_to = "launcher/")]
pub struct AutoTaskStep {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub action: AutoTaskAction,
}

/// `pre_launch` 失败时的行为。
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export_to = "launcher/")]
pub enum AutoTaskFailure {
    /// 终止启动（默认）。
    #[default]
    Abort,
    /// 忽略失败，继续启动。
    Continue,
}

/// 三个时点的自动任务集合（全局与实例同构）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export_to = "launcher/")]
pub struct AutoTasks {
    pub pre_launch: Vec<AutoTaskStep>,
    pub on_crash: Vec<AutoTaskStep>,
    pub post_exit: Vec<AutoTaskStep>,
    pub pre_on_failure: AutoTaskFailure,
}

impl Default for AutoTasks {
    fn default() -> Self {
        Self {
            pre_launch: Vec::new(),
            on_crash: vec![AutoTaskStep {
                enabled: true,
                action: AutoTaskAction::PackCrashLogs,
            }],
            post_exit: vec![AutoTaskStep {
                enabled: true,
                action: AutoTaskAction::RecordMods,
            }],
            pre_on_failure: AutoTaskFailure::Abort,
        }
    }
}

/// 执行阶段（事件载荷与前端共用）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export_to = "launcher/")]
pub enum AutoTaskPhase {
    PreLaunch,
    OnCrash,
    PostExit,
}

/// 进度事件载荷（`auto-task-progress`）。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "launcher/")]
pub struct AutoTaskProgress {
    pub version_id: String,
    pub phase: AutoTaskPhase,
    /// 当前步骤下标（0-based，指向**排序后**的步骤列表）。
    pub step_index: u32,
    pub step_total: u32,
    pub action: String,
    pub message: String,
    /// 该步骤是否已完成（true = 收尾帧）。
    pub done: bool,
}

/// 自动任务失败信息。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "launcher/")]
pub struct AutoTaskFailureInfo {
    pub action: String,
    pub message: String,
}

/// 终态汇总事件载荷（`auto-tasks-finished`）。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "launcher/")]
pub struct AutoTasksFinished {
    pub version_id: String,
    pub phase: AutoTaskPhase,
    pub failures: Vec<AutoTaskFailureInfo>,
}

/// 实例自动任务状态：`global` 为默认值/"自定义"预填；`instance` 为 Some 时该实例采用自己的。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "launcher/")]
pub struct InstanceAutoTasksState {
    pub global: AutoTasks,
    pub instance: Option<AutoTasks>,
}

/// GC 类型。
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export_to = "launcher/")]
pub enum GcType {
    PreferZgc,
    PreferGenerationalZgc,
    StandardG1Gc,
    #[default]
    TunedG1Gc,
    Custom,
}

/// 内存分配模式。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
#[ts(export_to = "launcher/")]
pub enum MemoryMode {
    #[default]
    Auto,
    Custom(u32),
}

/// 游戏进程优先级（Windows 进程优先级类）。
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, TS)]
#[ts(export_to = "launcher/")]
pub enum ProcessPriority {
    Low,
    /// 中（平衡），默认。
    #[default]
    Normal,
    High,
    RealTime,
}

/// 游戏窗口尺寸。
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, TS)]
#[ts(export_to = "launcher/")]
pub enum WindowSize {
    /// 默认（不传 --width/--height，由游戏自己决定）。
    #[default]
    Default,
    Custom(u32, u32),
}

/// 全局启动参数。
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export_to = "launcher/")]
pub struct LaunchOptions {
    pub memory: MemoryMode,

    /// 最小堆 -Xms（MB）；None = 不设置。仅 Custom 模式生效。
    #[serde(default)]
    pub min_memory: Option<u32>,

    /// 游戏窗口标题；空 = 不改。
    #[serde(default)]
    pub window_title: String,

    #[serde(default)]
    pub process_priority: ProcessPriority,

    #[serde(default)]
    pub window_size: WindowSize,

    /// 禁用 Java Launch Wrapper。
    #[serde(default)]
    pub disable_launch_wrapper: bool,

    /// 禁用 LWJGL Unsafe Agent。
    #[serde(default)]
    pub disable_lwjgl_unsafe_agent: bool,

    /// 启动游戏前进行内存优化。
    #[serde(default)]
    pub pre_launch_memory_optimize: bool,

    /// 使用高性能显卡。
    #[serde(default = "default_true")]
    pub use_high_performance_gpu: bool,

    /// 游戏语言代码（如 zh_cn / en_us）；空 = 不干预。
    #[serde(default = "default_game_language")]
    pub game_language: String,

    /// 服务器直连：快速进服。None = 不指定。
    #[serde(default)]
    pub server: Option<String>,

    /// 服务器直连端口（配合 server 使用）。
    #[serde(default)]
    pub port: Option<u16>,

    /// 正版登录时验证 SSL 证书。
    #[serde(default = "default_true")]
    pub verify_ssl: bool,

    pub jvm_args: Vec<String>,
    pub game_args: Vec<String>,

    /// 自动任务（pre_launch / on_crash / post_exit）。
    #[serde(default)]
    pub auto_tasks: AutoTasks,

    #[serde(default)]
    pub gc_type: GcType,
}

/// 实例级启动参数快照：只包含“可以按实例覆盖”的字段。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "launcher/")]
pub struct InstanceLaunchOptions {
    #[serde(default)]
    pub memory: MemoryMode,
    #[serde(default)]
    pub min_memory: Option<u32>,
    #[serde(default)]
    pub window_title: String,
    #[serde(default)]
    pub process_priority: ProcessPriority,
    #[serde(default)]
    pub window_size: WindowSize,
    #[serde(default)]
    pub disable_launch_wrapper: bool,
    #[serde(default)]
    pub disable_lwjgl_unsafe_agent: bool,
    #[serde(default)]
    pub pre_launch_memory_optimize: bool,
    #[serde(default = "default_true")]
    pub use_high_performance_gpu: bool,
    #[serde(default = "default_game_language")]
    pub game_language: String,
    #[serde(default = "default_true")]
    pub verify_ssl: bool,
    #[serde(default)]
    pub jvm_args: Vec<String>,
    #[serde(default)]
    pub game_args: Vec<String>,
    #[serde(default)]
    pub gc_type: GcType,
    #[serde(default)]
    pub java: JdkSelection,
}

/// 实例级启动参数状态：`global` 为全局快照，`instance` 为实例快照，`effective` 为实际生效值。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "launcher/")]
pub struct InstanceLaunchState {
    pub global: InstanceLaunchOptions,
    pub instance: Option<InstanceLaunchOptions>,
    pub effective: InstanceLaunchOptions,
}

fn default_true() -> bool {
    true
}

fn default_game_language() -> String {
    "auto_zh_CN".into()
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    BackupScope::export().unwrap();
    AutoTaskAction::export().unwrap();
    AutoTaskStep::export().unwrap();
    AutoTaskFailure::export().unwrap();
    AutoTasks::export().unwrap();
    AutoTaskPhase::export().unwrap();
    AutoTaskProgress::export().unwrap();
    AutoTaskFailureInfo::export().unwrap();
    AutoTasksFinished::export().unwrap();
    InstanceAutoTasksState::export().unwrap();
    GcType::export().unwrap();
    MemoryMode::export().unwrap();
    ProcessPriority::export().unwrap();
    WindowSize::export().unwrap();
    LaunchOptions::export().unwrap();
    InstanceLaunchOptions::export().unwrap();
    InstanceLaunchState::export().unwrap();
}
