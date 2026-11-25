use std::env;
use std::path::PathBuf;

fn main() {
    build_cubiomes();
}

/// 编译 vendored xpple/cubiomes。
/// Windows MSVC 目标用 clang-cl（上游不支持 cl.exe 的 VLA）。
fn build_cubiomes() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vendor/cubiomes");
    let mut build = cc::Build::new();
    build.include(&root);

    if env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") && !env_tool_set("CC") {
        let clang = find_clang_cl().unwrap_or_else(|| {
            panic!(
                "种子预览需要 clang-cl 编译 cubiomes：请安装 LLVM 并确保 clang-cl.exe 可用，或设置 CC/AR"
            )
        });
        build.compiler(&clang);

        if !env_tool_set("AR") {
            if let Some(ar) = find_llvm_lib() {
                build.archiver(ar);
            }
        }
    }

    for file in [
        "biomenoise.c",
        "biomes.c",
        "layers.c",
        "generator.c",
        "finders.c",
        "util.c",
        "noise.c",
        "terrainnoise.c",
        "quadbase.c",
        "xradv.c",
        "carver.c",
        "features/stronghold.c",
        "features/end_city.c",
        "features/fortress.c",
        "features/abandoned_camp.c",
        "features/ore.c",
        "elytra_shim.c",
    ] {
        build.file(root.join(file));
    }
    build.flag_if_supported("-fwrapv");
    build.warnings(false);
    build.compile("cubiomes_elytra");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        println!("cargo:rustc-link-lib=m");
    }
    println!("cargo:rerun-if-changed={}", root.display());
}

/// 是否由用户显式指定了工具：兼容 cc 的 `<var>_<target>`、`<var>_<target_underscored>`、`TARGET_<var>`、`<var>`。
fn env_tool_set(name: &str) -> bool {
    if env::var_os(name).is_some() || env::var_os(format!("TARGET_{name}")).is_some() {
        return true;
    }
    let Ok(target) = env::var("TARGET") else {
        return false;
    };
    let underscored = target.replace(['-', '.'], "_");
    env::var_os(format!("{name}_{target}")).is_some()
        || env::var_os(format!("{name}_{underscored}")).is_some()
}

fn llvm_bin_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    for var in ["LIBCLANG_PATH", "LLVM_HOME", "LLVM_PATH"] {
        if let Some(v) = env::var_os(var) {
            let p = PathBuf::from(v);
            dirs.push(p.clone());
            dirs.push(p.join("bin"));
        }
    }

    #[cfg(windows)]
    {
        for var in ["ProgramFiles", "ProgramFiles(x86)", "ProgramW6432"] {
            if let Some(v) = env::var_os(var) {
                dirs.push(PathBuf::from(v).join("LLVM").join("bin"));
            }
        }
        if let Some(v) = env::var_os("LOCALAPPDATA") {
            dirs.push(PathBuf::from(v).join("Programs").join("LLVM").join("bin"));
        }
        dirs.push(PathBuf::from(r"C:\Program Files\LLVM\bin"));
    }

    #[cfg(not(windows))]
    {
        dirs.extend(
            [
                "/usr/bin",
                "/usr/local/bin",
                "/opt/homebrew/opt/llvm/bin",
                "/usr/local/opt/llvm/bin",
            ]
            .into_iter()
            .map(PathBuf::from),
        );
    }

    dirs
}

fn find_tool(names: &[&str], dirs: &[PathBuf]) -> Option<PathBuf> {
    let mut candidates = Vec::new();
    for dir in dirs {
        for name in names {
            candidates.push(dir.join(name));
        }
    }

    if let Some(paths) = env::var_os("PATH") {
        for dir in env::split_paths(&paths) {
            for name in names {
                candidates.push(dir.join(name));
            }
        }
    }

    candidates.into_iter().find(|p| p.is_file())
}

fn find_clang_cl() -> Option<PathBuf> {
    let names: &[&str] = if cfg!(windows) {
        &["clang-cl.exe", "clang-cl"]
    } else {
        &["clang-cl"]
    };
    find_tool(names, &llvm_bin_dirs())
}

fn find_llvm_lib() -> Option<PathBuf> {
    let names: &[&str] = if cfg!(windows) {
        &["llvm-lib.exe", "llvm-lib"]
    } else {
        &["llvm-lib"]
    };
    find_tool(names, &llvm_bin_dirs())
}
