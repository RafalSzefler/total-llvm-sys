#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]

use std::{
    io::{Read as _, Write as _}, path::{Path, PathBuf},
};

mod arch;
mod features;
mod linking;
mod os;

pub fn main() {
    let version = features::get_current_llvm_feature();
    let os = os::OS::current();
    let arch = arch::Arch::current();
    let archive_url_template = std::env::var("TOTAL_LLVM_SYS_ARCHIVE_URL_TEMPLATE")
        .expect("TOTAL_LLVM_SYS_ARCHIVE_URL_TEMPLATE is not set.");

    let archive_url_ref = archive_url_template.trim();
    if archive_url_ref.is_empty() {
        panic!("TOTAL_LLVM_SYS_ARCHIVE_URL_TEMPLATE is empty.");
    }

    println!("cargo:rerun-if-env-changed=TOTAL_LLVM_SYS_ARCHIVE_URL_TEMPLATE");

    let archive_url = archive_url_ref
        .replace("{llvmVersion}", version.as_str())
        .replace("{os}", os.as_str())
        .replace("{arch}", arch.as_str());

    let build_dir = get_build_dir().join("total-llvm-sys");
    if !build_dir.exists() {
        let build_dir_str = build_dir.display().to_string();
        std::fs::create_dir_all(&build_dir).expect(&format!("Failed to create [{build_dir_str}] build directory"));
    }

    let llvm_dir = build_dir.join(format!("{}-{}-{}", version.as_str(), os.as_str(), arch.as_str()));
    let llvm_dir_str = llvm_dir.display().to_string();
    if llvm_dir.exists() {
        std::fs::remove_dir_all(&llvm_dir).expect(&format!("Failed to clear [{llvm_dir_str}] LLVM build directory."));
        std::fs::create_dir_all(&llvm_dir).expect(&format!("Failed to create [{llvm_dir_str}] LLVM build directory."));
    }

    let filename = format!("{}-{}-{}.zip", version.as_str(), os.as_str(), arch.as_str());
    download_archive(&archive_url, &build_dir, &filename);
    unzip_archive(&build_dir, &filename);
    set_permissions_on_bin(&llvm_dir);

    println!("cargo:rerun-if-changed={llvm_dir_str}");
    println!("cargo:warning=LLVM downloaded and stored in [{llvm_dir_str}]. Linking to it.");
    linking::link_llvm(&llvm_dir);
}


fn get_tmp_build_dir() -> PathBuf {
    std::env::temp_dir()
}

fn get_build_dir() -> PathBuf {
    let Ok(build_dir) = std::env::var("TOTAL_LLVM_SYS_BUILD_DIR") else {
        return get_tmp_build_dir();
    };
    let build_dir = build_dir.trim();
    if build_dir.is_empty() {
        return get_tmp_build_dir();
    }
    let build_dir = PathBuf::from(build_dir);
    let build_dir_str = build_dir.display();
    assert!(!build_dir.exists() || build_dir.is_dir(), "TOTAL_LLVM_SYS_BUILD_DIR is not a directory: {build_dir_str}");
    build_dir
}

fn download_archive(url: &str, build_dir: &PathBuf, filename: &str) {
    if !build_dir.exists() {
        std::fs::create_dir_all(build_dir).unwrap();
    }

    let archive_file = build_dir.join(filename);
    let mut file = std::fs::File::create(&archive_file).unwrap();
    let resp = reqwest::blocking::get(url).unwrap();
    let mut resp = match resp.error_for_status() {
        Ok(ok) => ok,
        Err(err) => {
            drop(file);
            let _ = std::fs::remove_file(archive_file);
            let status = err.status().unwrap_or_default();
            let url = err.url().unwrap();
            panic!("Could not download llvm build. Received [{status}] from [{url}].");
        }
    };

    let mut buffer = Vec::new();
    buffer.resize(1 << 25, 0u8);

    loop {
        let bytes_read = resp.read(&mut buffer)
            .expect("Failed to fetch LLVM archive from remote server.");

        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read])
            .expect("Failed to write LLVM archive to local filesystem.");
    }

    file.flush().expect("Failed to flush LLVM archive to local filesystem.");
    file.sync_all().expect("Failed to sync_all LLVM archive to local filesystem.");
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

fn set_permissions_on_bin(llvm_dir: &Path) {
    cfg_select!(
        any(target_os = "linux", target_os = "macos") => {
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
        },
        _ => {
            let _ = llvm_dir;
        }
    );
}

