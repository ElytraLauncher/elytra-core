//! 地图相关契约（`map/`）。

use serde::Serialize;
use ts_rs::TS;

/// 世界概要（种子 / 高度范围 / 维度列表）。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "map/")]
pub struct MapWorldInfo {
    /// 十进制种子；读取失败为 None。
    pub seed: Option<String>,
    pub data_version: Option<i32>,
    pub min_y: i32,
    pub max_y: i32,
    pub dimensions: Vec<String>,
}

/// region 概要（前端视口定位 + 变更检测）。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "map/")]
pub struct MapRegionInfo {
    pub x: i32,
    pub z: i32,
    /// region 文件最后修改（unix 秒）。
    #[ts(type = "number")]
    pub mtime: u64,
}

/// 递归发现的世界根目录。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "map/")]
pub struct MapFolderEntry {
    pub root: String,
    pub name: String,
}

/// 地图方块信息。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "map/")]
pub struct MapBlockInfo {
    pub biome: Option<String>,
}

/// 单个群系命中坐标。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "map/")]
pub struct MapBiomeHit {
    pub biome: String,
    pub x: i32,
    pub z: i32,
}

/// 群系统计。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "map/")]
pub struct MapBiomeSummary {
    pub biome: String,
    pub count: u32,
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    MapWorldInfo::export().unwrap();
    MapRegionInfo::export().unwrap();
    MapFolderEntry::export().unwrap();
    MapBlockInfo::export().unwrap();
    MapBiomeHit::export().unwrap();
    MapBiomeSummary::export().unwrap();
}
