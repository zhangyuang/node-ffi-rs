extern crate napi_build;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Build script for ffi-rs.
///
/// On Android targets libffi calls `__builtin___clear_cache` while emitting
/// closure trampolines. On aarch64 the clang driver lowers that builtin to an
/// *external* reference to the `__clear_cache` symbol, whose implementation
/// lives in compiler-rt (`libclang_rt.builtins.a`). The Android NDK links that
/// archive in by default, but only when the NDK clang is actually used as the
/// C compiler and linker for both libffi's C sources and the final link.
///
/// If the binary is instead built with a glibc cross toolchain (e.g.
/// `gcc-aarch64-linux-gnu`), `__clear_cache` is left as an *undefined dynamic*
/// symbol that glibc happens to resolve but bionic libc does not, so `dlopen`
/// of the resulting `.node` fails on real Android devices. See issue #138.
///
/// To make the Android build robust regardless of which toolchain is wired up
/// by the surrounding CI, we explicitly locate `libclang_rt.builtins.a` in the
/// NDK and ask cargo to link it statically. When the NDK clang is already the
/// linker this is a harmless no-op (the symbol is resolved twice, statically);
/// when a non-NDK linker is used, this is what makes the build correct.
fn main() {
  napi_build::setup();

  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
  let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
  if target_os != "android" {
    return;
  }

  if let Some(builtins) = locate_ndk_builtins(&target_arch) {
    let dir = builtins
      .parent()
      .map(|p| p.to_path_buf())
      .unwrap_or_else(|| PathBuf::from("."));
    println!("cargo:rustc-link-search=native={}", dir.display());
    // The `:` prefix makes rustc treat the name as an exact archive filename
    // (no `lib` prefix / `.a` suffix rewriting), which matches the NDK's
    // `libclang_rt.builtins-<arch>.a` naming.
    let stem = builtins
      .file_name()
      .and_then(|s| s.to_str())
      .unwrap_or("libclang_rt.builtins.a");
    let lib_name = stem
      .strip_prefix("lib")
      .unwrap_or(stem)
      .strip_suffix(".a")
      .unwrap_or(stem);
    println!("cargo:rustc-link-lib=static=:{}", lib_name);
    println!(
      "cargo:warning=[ffi-rs] android: linking compiler-rt builtins from {}",
      builtins.display()
    );
  } else {
    println!(
      "cargo:warning=[ffi-rs] android: libclang_rt.builtins.a not found; relying on the NDK clang driver to link it. If the resulting .node fails to dlopen with `undefined symbol: __clear_cache`, build with the Android NDK toolchain (ANDROID_NDK_LATEST_HOME) so libffi is compiled and linked with NDK clang."
    );
  }

  // napi-rs style workaround: rustc may ask the linker for libgcc, which no
  // longer exists as a real archive in recent NDKs (replaced by libunwind +
  // libclang_rt.builtins). Provide a tiny shim so the link still succeeds.
  provide_libgcc_shim();
}

/// Search a set of likely NDK locations for the per-architecture builtins
/// archive and return its path if found.
fn locate_ndk_builtins(target_arch: &str) -> Option<PathBuf> {
  let arch_name = arch_subdir_name(target_arch)?;
  let candidates = ndk_root_candidates();

  for ndk_root in candidates {
    let prebuilt = find_prebuilt_dir(&ndk_root);
    let prebuilt = match prebuilt {
      Some(p) => p,
      None => continue,
    };
    // libclang_rt.builtins-<arch>.a lives under
    //   <prebuilt>/lib/clang/<clang-version>/lib/linux/
    // (and sometimes lib64/... on x86_64 hosts). The clang version folder
    // varies across NDK releases, so we glob over any version present.
    let clang_lib_dirs = [
      prebuilt.join("lib").join("clang"),
      prebuilt.join("lib64").join("clang"),
    ];
    for clang_dir in clang_lib_dirs {
      if let Ok(entries) = std::fs::read_dir(&clang_dir) {
        for entry in entries.flatten() {
          let version_dir = entry.path();
          // The platform-specific archives live in lib/linux/.
          let archives_dir = version_dir.join("lib").join("linux");
          // Prefer the architecture-specific archive; fall back to the
          // generic one.
          for name in [
            format!("libclang_rt.builtins-{}.a", arch_name),
            "libclang_rt.builtins.a".to_string(),
          ] {
            let candidate = archives_dir.join(&name);
            if candidate.is_file() {
              return Some(candidate);
            }
          }
        }
      }
    }
  }

  // Last resort: ask the C compiler (if it is the NDK clang) where its
  // resource directory lives via `clang --print-libgcc-file-name` /
  // `clang --print-file-name`.
  if let Some(found) = probe_via_cc(&arch_name) {
    return Some(found);
  }

  None
}

/// Map a Rust target architecture to the NDK's archive suffix token.
fn arch_subdir_name(target_arch: &str) -> Option<String> {
  Some(match target_arch {
    "aarch64" => "aarch64".to_string(),
    "arm" => "arm".to_string(),
    "x86_64" => "x86_64".to_string(),
    "x86" => "i386".to_string(),
    other => return Some(other.to_string()),
  })
}

/// Collect candidate NDK root directories from environment variables and the
/// directory of the configured C compiler.
fn ndk_root_candidates() -> Vec<PathBuf> {
  let mut roots = Vec::new();
  for var in [
    "ANDROID_NDK_LATEST_HOME",
    "ANDROID_NDK_HOME",
    "ANDROID_NDK_ROOT",
    "ANDROID_NDK",
    "NDK_TOOLCHAIN",
  ] {
    if let Ok(v) = env::var(var) {
      if !v.is_empty() {
        roots.push(PathBuf::from(v));
      }
    }
  }
  // Derive the NDK root from the configured CC when possible: NDK clang sits at
  //   <ndk>/toolchains/llvm/prebuilt/<host>-x86_64/bin/<target>-clang
  if let Ok(cc) = env::var("CC").or_else(|_| env::var("TARGET_CC")) {
    if !cc.is_empty() {
      if let Some(ndk_root) = ndk_root_from_clang_path(&PathBuf::from(&cc)) {
        roots.push(ndk_root);
      }
    }
  }
  roots
}

/// Given a path like `<ndk>/toolchains/llvm/prebuilt/<host>/bin/aarch64-linux-android24-clang`,
/// walk up to the `<ndk>` root.
fn ndk_root_from_clang_path(clang: &Path) -> Option<PathBuf> {
  let mut p = clang.parent()?; // .../bin
  for _ in 0..6 {
    if p.join("toolchains").is_dir() && p.join("source.properties").exists() {
      return Some(p.to_path_buf());
    }
    p = match p.parent() {
      Some(parent) => parent,
      None => return None,
    };
  }
  None
}

/// Inside an NDK root, find the `toolchains/llvm/prebuilt/<host>` directory.
fn find_prebuilt_dir(ndk_root: &Path) -> Option<PathBuf> {
  let prebuilt_root = ndk_root.join("toolchains").join("llvm").join("prebuilt");
  if let Ok(entries) = std::fs::read_dir(&prebuilt_root) {
    for entry in entries.flatten() {
      let p = entry.path();
      if p.is_dir() && p.join("bin").is_dir() {
        return Some(p);
      }
    }
  }
  None
}

/// Ask the configured C compiler to report its builtins archive directly.
fn probe_via_cc(arch_name: &str) -> Option<PathBuf> {
  let cc = env::var("CC").or_else(|_| env::var("TARGET_CC")).ok()?;
  if cc.is_empty() {
    return None;
  }
  // `clang --print-file-name=libclang_rt.builtins-<arch>.a` returns the resolved
  // path (or just the bare name if not found, which we then reject).
  let query = format!("libclang_rt.builtins-{}.a", arch_name);
  let output = Command::new(&cc)
    .args(["--print-file-name", &query])
    .output()
    .ok()?;
  let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
  if path_str.is_empty() || path_str == query {
    return None;
  }
  let p = PathBuf::from(path_str);
  if p.is_file() {
    Some(p)
  } else {
    None
  }
}

/// Write a `libgcc.a` containing `INPUT(-lunwind)` into OUT_DIR and add that
/// directory to the linker search path. Recent NDKs ship libunwind instead of
/// libgcc; rustc still asks the linker for `-lgcc`, so this shim keeps the link
/// working without pulling in the (now empty) NDK libgcc.
fn provide_libgcc_shim() {
  let out_dir = match env::var("OUT_DIR") {
    Ok(d) => d,
    Err(_) => return,
  };
  let dist = Path::new(&out_dir).join("libgcc.a");
  if let Ok(existing) = std::fs::read_to_string(&dist) {
    // Already populated (e.g. by a re-run); keep it as-is.
    if !existing.is_empty() {
      println!("cargo:rustc-link-search={}", out_dir);
      return;
    }
  }
  if std::fs::write(&dist, b"INPUT(-lunwind)\n").is_ok() {
    println!("cargo:rustc-link-search={}", out_dir);
  }
}
