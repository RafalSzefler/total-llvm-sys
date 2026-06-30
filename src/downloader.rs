use std::{collections::HashSet, io::Write, path::PathBuf, sync::{Arc, Mutex}};

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt as _;
use crate::config::Config;

pub async fn download_and_install_llvm(config: &Config) -> Result<(), anyhow::Error> {
    let mp = MultiProgress::new();
    mp.println("=== Download & Install LLVM ===")?;
    let archive_path = download_llvm(config, &mp).await?;
    let dst_folder = install_llvm(config, &archive_path, &mp).await?;
    set_permissions_on_bin(&dst_folder);
    mp.println(format!("LLVM [{}] successfully installed to [{}]", config.llvm_version, dst_folder.display())).unwrap();
    std::fs::remove_file(archive_path)?;
    mp.println("=== All tasks complete ===")?;
    mp.println("")?;
    mp.println("In order to use the installed LLVM, either add the following path to your $PATH environment variable, or (if you use llvm_sys) set the $LLVM_SYS_<version>_PREFIX environment variable:").unwrap();
    mp.println("")?;
    mp.println(format!("{}", dst_folder.display()))?;
    mp.println("")?;
    Ok(())
}

async fn download_llvm(config: &Config, mp: &MultiProgress) -> Result<PathBuf, anyhow::Error> {
    let url = config.url_template
        .replace("{llvmVersion}", &config.llvm_version.to_string())
        .replace("{os}", config.os.as_str())
        .replace("{arch}", config.arch.as_str());

    mp.println(format!(
        "Downloading LLVM [{}] from [{}]",
        config.llvm_version, url
    ))?;

    if !config.tmp_dir.exists() {
        panic!("Temporary directory does not exist: {}", config.tmp_dir.display());
    }

    let archive_path = config.tmp_dir.join(format!("llvm-{}-{}-{}.zip", config.llvm_version, config.os.as_str(), config.arch.as_str()));
    if archive_path.exists() {
        mp.println(format!("[WARNING] Archive already exists: [{}]. Reusing.", archive_path.display()))?;
        return Ok(archive_path);
    }

    let mut file = tokio::fs::File::create(&archive_path).await?;

    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to download LLVM: {}", response.status()));
    }

    let total_size = response
        .content_length()
        .unwrap_or(0);

    let pb = mp.add(make_download_bar(total_size, "Download"));

    let mut downloaded: u64 = 0;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
        file.write_all(&chunk).await?;
    }

    file.flush().await?;
    file.sync_all().await?;

    pb.finish_and_clear();

    Ok(archive_path)
}

const NUMBER_OF_WORKERS: usize = 11;

async fn install_llvm(config: &Config, archive_path: &PathBuf, mp: &MultiProgress) -> Result<PathBuf, anyhow::Error> {
    let base = format!("llvm-{}-{}-{}", config.llvm_version, config.os.as_str(), config.arch.as_str());
    let dst_folder = config.install_dir.join(&base);

    mp.println(format!(
        "Installing LLVM [{}] to [{}]",
        config.llvm_version, dst_folder.display()
    ))?;

    if dst_folder.exists() {
        mp.println(format!("[WARNING] Destination directory already exists: [{}]. Removing.", dst_folder.display()))?;
        std::fs::remove_dir_all(&dst_folder)?;
    }

    let (total_size, single_root) = {
        let file = std::fs::File::open(&archive_path)?;
        let zip = zip::ZipArchive::new(file)?;
        let mut roots = HashSet::new();
        for filename in zip.file_names() {
            roots.insert(filename.split('/').next().unwrap());
        }
        (zip.len(), roots.len() == 1)
    };

    let pb = Arc::new(Mutex::new(mp.add(make_zip_bar(u64::try_from(total_size)?, "Install"))));

    let mut channels = Vec::with_capacity(NUMBER_OF_WORKERS);
    for _ in 0..NUMBER_OF_WORKERS {
        channels.push(Vec::with_capacity((total_size / NUMBER_OF_WORKERS) + 1));
    }
    for i in 0..total_size {
        channels[i % NUMBER_OF_WORKERS].push(i);
    }

    let mut handlers = Vec::new();
    for channel in channels {
        let pb_handle = pb.clone();
        let archive_path_handle = archive_path.clone();
        let dst_folder_handle = dst_folder.clone();
        handlers.push(tokio::task::spawn_blocking(move || {
            let file = std::fs::File::open(archive_path_handle).unwrap();
            let mut zip = zip::ZipArchive::new(file).unwrap();
            for idx in channel {
                let mut src_file = zip.by_index(idx).unwrap();
                let mut src_file_name = src_file.name();
                if single_root {
                    let pos = src_file_name.find('/').unwrap();
                    src_file_name = &src_file_name[pos + 1..];
                }
                src_file_name = src_file_name.trim();
                if src_file_name.is_empty() {
                    continue;
                }
                let outpath = dst_folder_handle.join(src_file_name);
                if src_file_name.ends_with('/') {
                    std::fs::create_dir_all(&outpath).unwrap();
                } else {
                    if let Some(p) = outpath.parent()
                        && !p.exists()
                    {
                        std::fs::create_dir_all(p).unwrap();
                    }
                    let mut outfile = std::fs::File::create(&outpath)
                        .expect(&format!("Couldn't create [{}] file.", outpath.display()));
                    std::io::copy(&mut src_file, &mut outfile).unwrap();
                    outfile.flush().unwrap();
                }
                pb_handle.lock().unwrap().inc(1);
            }
        }));
    }

    for handler in handlers {
        handler.await?;
    }

    pb.lock().unwrap().finish_and_clear();

    Ok(dst_folder)
}

fn make_download_bar(len: u64, prefix: &str) -> ProgressBar {
    let template = if len > 0 {
        "{prefix:12} [{bar:40.cyan/blue}] {bytes}/{total_bytes} {eta}"
    } else {
        "{prefix:12} [{bar:40.cyan/blue}] {pos} {eta}"
    };

    // Hidden until `mp.add` so indicatif never draws the default `{wide_bar}` style
    // directly to stderr (e.g. `set_prefix` triggers an immediate draw).
    ProgressBar::with_draw_target(Some(len), ProgressDrawTarget::hidden())
        .with_style(
            ProgressStyle::with_template(template)
                .unwrap()
                .progress_chars("#>-"),
        )
        .with_prefix(prefix.to_string())
}

fn make_zip_bar(len: u64, prefix: &str) -> ProgressBar {
    let template = if len > 0 {
        "{prefix:12} [{bar:40.cyan/blue}] {pos}/{len} {eta}"
    } else {
        "{prefix:12} [{bar:40.cyan/blue}] {pos} {eta}"
    };

    // Hidden until `mp.add` so indicatif never draws the default `{wide_bar}` style
    // directly to stderr (e.g. `set_prefix` triggers an immediate draw).
    ProgressBar::with_draw_target(Some(len), ProgressDrawTarget::hidden())
        .with_style(
            ProgressStyle::with_template(template)
                .unwrap()
                .progress_chars("#>-"),
        )
        .with_prefix(prefix.to_string())
}

fn set_permissions_on_bin(llvm_dir: &PathBuf) {
    cfg_select!(
        any(target_os = "linux", target_os = "macos") => {
            use std::os::unix::fs::PermissionsExt;
            let mut bin_dir = llvm_dir.join("bin");
            if !bin_dir.exists() {
                let base = format!("llvm-{}-{}-{}", config.llvm_version, config.os.as_str(), config.arch.as_str());
                bin_dir = llvm_dir.join(&base).join("lib");
                if !bin_dir.exists() {
                    panic!("Bin directory not found in LLVM install dir.", bin_dir.display());
                }
            }
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
