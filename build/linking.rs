//! This module mostly follows what llvm-sys' build.rs does.
//! Except it is slightly simplified (static linking only).
use std::path::{Path, PathBuf};

pub fn link_llvm(llvm_dir: &Path) {
    let llvm_bin_dir = llvm_dir.join("bin");
    assert!(
        llvm_bin_dir.exists(),
        "bin directory not found: {}",
        llvm_bin_dir.display()
    );

    let llvm_config = locate_llvm_config(&llvm_bin_dir);
    println!("cargo:config_path={}", llvm_config.display());

    let call_llvm_config = |args: &[&str]| call_llvm_config(&llvm_config, args);

    let libdir = call_llvm_config(&["--libdir"]);
    println!("cargo:libdir={libdir}"); // DEP_LLVM_LIBDIR
    println!("cargo:rustc-link-search=native={libdir}");

    for link_search_dir in get_system_library_dirs() {
        println!("cargo:rustc-link-search=native={link_search_dir}");
    }

    let link_libs = get_link_libraries(&llvm_config);
    for lib in link_libs {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    for lib in get_system_libraries(&llvm_config) {
        println!("cargo:rustc-link-lib=dylib={lib}");
    }
}

fn locate_llvm_config(bin_dir: &Path) -> PathBuf {
    let potential_names = ["llvm-config", "llvm-config.exe", "llvm_config", "llvm_config.exe"];
    for name in potential_names {
        let path = bin_dir.join(name);
        if path.exists() {
            return path;
        }
    }
    panic!("llvm-config not found in {}", bin_dir.display());
}

fn call_llvm_config(llvm_config: &Path, args: &[&str]) -> String {
    let output = std::process::Command::new(llvm_config).args(args).output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

fn get_link_libraries(llvm_config: &Path) -> Vec<String> {
    let libs = call_llvm_config(llvm_config, &["--libnames", "--link-static"]);
    libs.split_whitespace().map(extract_lib_name).collect()
}

fn get_system_library_dirs() -> Vec<String> {
    let mut system_library_dirs = Vec::new();
    if target_os_is("openbsd") || target_os_is("freebsd") {
        system_library_dirs.push("/usr/local/lib".to_string());
    } else if target_os_is("macos") {
        if let Some(p) = homebrew_prefix(None) {
            system_library_dirs.push(format!("{p}/lib"));
        }
    } else if target_os_is("linux") && cfg!(target_feature = "crt-static") {
        // When linking statically on Linux, we need to provide the directory
        // with system-wide static libraries explicitly.
        #[cfg(any(target_arch = "x86_64", target_arch = "powerpc64", target_arch = "aarch64"))]
        {
            system_library_dirs.push("/lib64".to_string());
            system_library_dirs.push("/usr/lib64".to_string());
            system_library_dirs.push("/usr/local/lib64".to_string());
        }
        system_library_dirs.push("/lib".to_string());
        system_library_dirs.push("/usr/lib".to_string());
        system_library_dirs.push("/usr/local/lib".to_string());
    }

    system_library_dirs
}

fn get_system_libraries(llvm_config: &Path) -> Vec<String> {
    let libs = call_llvm_config(llvm_config, &["--system-libs"]);
    let mut system_libs: Vec<String> = libs
        .split_whitespace()
        .map(|flag| {
            if target_env_is("msvc") {
                return extract_lib_name(flag);
            }

            if let Some(flag) = flag.strip_prefix("-l") {
                // Linker flags style, -lfoo
                if target_os_is("macos") {
                    // .tdb libraries are "text-based stub" files that provide lists of symbols,
                    // which refer to libraries shipped with a given system and aren't shipped
                    // as part of the corresponding SDK. They're named like the underlying
                    // library object, including the 'lib' prefix that we need to strip.
                    if let Some(flag) = flag.strip_prefix("lib").and_then(|flag| flag.strip_suffix(".tbd")) {
                        return flag.to_string();
                    }
                }

                if let Some(i) = flag.find(".so.") {
                    // On some distributions (OpenBSD, perhaps others), we get sonames
                    // like "-lz.so.7.0". Correct those by pruning the file extension
                    // and library version.
                    return flag[..i].to_string();
                }
                return flag.to_string();
            }

            let maybe_lib = std::path::Path::new(flag);
            if maybe_lib.is_file() {
                // Library on disk, likely an absolute path to a .so. We'll add its location to
                // the library search path and specify the file as a link target.
                println!("cargo:rustc-link-search={}", maybe_lib.parent().unwrap().display());

                // Expect a file named something like libfoo.so, or with a version libfoo.so.1.
                // Trim everything after and including the last .so and remove the leading 'lib'
                let soname = maybe_lib
                    .file_name()
                    .unwrap()
                    .to_str()
                    .expect("Library filename must be a valid string");

                // Check for any valid library filename, even if it's a different kind from the
                // one we asked for. Some configurations give us a path to a static archive,
                // even when we're asking for shared libraries.
                let file_extension = if target_os_is("macos") { ".dylib" } else { ".so" };

                if let Some((stem, _rest)) = soname.rsplit_once(file_extension) {
                    return stem
                        .strip_prefix("lib")
                        .unwrap_or_else(|| panic!("system library '{soname}' does not have a 'lib' prefix"))
                        .to_string();
                }
            }

            panic!("Unable to parse result of llvm-config --system-libs: {flag}");
        })
        .collect();

    if let Some(libcpp) = get_system_libcpp() {
        system_libs.push(libcpp.to_string());
    }

    system_libs
}

fn target_env_is(name: &str) -> bool {
    match std::env::var_os("CARGO_CFG_TARGET_ENV") {
        Some(s) => s == name,
        None => false,
    }
}

fn target_os_is(name: &str) -> bool {
    match std::env::var_os("CARGO_CFG_TARGET_OS") {
        Some(s) => s == name,
        None => false,
    }
}

fn homebrew_prefix(name: Option<&str>) -> Option<String> {
    let mut cmd = std::process::Command::new("brew");
    cmd.arg("--prefix");

    if let Some(name) = name {
        cmd.arg(name);
    }

    cmd.output()
        .ok()
        .filter(|o| !o.stdout.is_empty())
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|val| val.trim().to_string())
}

fn extract_lib_name(lib: &str) -> String {
    let mut result: &str;
    if let Some(name) = lib.strip_prefix("lib") {
        result = name;
        if let Some(name) = result.strip_suffix(".a") {
            result = name;
        }
    } else if let Some(name) = lib.strip_suffix(".lib") {
        result = name;
    } else {
        panic!("'{lib}' does not look like a static library name");
    }
    result.to_string()
}

fn get_system_libcpp() -> Option<&'static str> {
    if let Some(libcpp) = option_env!("LLVM_SYS_LIBCPP") {
        // Use the library defined by the caller, if provided.
        Some(libcpp)
    } else if target_env_is("msvc") {
        // MSVC doesn't need an explicit one.
        None
    } else if target_os_is("macos") {
        // On OS X 10.9 and later, LLVM's libc++ is the default. On earlier
        // releases GCC's libstdc++ is default. Unfortunately we can't
        // reasonably detect which one we need (on older ones libc++ is
        // available and can be selected with -stdlib=lib++), so assume the
        // latest, at the cost of breaking the build on older OS releases
        // when LLVM was built against libstdc++.
        Some("c++")
    } else if target_os_is("freebsd") || target_os_is("openbsd") {
        Some("c++")
    } else {
        // Otherwise assume GCC's libstdc++.
        // This assumption is probably wrong on some platforms, but it can be
        // always overwritten through `LLVM_SYS_LIBCPP` variable.
        Some("stdc++")
    }
}
