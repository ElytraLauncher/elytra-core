//! 种子地图相关契约（`seedmap/`）。

use serde::Serialize;
use ts_rs::TS;

/// 单个方块群系。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "seedmap/")]
pub struct SeedMapBlock {
    pub biome: String,
}

/// 结构坐标。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "seedmap/")]
pub struct SeedMapStructure {
    pub kind: String,
    pub x: i32,
    pub z: i32,
}

/// 史莱姆区块坐标。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "seedmap/")]
pub struct SeedMapSlimeChunk {
    pub x: i32,
    pub z: i32,
}

/// 可供种子地图选择的版本。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "seedmap/")]
pub struct SeedMapVersion {
    pub id: String,
    pub label: String,
}

/// 解析后的版本（实际版本 + 家族 + 是否精确匹配）。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export_to = "seedmap/")]
pub struct ResolvedVersion {
    pub version: String,
    pub family: String,
    pub exact: bool,
}

/// 按 `ts_export.rs` 的登记顺序导出本模块类型。
pub(crate) fn export() {
    SeedMapBlock::export().unwrap();
    SeedMapSlimeChunk::export().unwrap();
    SeedMapStructure::export().unwrap();
    SeedMapVersion::export().unwrap();
    ResolvedVersion::export().unwrap();
}
