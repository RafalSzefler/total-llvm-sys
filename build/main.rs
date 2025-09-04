#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::enum_variant_names, clippy::inline_always)]
use std::{
    io::Write as _,
    path::{Path, PathBuf},
    sync::LazyLock,
};

mod arch;
mod features;
mod linking;
mod os;

static ROOT_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()));

fn get_root_dir() -> &'static PathBuf {
    &ROOT_DIR
}

fn download_archive(url: &str, build_dir: &PathBuf, filename: &str) {
    if !build_dir.exists() {
        std::fs::create_dir_all(build_dir).unwrap();
    }

    let mut file = std::fs::File::create(build_dir.join(filename)).unwrap();
    let resp = reqwest::blocking::get(url).unwrap();
    let mut resp = resp.error_for_status().unwrap();
    std::io::copy(&mut resp, &mut file).unwrap();
    file.sync_all().unwrap();
    file.flush().unwrap();
}

fn unzip_archive(build_dir: &Path, archive_filename: &str) {
    let archive_path = build_dir.join(archive_filename);
    let mut zip = zip::ZipArchive::new(std::fs::File::open(&archive_path).unwrap()).unwrap();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let outpath = build_dir.join(file.name());
        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent()
                && !p.exists()
            {
                std::fs::create_dir_all(p).unwrap();
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    std::fs::remove_file(archive_path).unwrap();
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn set_permissions_on_bin(llvm_dir: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let bin_dir = llvm_dir.join("bin");
    let mode = std::fs::Permissions::from_mode(0o755);
    let mut bin_files = Vec::new();
    for entry in std::fs::read_dir(bin_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if !file_name.starts_with("llvm-") {
            continue;
        }
        bin_files.push(path);
    }

    for path in &bin_files {
        std::fs::set_permissions(path, mode.clone()).unwrap();
    }
}

#[cfg(target_os = "windows")]
fn set_permissions_on_bin(_llvm_dir: &Path) {
    // Windows doesn't need to set permissions on the binaries
}

fn main() {
    let version = features::get_current_llvm_feature();
    let os = os::get_current_os();
    let arch = arch::get_current_arch();
    let archive_url_template = std::env::var("ARCHIVE_URL_TEMPLATE").unwrap();
    println!("cargo:rerun-if-env-changed=ARCHIVE_URL_TEMPLATE");
    let archive_url = archive_url_template
        .replace("{llvmVersion}", version.as_str())
        .replace("{os}", os.as_str())
        .replace("{arch}", arch.as_str());
    let build_dir_name = format!(".build-{}-{}-{}", version.as_str(), os.as_str(), arch.as_str());
    let root_dir = get_root_dir();
    let build_dir = root_dir.join(build_dir_name);
    let llvm_dir = build_dir.join(format!("{}-{}-{}", version.as_str(), os.as_str(), arch.as_str()));
    if !llvm_dir.exists() {
        let archive_filename = format!("{}-{}-{}.zip", version.as_str(), os.as_str(), arch.as_str());
        download_archive(&archive_url, &build_dir, &archive_filename);
        unzip_archive(&build_dir, &archive_filename);
    }

    set_permissions_on_bin(&llvm_dir);

    assert!(llvm_dir.exists(), "LLVM directory not found: {}", llvm_dir.display());

    println!("cargo:rerun-if-changed={}", llvm_dir.display());

    linking::link_llvm(&llvm_dir);
}
