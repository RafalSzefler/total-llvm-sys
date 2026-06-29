#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
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

    #[inline]
    pub const fn current() -> Self {
        cfg_select!(
            target_os = "windows" => OS::Windows,
            target_os = "linux" => OS::Linux,
            target_os = "macos" => OS::MacOS,
            _ => panic!("Unsupported OS."),
        )
    }
}
