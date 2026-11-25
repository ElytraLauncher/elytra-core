//! 子进程装配：应用运行期间**不得弹出控制台窗口**（GUI 进程无控制台附着，spawn 控制台程序
//! 时系统会为子进程新建窗口；装 Forge/NeoForge 时逐个 `java -cp …` 连弹多个）。
//!
//! **所有 spawn 点必须经 [`NoWindow`] 装配。**

/// Windows 进程创建标志：子进程不创建控制台窗口。
#[cfg(target_os = "windows")]
pub const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// 无窗口装配：Windows 合并 `CREATE_NO_WINDOW`，其它平台空实现。`creation_flags` 是**覆盖式**设置、一条命令只能设一次，
/// 需附加标志（`CREATE_BREAKAWAY_FROM_JOB` / `CREATE_NEW_PROCESS_GROUP`…）时走 `no_window_with`；
/// 再调一次会丢掉 `CREATE_NO_WINDOW`（窗口又冒出来）。
pub trait NoWindow {
    /// 隐藏控制台窗口并附加 `extra_flags`（Windows 有效，其它平台为空实现）。
    fn no_window_with(&mut self, extra_flags: u32) -> &mut Self;
    /// 仅隐藏控制台窗口。
    fn no_window(&mut self) -> &mut Self {
        self.no_window_with(0)
    }
}

#[cfg(target_os = "windows")]
impl NoWindow for std::process::Command {
    fn no_window_with(&mut self, extra_flags: u32) -> &mut Self {
        use std::os::windows::process::CommandExt as _;
        self.creation_flags(CREATE_NO_WINDOW | extra_flags);
        self
    }
}

#[cfg(not(target_os = "windows"))]
impl NoWindow for std::process::Command {
    fn no_window_with(&mut self, _extra_flags: u32) -> &mut Self {
        self
    }
}

#[cfg(target_os = "windows")]
impl NoWindow for tokio::process::Command {
    fn no_window_with(&mut self, extra_flags: u32) -> &mut Self {
        self.creation_flags(CREATE_NO_WINDOW | extra_flags);
        self
    }
}

#[cfg(not(target_os = "windows"))]
impl NoWindow for tokio::process::Command {
    fn no_window_with(&mut self, _extra_flags: u32) -> &mut Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 装配调用不得 panic，且链式用法可继续设置 stdio（两种命令类型都覆盖）。
    #[test]
    fn no_window_is_chainable() {
        let mut std_cmd = std::process::Command::new("java");
        std_cmd
            .no_window_with(0x0000_0200)
            .stdout(std::process::Stdio::null());
        assert_eq!(std_cmd.get_program().to_string_lossy(), "java");

        let mut tokio_cmd = tokio::process::Command::new("java");
        tokio_cmd.no_window().stdout(std::process::Stdio::null());
        assert_eq!(tokio_cmd.as_std().get_program().to_string_lossy(), "java");
    }
}
