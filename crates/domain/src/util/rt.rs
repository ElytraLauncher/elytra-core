//! 同步/异步运行时桥接：注入宿主 tokio Handle，无上下文时复用兜底 runtime。

use std::future::Future;
use std::sync::{LazyLock, OnceLock};

use tokio::runtime::{Handle, RuntimeFlavor};
pub use tokio::task::JoinHandle;

static HANDLE: OnceLock<Handle> = OnceLock::new();

/// 注入宿主 runtime（Tauri / CLI / TUI）。必须在任何 spawn 前调用一次。
pub fn set_handle(handle: Handle) {
    let _ = HANDLE.set(handle);
}

fn fallback_runtime() -> &'static tokio::runtime::Runtime {
    static RT: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(2)
            .build()
            .expect("tokio runtime")
    });
    &RT
}

/// 当前可用 handle：宿主注入 > 当前上下文 > 兜底 runtime。
pub fn handle() -> Handle {
    if let Some(h) = HANDLE.get() {
        return h.clone();
    }
    if let Ok(h) = Handle::try_current() {
        return h;
    }
    fallback_runtime().handle().clone()
}

/// 在宿主 runtime 上 spawn；可从任意线程调用。
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    handle().spawn(future)
}

/// 在宿主 runtime 的 blocking 池执行；可从任意线程调用。
pub fn spawn_blocking<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    handle().spawn_blocking(f)
}

/// 同步执行异步操作（低频阻塞可接受；secret-service 钥匙串 / task DB 复用）。有 tokio 上下文则复用现有 runtime：
/// multi_thread 下 worker 直接 `Handle::block_on` 会 panic（runtime 内不能再启动 runtime），须 `block_in_place`
/// 让出 worker（外部线程下 no-op）；current_thread 不可用则直接 `block_on`；无上下文时自维护兜底 runtime（2 worker）。
pub fn block_on<F: Future>(fut: F) -> F::Output {
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        if handle.runtime_flavor() == RuntimeFlavor::MultiThread {
            tokio::task::block_in_place(move || handle.block_on(fut))
        } else {
            handle.block_on(fut)
        }
    } else {
        fallback_runtime().block_on(fut)
    }
}
