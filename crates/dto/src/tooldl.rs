//! 工具箱下载器契约（`tooldl/`）。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 任务状态（前端镜像同名小写字符串）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export_to = "tooldl/")]
#[serde(rename_all = "lowercase")]
pub enum ToolDlStatus {
    /// 已入队，等待空闲槽位。
    Queued,
    Downloading,
    /// 用户暂停（保留 `.part` 与位图，继续时断点续传）。
    Paused,
    Done,
    Error,
    /// 用户取消（中间产物已清理）。
    Cancelled,
}

/// 对外快照（前端手写同名 interface）。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "tooldl/")]
pub struct ToolDlSnapshot {
    pub id: String,
    pub url: String,
    pub file_name: String,
    /// 最终落地路径
    pub dest: String,
    pub save_dir: String,
    /// 总字节（0 = 未知）
    #[ts(type = "number")]
    pub total: u64,
    #[ts(type = "number")]
    pub downloaded: u64,
    /// 字节/秒（0 = 未在下载）
    #[ts(type = "number")]
    pub speed: u64,
    /// 是否走 Range 分段
    pub ranged: bool,
    /// 实际分段数（1 = 单连接）
    pub segments: u32,
    pub status: ToolDlStatus,
    pub error: Option<String>,
    #[ts(type = "number")]
    pub created_at: u64,
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    ToolDlStatus::export().unwrap();
    ToolDlSnapshot::export().unwrap();
}
