use std::path::PathBuf;

use crate::{arch::Arch, cli_args::CliArgs, llvm_version::LLVMVersion, os::OS};

pub struct Config {
    pub llvm_version: LLVMVersion,
    pub url_template: String,
    pub arch: Arch,
    pub os: OS,
    pub tmp_dir: PathBuf,
    pub install_dir: PathBuf,
}

impl Config {
    pub fn new(cli_args: CliArgs) -> Self {
        Self {
            llvm_version: cli_args.llvm_version,
            url_template: cli_args.url_template,
            arch: Arch::current(),
            os: OS::current(),
            tmp_dir: cli_args.tmp_dir,
            install_dir: cli_args.install_dir,
        }
    }
}
