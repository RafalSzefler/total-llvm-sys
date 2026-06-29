use std::{ffi::OsString, path::PathBuf};

use clap::Parser;

use super::llvm_version::LLVMVersion;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// LLVM version to download.
    #[arg(long, default_value_t = LLVMVersion::LLVM_22)]
    pub llvm_version: LLVMVersion,

    /// URL template to download packed LLVM from.
    #[arg(long, default_value = "https://github.com/RafalSzefler/llvm-builds/releases/download/core/llvm-{llvmVersion}-{os}-{arch}.zip")]
    pub url_template: String,

    /// Temporary directory, where the LLVM archive will be downloaded.
    #[arg(long, default_value = tmp_dir())]
    pub tmp_dir: PathBuf,

    /// Installation directory, where the LLVM will be installed.
    #[arg(long, default_value = install_dir())]
    pub install_dir: PathBuf,
}

impl CliArgs {
    pub fn parse_command_line_args() -> Self {
        Self::parse()
    }
}

fn tmp_dir() -> OsString {
    std::env::temp_dir().into_os_string()
}

fn install_dir() -> OsString {
    let base_dirs = directories::BaseDirs::new().unwrap();
    base_dirs.cache_dir().join("llvm_downloader").into_os_string()
}
