use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

static CURRENT_ARCH: LazyLock<Arch> = LazyLock::new(|| {
    if cfg!(target_arch = "x86") {
        Arch::X86
    } else if cfg!(target_arch = "x86_64") {
        Arch::X86_64
    } else if cfg!(target_arch = "aarch64") {
        Arch::Arm64
    } else if cfg!(target_arch = "arm") {
        Arch::Arm
    } else {
        panic!("Unsupported architecture.");
    }
});

pub fn get_current_arch() -> &'static Arch {
    &CURRENT_ARCH
}
