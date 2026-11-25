//! 服务器状态契约（`server_status/`）。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 服务器状态（前端展示；serde 直出 JSON 给 JS）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export_to = "server_status/")]
pub struct ServerStatus {
    /// 版本类型："java" | "bedrock"。
    pub edition: String,
    /// 版本名（如 "1.21.4" / "1.21.44"）。
    pub version_name: String,
    /// Java：SLP 协议号；Bedrock：无可得 → None。
    pub protocol: Option<i32>,
    /// 在线玩家数。
    pub online_players: u32,
    /// 最大玩家数。
    pub max_players: u32,
    /// MOTD（§ 格式原始文本）。
    pub motd: String,
    /// 在线玩家样例（Bedrock 无此信息 → 空）。
    pub player_sample: Vec<String>,
    /// 服务器图标（"data:image/png;base64,..."，服务器未提供 → None）。
    pub favicon: Option<String>,
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    ServerStatus::export().unwrap();
}
