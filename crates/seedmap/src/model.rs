use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SeedMapBlock {
    pub biome: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SeedMapStructure {
    pub kind: String,
    pub x: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SeedMapSlimeChunk {
    pub x: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SeedMapVersion {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedVersion {
    pub version: String,
    pub family: String,
    pub exact: bool,
}
