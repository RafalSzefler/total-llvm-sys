use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OS {
    Windows,
    Linux,
    MacOS,
}

impl OS {
    pub const fn as_str(self) -> &'static str {
        match self {
            OS::Windows => "windows",
            OS::Linux => "linux",
            OS::MacOS => "macos",
        }
    }
}

static CURRENT_OS: LazyLock<OS> = LazyLock::new(|| {
    if cfg!(target_os = "windows") {
        OS::Windows
    } else if cfg!(target_os = "linux") {
        OS::Linux
    } else if cfg!(target_os = "macos") {
        OS::MacOS
    } else {
        panic!("Unsupported OS.");
    }
});

pub fn get_current_os() -> &'static OS {
    &CURRENT_OS
}