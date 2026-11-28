//! 存档玩家最后位置：兼容 26.x `players/data`、旧版 `playerdata` 与 `level.dat Data.Player`。
//!
//! 玩家 NBT 不含显示名；名称从世界根目录向上查找的 `usercache.json` 补齐，缺失时用 UUID 前缀。

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use fastnbt::Value;
use serde::Serialize;

use super::world_info::read_nbt;

/// 玩家列表上限；地图浮动标记还会再限制到 5 个。
const MAX_PLAYERS: usize = 200;
/// 26.x 单人玩家数据在 `players/data`，旧版在 `playerdata`。
const PLAYER_DIRS: [&str; 2] = ["players/data", "playerdata"];

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MapPlayer {
    pub uuid: String,
    pub name: String,
    pub x: f64,
    pub z: f64,
    /// 与 `region_io::list_dimensions` 一致的 UI 维度 id。
    pub dimension: String,
    /// 最后保存时间（Unix 毫秒）；无文件时间戳时为 None。
    pub last_played: Option<u64>,
}

/// 读取存档内所有玩家最后位置；单人老存档回退 `level.dat Data.Player`。
pub fn load_players(world_root: &Path) -> Vec<MapPlayer> {
    let names = load_usercache(world_root);
    let mut seen = HashSet::new();
    let mut players = Vec::new();

    for relative in PLAYER_DIRS {
        let dir = world_root.join(relative);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let is_dat = path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("dat"));
            if !is_dat {
                continue;
            }
            let Some(uuid) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            let key = normalize_uuid(uuid);
            if key.is_empty() || !seen.insert(key.clone()) {
                continue;
            }
            let Some((x, z, dimension)) = read_player_position(&path) else {
                continue;
            };
            players.push(MapPlayer {
                uuid: key,
                name: names
                    .get(&normalize_uuid(uuid))
                    .cloned()
                    .unwrap_or_else(|| short_uuid(uuid)),
                x,
                z,
                dimension,
                last_played: entry
                    .metadata()
                    .ok()
                    .and_then(|metadata| metadata.modified().ok())
                    .and_then(unix_millis),
            });
            if players.len() >= MAX_PLAYERS {
                break;
            }
        }
    }

    // 老式单人存档把玩家写在 level.dat；若没有同名 playerdata 就补一条。
    if let Some(player) = read_level_player(world_root, &names, &mut seen) {
        players.push(player);
    }

    players.sort_by_key(|player| std::cmp::Reverse(player.last_played));
    players
}

fn read_player_position(path: &Path) -> Option<(f64, f64, String)> {
    let root = read_nbt(path)?;
    if let Some(player) = root.get("Player").or_else(|| root.get("Data")) {
        player_position(player)
    } else {
        player_position(&Value::Compound(root))
    }
}

fn read_level_player(
    world_root: &Path,
    names: &HashMap<String, String>,
    seen: &mut HashSet<String>,
) -> Option<MapPlayer> {
    let root = read_nbt(&world_root.join("level.dat"))?;
    let data = root.get("Data").or_else(|| root.get("data"))?;
    let Value::Compound(data) = data else {
        return None;
    };
    let player = data.get("Player")?;
    let (x, z, dimension) = player_position(player)?;
    let uuid = data
        .get("singleplayer_uuid")
        .and_then(uuid_from_int_array)
        .unwrap_or_else(|| "singleplayer".to_string());
    if !seen.insert(normalize_uuid(&uuid)) {
        return None;
    }
    let name = names
        .get(&normalize_uuid(&uuid))
        .cloned()
        .unwrap_or_else(|| "Singleplayer".to_string());
    Some(MapPlayer {
        uuid: normalize_uuid(&uuid),
        name,
        x,
        z,
        dimension,
        last_played: std::fs::metadata(world_root.join("level.dat"))
            .ok()
            .and_then(|metadata| metadata.modified().ok())
            .and_then(unix_millis),
    })
}

fn player_position(value: &Value) -> Option<(f64, f64, String)> {
    let Value::Compound(map) = value else {
        return None;
    };
    let (x, z) = match map.get("Pos") {
        Some(Value::List(list)) if list.len() >= 3 => (number(&list[0])?, number(&list[2])?),
        _ => return None,
    };
    Some((x, z, dimension_id(map.get("Dimension"))))
}

fn number(value: &Value) -> Option<f64> {
    match value {
        Value::Double(value) => Some(*value),
        Value::Float(value) => Some(f64::from(*value)),
        Value::Int(value) => Some(f64::from(*value)),
        Value::Long(value) => Some(*value as f64),
        _ => None,
    }
}

fn dimension_id(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(value)) => map_dimension(value),
        Some(Value::Int(value)) => match value {
            -1 => "DIM-1".to_string(),
            1 => "DIM1".to_string(),
            _ => String::new(),
        },
        _ => String::new(),
    }
}

fn map_dimension(value: &str) -> String {
    match value {
        "" | "minecraft:overworld" => String::new(),
        "minecraft:the_nether" | "DIM-1" => "DIM-1".to_string(),
        "minecraft:the_end" | "DIM1" => "DIM1".to_string(),
        value if value.contains(':') => {
            let (namespace, path) = value.split_once(':').unwrap_or_default();
            format!("dimensions/{namespace}/{path}")
        }
        value => value.to_string(),
    }
}

fn load_usercache(world_root: &Path) -> HashMap<String, String> {
    let mut names = HashMap::new();
    let mut current = Some(world_root);
    for _ in 0..4 {
        let Some(dir) = current else {
            break;
        };
        let path = dir.join("usercache.json");
        if let Ok(bytes) = std::fs::read(&path) {
            if let Ok(entries) = serde_json::from_slice::<Vec<serde_json::Value>>(&bytes) {
                for entry in entries {
                    let Some(uuid) = entry.get("uuid").and_then(|value| value.as_str()) else {
                        continue;
                    };
                    let Some(name) = entry.get("name").and_then(|value| value.as_str()) else {
                        continue;
                    };
                    names
                        .entry(normalize_uuid(uuid))
                        .or_insert_with(|| name.to_string());
                }
                if !names.is_empty() {
                    break;
                }
            }
        }
        current = dir.parent();
    }
    names
}

fn uuid_from_int_array(value: &Value) -> Option<String> {
    let Value::IntArray(array) = value else {
        return None;
    };
    let ints = array.clone().into_inner();
    if ints.len() != 4 {
        return None;
    }
    let hex: String = ints
        .iter()
        .map(|value| format!("{:08x}", *value as u32))
        .collect();
    Some(format!(
        "{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    ))
}

fn unix_millis(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis() as u64)
}

fn normalize_uuid(value: &str) -> String {
    value
        .chars()
        .filter(|ch| *ch != '-')
        .flat_map(char::to_lowercase)
        .collect()
}

fn short_uuid(value: &str) -> String {
    let normalized = normalize_uuid(value);
    normalized.chars().take(8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Write;

    fn write_gzip(path: &Path, value: &Value) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let raw = fastnbt::to_bytes(value).unwrap();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&raw).unwrap();
        std::fs::write(path, encoder.finish().unwrap()).unwrap();
    }

    #[test]
    fn reads_modern_player_and_usercache_name() {
        let dir = tempfile::tempdir().unwrap();
        let uuid = "775e21d9-3732-4469-b3ba-c4b1188efd00";
        let mut player = HashMap::new();
        player.insert(
            "Pos".to_string(),
            Value::List(vec![
                Value::Double(12.5),
                Value::Double(70.0),
                Value::Double(-34.25),
            ]),
        );
        player.insert(
            "Dimension".to_string(),
            Value::String("minecraft:overworld".to_string()),
        );
        write_gzip(
            &dir.path().join(format!("players/data/{uuid}.dat")),
            &Value::Compound(player),
        );
        std::fs::write(
            dir.path().join("usercache.json"),
            format!(r#"[{{"name":"Alice","uuid":"{uuid}"}}]"#),
        )
        .unwrap();

        let players = load_players(dir.path());
        assert_eq!(players.len(), 1);
        let player = &players[0];
        assert_eq!(player.name, "Alice");
        assert_eq!(player.uuid, normalize_uuid(uuid));
        assert_eq!(player.dimension, "");
        assert!((player.x - 12.5).abs() < f64::EPSILON);
        assert!((player.z + 34.25).abs() < f64::EPSILON);
    }

    #[test]
    fn maps_custom_dimensions() {
        assert_eq!(map_dimension("minecraft:the_nether"), "DIM-1");
        assert_eq!(map_dimension("minecraft:the_end"), "DIM1");
        assert_eq!(
            map_dimension("twilightforest:twilight_forest"),
            "dimensions/twilightforest/twilight_forest"
        );
    }
}
