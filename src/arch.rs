#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Arch {
    X86,
    X86_64,
    Arm,
    Arm64,
}

impl Arch {
    pub const fn as_str(self) -> &'static str {
        match self {
            Arch::X86 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::Arm => "arm",
            Arch::Arm64 => "arm64",
        }
    }

    #[inline]
    pub const fn current() -> Self {
        cfg_select!(
            target_arch = "x86" => Arch::X86,
            target_arch = "x86_64" => Arch::X86_64,
            target_arch = "aarch64" => Arch::Arm64,
            target_arch = "arm" => Arch::Arm,
            _ => panic!("Unsupported architecture."),
        )
    }
}
