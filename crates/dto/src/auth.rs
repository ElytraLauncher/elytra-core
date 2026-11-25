//! 账号相关契约（`auth/`）。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 第三方验证服务器账号。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "auth/")]
pub struct ExternalAccount {
    /// 验证服务器根地址（如 https://littleskin.cn/api/yggdrasil）
    pub api_url: String,
    /// Minecraft profile uuid（无连字符）
    pub uuid: String,
    pub name: String,
    /// 服务器签发的 access_token（启动游戏用）
    pub access_token: String,
    /// 皮肤站登录账号（authenticate 响应 user.username；旧账号为 None）
    #[serde(default)]
    pub site_username: Option<String>,
    /// 皮肤站名称（元数据探测 GET {api}/api/yggdrasil → meta.serverName；旧账号为 None）
    #[serde(default)]
    pub server_name: Option<String>,
    /// Yggdrasil clientToken（refresh 换角色/续期用；旧账号为 None）
    #[serde(default)]
    pub client_token: Option<String>,
    /// 账号全部角色（authenticate 响应 availableProfiles；切换角色用）
    #[serde(default)]
    pub profiles: Vec<ExternalProfile>,
}

/// 第三方账号下的角色。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "auth/")]
pub struct ExternalProfile {
    pub id: String,
    pub name: String,
}

/// 预设验证服务器（label 展示名 + value API 地址）。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "auth/")]
pub struct ExternalServer {
    pub label: String,
    pub value: String,
}

/// 第三方登录配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export_to = "auth/")]
pub struct ExternalConfig {
    #[serde(default)]
    pub servers: Vec<ExternalServer>,
}

/// Microsoft 正版账号。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "auth/")]
pub struct MsAccount {
    /// Minecraft profile uuid（无连字符）
    pub uuid: String,
    pub name: String,
    /// Minecraft access_token（短时有效，启动游戏用）
    pub access_token: String,
    /// 微软 refresh_token（长期凭证，用于静默刷新）
    pub refresh_token: String,
    /// Xbox User ID（启动参数 --xuid）
    pub xuid: String,
    /// access_token 过期时间（unix 秒）
    #[ts(type = "number")]
    pub expires_at: u64,
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    MsAccount::export().unwrap();
    ExternalAccount::export().unwrap();
    ExternalProfile::export().unwrap();
    ExternalServer::export().unwrap();
    ExternalConfig::export().unwrap();
}
