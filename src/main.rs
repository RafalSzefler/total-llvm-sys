mod arch;
mod config;
mod downloader;
mod cli_args;
mod llvm_version;
mod os;

use tokio::runtime::Builder;

use config::Config;


pub fn main() -> Result<(), anyhow::Error> {
    #[cfg(windows)]
    let _ = enable_ansi_support::enable_ansi_support();

    let cli_args = cli_args::CliArgs::parse_command_line_args();
    let config = Config::new(cli_args);

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("Failed building the tokio runtime");

    runtime.block_on(downloader::download_and_install_llvm(&config))
}
