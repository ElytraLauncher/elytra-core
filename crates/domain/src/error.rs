//! 结构化错误：`@i18n` key + 参数 + 中文兜底 + 技术细节，供 GUI/CLI/TUI 共用。
//!
//! `Display` 必须保持既有协议（首行 `@i18n:<key>?k=v`、次行兜底、其后细节），
//! Tauri 命令把它序列化成字符串后前端无需改动。

use std::fmt;

/// 粗分类；精确匹配用 [`CoreError::key`]。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Cancelled,
    Network,
    Http,
    Io,
    Parse,
    Config,
    Auth,
    Version,
    Loader,
    Modpack,
    Jdk,
    Task,
    Online,
    Unsupported,
    Other,
}

/// 领域错误。错误路径允许分配；不要把它放进热循环。
#[derive(Debug, Clone)]
pub struct CoreError {
    kind: ErrorKind,
    key: Option<String>,
    params: Vec<(String, String)>,
    fallback: String,
    detail: Option<String>,
}

impl CoreError {
    pub fn new(fallback: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Other,
            key: None,
            params: Vec::new(),
            fallback: fallback.into(),
            detail: None,
        }
    }

    pub fn with_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn keyed(key: impl Into<String>, fallback: impl Into<String>) -> Self {
        let key = key.into();
        let kind = kind_from_key(&key);
        Self {
            kind,
            key: Some(key),
            params: Vec::new(),
            fallback: fallback.into(),
            detail: None,
        }
    }

    /// 从完整 `@i18n` 协议文本还原（事件 payload / 旧 String 边界）。
    pub fn from_legacy(text: &str) -> Self {
        let mut lines = text.split('\n');
        let first = lines.next().unwrap_or_default();
        let rest: Vec<&str> = lines.collect();
        if let Some(spec) = first.strip_prefix("@i18n:") {
            let (key, params) = parse_key_spec(spec);
            let fallback = rest.first().copied().unwrap_or(&key).trim().to_string();
            let detail = join_detail(&rest[1.min(rest.len())..]);
            let mut err = Self::keyed(key, fallback);
            err.params = params;
            err.detail = detail;
            return err;
        }
        let fallback = first.trim().to_string();
        let detail = join_detail(&rest);
        Self {
            kind: ErrorKind::Other,
            key: None,
            params: Vec::new(),
            fallback,
            detail,
        }
    }

    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn params(&self) -> &[(String, String)] {
        &self.params
    }

    pub fn fallback(&self) -> &str {
        &self.fallback
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// 兼容旧调用方对错误文本的包含判断（如识别 `HTTP 404`）。
    pub fn contains(&self, pattern: &str) -> bool {
        self.to_string().contains(pattern)
    }

    /// 兼容旧调用方的前缀判断。
    pub fn starts_with(&self, pattern: &str) -> bool {
        self.to_string().starts_with(pattern)
    }

    /// 兼容旧调用方的空错误判断。
    pub fn is_empty(&self) -> bool {
        self.to_string().is_empty()
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_params(mut self, params: Vec<(String, String)>) -> Self {
        self.params = params;
        self
    }

    pub fn context(
        key: impl Into<String>,
        fallback: impl Into<String>,
        source: impl fmt::Display,
    ) -> Self {
        Self::keyed(key, fallback).with_detail(source.to_string())
    }
}

fn kind_from_key(key: &str) -> ErrorKind {
    let seg = key
        .strip_prefix("errors.")
        .and_then(|s| s.split('.').next());
    match seg {
        Some("download" | "fetch" | "network" | "http") => ErrorKind::Network,
        Some("io") => ErrorKind::Io,
        Some("parse") => ErrorKind::Parse,
        Some("config") => ErrorKind::Config,
        Some("auth" | "login" | "account" | "skin") => ErrorKind::Auth,
        Some("version") => ErrorKind::Version,
        Some("loader") => ErrorKind::Loader,
        Some("modpack") => ErrorKind::Modpack,
        Some("jdk" | "java") => ErrorKind::Jdk,
        Some("task" | "launch") => ErrorKind::Task,
        Some("online") => ErrorKind::Online,
        Some("unsupported") => ErrorKind::Unsupported,
        _ => ErrorKind::Other,
    }
}

fn parse_key_spec(spec: &str) -> (String, Vec<(String, String)>) {
    let (key, query) = spec.split_once('?').unwrap_or((spec, ""));
    let params = query
        .split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|pair| {
            let (k, v) = pair.split_once('=')?;
            Some((k.to_string(), v.to_string()))
        })
        .collect();
    (key.to_string(), params)
}

fn join_detail(lines: &[&str]) -> Option<String> {
    let text = lines.join("\n").trim().to_string();
    (!text.is_empty()).then_some(text)
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(key) = &self.key {
            write!(f, "@i18n:{key}")?;
            if !self.params.is_empty() {
                let query = self
                    .params
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join("&");
                write!(f, "?{query}")?;
            }
            f.write_str("\n")?;
            f.write_str(&self.fallback)?;
        } else {
            f.write_str(&self.fallback)?;
        }
        if let Some(detail) = &self.detail {
            f.write_str("\n")?;
            f.write_str(detail)?;
        }
        Ok(())
    }
}

impl std::error::Error for CoreError {}

impl From<String> for CoreError {
    fn from(value: String) -> Self {
        Self::from_legacy(&value)
    }
}

impl From<&str> for CoreError {
    fn from(value: &str) -> Self {
        Self::from_legacy(value)
    }
}

impl From<CoreError> for String {
    fn from(value: CoreError) -> Self {
        value.to_string()
    }
}

impl serde::Serialize for CoreError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::CoreError;

    #[test]
    fn keyed_error_keeps_protocol() {
        let e = CoreError::keyed(
            "errors.jdk.too_old?need=17&cur=8",
            "该版本需要 Java 17（当前 8）",
        )
        .with_detail("detail");
        assert_eq!(
            e.to_string(),
            "@i18n:errors.jdk.too_old?need=17&cur=8\n该版本需要 Java 17（当前 8）\ndetail"
        );
        assert_eq!(e.kind(), super::ErrorKind::Jdk);
    }

    #[test]
    fn legacy_round_trip() {
        let text = "@i18n:errors.download.failed\n下载失败\nHTTP 404";
        let e = CoreError::from_legacy(text);
        assert_eq!(e.key(), Some("errors.download.failed"));
        assert_eq!(e.fallback(), "下载失败");
        assert_eq!(e.detail(), Some("HTTP 404"));
        assert_eq!(e.to_string(), text);
    }

    #[test]
    fn serializes_as_plain_string_for_tauri() {
        let e = CoreError::keyed("errors.task.cancelled", "任务已取消");
        assert_eq!(
            serde_json::to_string(&e).expect("serialize"),
            "\"@i18n:errors.task.cancelled\\n任务已取消\""
        );
    }
}
