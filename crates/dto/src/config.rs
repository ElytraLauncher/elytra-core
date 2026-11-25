//! 配置相关契约（`config/`）。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 文件下载源模板。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "config/")]
pub struct DownloadSource {
    pub name: String,

    pub manifest_url: String,
    pub resource_url: String,
    pub maven_url: String,
    pub libraries_url: String,

    pub forge_list_url: String,
    pub fabric_list_url: String,
    pub neoforge_list_url: String,
    pub quilt_list_url: String,
    pub optifine_list_url: String,

    pub forge_installer_url: String,
    pub neoforge_installer_url: String,
    pub fabric_loader_url: String,
    pub fabric_profile_url: String,
    pub quilt_profile_url: String,
    pub optifine_download_url: String,
}

/// 网络代理模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(export_to = "config/")]
#[serde(rename_all = "camelCase")]
pub enum NetworkMode {
    #[default]
    Auto,
    System,
    Direct,
}

/// 下载源选择模式。
///
/// 与源类型一致只派生 `Serialize`（源类型另有自定义 `Deserialize` 接受 camelCase 字符串，
/// DTO 暂不承载该反序列化逻辑）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default, TS)]
#[ts(export_to = "config/")]
#[serde(rename_all = "camelCase")]
pub enum DownloadSourceMode {
    /// 仅官方源
    OfficialOnly,
    /// 仅镜像源
    MirrorOnly,
    /// 自适应：程序自动判断更快源
    #[default]
    Auto,
}

/// 与源类型一致：自定义反序列化，仅接受 camelCase 字符串，未知值回退 `Auto`。
impl<'de> Deserialize<'de> for DownloadSourceMode {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        Ok(match s.as_str() {
            "officialOnly" => Self::OfficialOnly,
            "mirrorOnly" => Self::MirrorOnly,
            _ => Self::Auto,
        })
    }
}

/// 下载器配置。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "config/")]
pub struct DownloaderConfig {
    /// 文件源模板（代码内置、不持久化；DTO 侧该字段不参与 TS 输出）。
    #[serde(default = "default_sources", skip_serializing, skip_deserializing)]
    #[ts(skip)]
    pub sources: Vec<DownloadSource>,
    pub concurrency: u32,
    pub minecraft_dir: String,
    /// 全局限速（bytes/s）；0/None = 不限制。
    #[serde(default)]
    #[ts(type = "number")]
    pub max_speed: u64,
    /// 游戏下载源模式。
    #[serde(default)]
    pub game_source_mode: DownloadSourceMode,
    /// 社区下载源模式。
    #[serde(default)]
    pub community_source_mode: DownloadSourceMode,
    /// 网络代理模式。
    #[serde(default)]
    pub network_mode: NetworkMode,
    /// 仅前端展示使用（后端仅存储透传）。
    #[serde(default)]
    pub translate_display: String,
    /// 社区版本列表是否只显示正式版；仅前端使用，后端存储透传。
    #[serde(default = "default_show_release_only")]
    pub show_release_only: bool,
    /// JDK（managed runtime）下载源模式。
    #[serde(default)]
    pub jdk_source_mode: JdkSourceMode,
}

/// JDK（managed runtime）下载源模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default, TS)]
#[ts(export_to = "config/")]
#[serde(rename_all = "camelCase")]
pub enum JdkSourceMode {
    /// 仅官方源（Mojang runtime → Azul）
    OfficialOnly,
    /// 仅镜像（Adoptium TUNA 文件；元数据 API 例外走官方）
    MirrorOnly,
    /// 智能回退：官方 → Adoptium（TUNA→官方）→ Azul 全链
    #[default]
    Auto,
}

/// 与源类型一致：自定义反序列化，仅接受 camelCase 字符串，未知值回退 `Auto`。
impl<'de> Deserialize<'de> for JdkSourceMode {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        Ok(match s.as_str() {
            "officialOnly" => Self::OfficialOnly,
            "mirrorOnly" => Self::MirrorOnly,
            _ => Self::Auto,
        })
    }
}

/// 一个游戏目录（.minecraft 文件夹）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export_to = "config/")]
pub struct GameDir {
    /// 列表显示名称
    pub name: String,
    /// 完整路径
    pub path: String,
}

/// 游戏目录配置。
#[derive(Debug, Default, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "config/")]
pub struct GameDirsConfig {
    pub dirs: Vec<GameDir>,
}

/// 启动器更新配置（persist section `launcher_update`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export_to = "config/")]
#[serde(default)]
pub struct LauncherUpdateConfig {
    /// 下载目录（空 = 使用平台默认目录）
    pub download_dir: String,
}

/// 联机配置（persist section "online"）。
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export_to = "config/")]
pub struct OnlineConfig {
    /// 协调节点列表（easytier peer URI）。
    #[serde(default)]
    pub public_nodes: Vec<String>,
    /// 自定义联机广播名（guest 端 MC 局域网标签页显示名）。
    #[serde(default)]
    pub advertise_motd: Option<String>,
    /// 联机昵称（空 = 系统用户名）。
    #[serde(default)]
    pub nickname: Option<String>,
    /// 是否允许打洞失败时经公共节点中继。
    #[serde(default)]
    pub allow_relay: bool,
}

/// 工具箱下载器配置（persist section `tooldl`）。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export_to = "config/")]
#[serde(default)]
pub struct ToolDlConfig {
    /// 保存目录（空 = 尚未选择）
    pub save_dir: String,
    /// 同时下载的文件数上限
    pub max_files: u32,
    /// 单文件分段数（1 = 单连接）
    pub segments: u32,
}

/// 与源类型一致：默认 3 个并发文件、8 个分段。
impl Default for ToolDlConfig {
    fn default() -> Self {
        Self {
            save_dir: String::new(),
            max_files: 3,
            segments: 8,
        }
    }
}

/// 缺省下载源：源类型使用内置官方 + 镜像模板；DTO 不复制内置拓扑（该字段 `#[ts(skip)]`，前端不消费）。
fn default_sources() -> Vec<DownloadSource> {
    Vec::new()
}

fn default_show_release_only() -> bool {
    true
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    DownloaderConfig::export().unwrap();
    DownloadSource::export().unwrap();
    NetworkMode::export().unwrap();
    DownloadSourceMode::export().unwrap();
    JdkSourceMode::export().unwrap();
    GameDir::export().unwrap();
    GameDirsConfig::export().unwrap();
    OnlineConfig::export().unwrap();
    ToolDlConfig::export().unwrap();
    LauncherUpdateConfig::export().unwrap();
}
