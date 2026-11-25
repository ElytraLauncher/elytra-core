# Elytra Core

Elytra Launcher 的非 Tauri 核心：下载器、版本装配、ModLoader、任务编排、账号、JDK、持久化、联机、地图与种子预览。

## Workspace

| crate | 职责 |
|---|---|
| `elytra-domain` | 领域与基础设施（无 Tauri / easytier / cubiomes / mca） |
| `elytra-map` | 存档地图 region 读取与瓦片渲染 |
| `elytra-seedmap` | cubiomes 种子预览 |
| `elytra-online` | EasyTier 联机 |
| `elytra-dto` | 前端契约镜像（唯一 ts-rs 消费者，非默认成员） |

## 宿主接入

```rust
// 安装带时间戳的 logger，并把 stdout/stderr 落到配置目录 launcher.log
elytra_domain::logging::init();
elytra_domain::host::init(elytra_domain::host::Config {
    identity: elytra_domain::host::Identity {
        app_version: "1.7.4".into(),
        curseforge_key: None,
        ..Default::default()
    },
    ..Default::default()
})?;
elytra_domain::util::set_handle(tokio::runtime::Handle::current());
```

事件通过 `elytra_domain::task::EventSink` 注入；CLI/TUI 可直接同步实现，Tauri 使用 `DispatchSink`。
错误是 `elytra_domain::error::CoreError`，`Display` 保持 `@i18n:<key>?k=v` 协议。

## 构建

- 完整 workspace：Rust 1.99 + protoc + 7z + libclang；Windows seedmap 另需 clang-cl。
- 仅 core：`cargo build -p elytra-domain`，不需要上述系统工具。
- 测试：`cargo test --workspace -- --test-threads=1`。
- 前端类型：在 Elytra app 的 `src-tauri` 下 `cargo run --bin gen_ts --features gen-ts`，由 `elytra-dto` 导出。

一个进程只允许一次 `host::init`；`TaskManager` / 联机会话 / 工具箱注册表是进程级单例。
