//! 前端 TypeScript 契约类型（DTO）镜像。
//!
//! 本 crate 是唯一允许依赖 `ts-rs` 的契约层：字段、serde 属性与 `#[ts(...)]`
//! 均与业务 crate 的源类型一一对应，但嵌套字段使用本 crate 的镜像类型，
//! 保证生成的 TS 类型图不跨越 crate 边界。
//!
//! 导出入口：[`export_all`]（输出目录由 `TS_RS_EXPORT_DIR` 控制）。

pub mod auth;
pub mod config;
pub mod game;
pub mod jdk;
pub mod launcher;
pub mod map;
pub mod seedmap;
pub mod server_status;
pub mod task;
pub mod tooldl;
pub mod update;

/// 导出全部登记类型到 `TS_RS_EXPORT_DIR`（见 `src-tauri/.cargo/config.toml`）。
pub fn export_all() {
    auth::export();
    config::export();
    game::export();
    jdk::export();
    launcher::export();
    map::export();
    seedmap::export();
    server_status::export();
    task::export();
    tooldl::export();
    update::export();
}
