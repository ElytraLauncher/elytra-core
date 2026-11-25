/// 离线用户名键
pub const LAST_OFFLINE_USERNAME: &str = "last_offline_username";
/// 启动器默认公开名称
pub const APP_NAME: &str = "Elytra Launcher";
/// 默认启动器品牌名（-Dminecraft.launcher.brand）
pub const LAUNCHER_BRAND: &str = "Elytra";
/// 默认 Minecraft 客户端 ID（--clientId）
pub const CLIENT_ID: &str = "elytra";
/// 默认微软 OAuth 公共客户端 ID（公开信息，不是密钥；回调固定 `http://localhost:27205`）。
pub const MS_CLIENT_ID: &str = "eef6b0c0-8cbb-4091-84d0-b59fa0600dba";
/// 默认配置目录名
pub const CONFIG_DIR: &str = "Elytra";
/// 配置文件名
pub const SETTINGS_FILE: &str = "Settings.toml";
/// 默认系统钥匙串服务标识（macOS Keychain / Linux Secret Service 共用）
pub const KEYCHAIN_SERVICE: &str = "ElytraLauncher";
/// 主密钥条目 account（macOS）/ attribute（Linux）
pub const KEYCHAIN_MASTER_KEY_ACCOUNT: &str = "master-key";
/// 主密钥条目 label（用户可见）
pub const KEYCHAIN_MASTER_KEY_LABEL: &str = "Elytra Launcher master key";

/// 镜像源（BMCLAPI 等）并发上限：16。实测（2026-10）跨文件 32 并发全部 302 无 429、
/// 总耗时随并发线性下降；429 风险集中在**同文件**重复 Range 并发（节点侧按 hash 签名限流），
/// 跨文件高并发安全。单文件分段 ≤4 段，16 ≥ 4×4 文件在飞同文件不会触及节点限流。
/// 前端 Settings.vue 的 MIRROR_CONCURRENCY 需与此同步。
pub const MIRROR_CONCURRENCY: usize = 16;

/// 当前宿主的启动器版本。
pub fn app_version() -> &'static str {
    &crate::host::config().identity.app_version
}

/// 当前宿主的启动器名称。
pub fn app_name() -> &'static str {
    &crate::host::config().identity.app_name
}

/// 当前宿主的启动器品牌名。
pub fn launcher_brand() -> &'static str {
    &crate::host::config().identity.launcher_brand
}

/// 当前宿主的微软 OAuth client id。
pub fn client_id() -> &'static str {
    &crate::host::config().identity.client_id
}

/// 当前宿主的配置目录名。
pub fn config_dir_name() -> &'static str {
    &crate::host::config().identity.config_dir_name
}

/// 当前宿主的钥匙串服务标识。
pub fn keychain_service() -> &'static str {
    &crate::host::config().identity.keychain_service
}

/// 当前宿主的构建期 CurseForge key（用户 key 优先）。
pub fn curseforge_key() -> Option<&'static str> {
    crate::host::config().identity.curseforge_key.as_deref()
}

/// 下载 HTTP 客户端 User-Agent。
pub fn ua_download() -> &'static str {
    static UA: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    UA.get_or_init(|| format!("{}/{}", app_name(), app_version()))
}

/// Mojang API 调用 User-Agent。
pub fn ua_mojang_api() -> &'static str {
    static UA: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    UA.get_or_init(|| {
        format!(
            "{}/{} (+https://github.com/nefelibetas/ElytraLauncher)",
            app_name(),
            app_version()
        )
    })
}

/// 下载器配置
pub const SECT_DOWNLOADER: &str = "downloader";
/// 启动器配置
pub const SECT_LAUNCHER: &str = "launcher";
/// JDK 配置
pub const SECT_JDK: &str = "jdk";
/// JDK 扫描缓存 section（本机发现的 JDK 列表，后端维护）
pub const SECT_JDK_SCANNED: &str = "jdk_scanned";
/// 微软账号列表（secrets.bin 加密存储，按登录时间排序，末尾为当前）
pub const SECT_MS_ACCOUNTS: &str = "ms_accounts";
pub const SECT_EXTERNAL_ACCOUNTS: &str = "external_accounts";
pub const SECT_EXTERNAL: &str = "external";
/// 离线账号历史列表（最近使用排序，末尾为当前）
pub const SECT_OFFLINE_ACCOUNTS: &str = "offline_accounts";
/// 游戏目录
pub const SECT_GAME_DIRS: &str = "game_dirs";
/// 联机配置（协调节点等）
pub const SECT_ONLINE: &str = "online";
/// 本地皮肤收藏索引（PNG 原文存配置目录 `skins/<id>.png`）
pub const SECT_LOCAL_SKINS: &str = "local_skins";
/// 工具箱多线程下载器（保存目录 / 并发文件数 / 分段数）
pub const SECT_TOOL_DL: &str = "tooldl";
/// 启动器更新下载（默认下载目录）
pub const SECT_LAUNCHER_UPDATE: &str = "launcher_update";
