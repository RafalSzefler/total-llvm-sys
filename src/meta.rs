/// Represents supported LLVM versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
#[must_use]
pub enum LLVMVersion {
    LLVM19 = 19,
    LLVM20 = 20,
    LLVM21 = 21,
}

/// Represents supported cpu architectures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[must_use]
pub enum Arch {
    X86 = 0,
    X86_64 = 1,
    Arm = 2,
    Arm64 = 3,
}

/// Represents supported operating systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[must_use]
pub enum OS {
    Windows = 0,
    Linux = 1,
    MacOS = 2,
}

/// General struct that keeps info about LLVM version,
/// current architecture and os.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[must_use]
pub struct Target {
    llvm_version: LLVMVersion,
    arch: Arch,
    os: OS,
}

impl Target {
    /// Creates a new [`Target`] instance.
    #[inline(always)]
    pub const fn new(llvm_version: LLVMVersion, arch: Arch, os: OS) -> Self {
        Self { llvm_version, arch, os }
    }

    #[inline(always)]
    pub const fn llvm_version(&self) -> LLVMVersion {
        self.llvm_version
    }

    #[inline(always)]
    pub const fn arch(&self) -> Arch {
        self.arch
    }

    #[inline(always)]
    pub const fn os(&self) -> OS {
        self.os
    }

    /// Retrieves current [`Target`]. This function is fully
    /// evaluated at compile time and has zero overhead.
    #[inline(always)]
    pub const fn get_current() -> Self {
        Self::new(get_current_llvm_version(), get_current_arch(), get_current_os())
    }
}

#[inline(always)]
const fn get_current_llvm_version() -> LLVMVersion {
    if cfg!(feature = "llvm-19") {
        LLVMVersion::LLVM19
    } else if cfg!(feature = "llvm-20") {
        LLVMVersion::LLVM20
    } else if cfg!(feature = "llvm-21") {
        LLVMVersion::LLVM21
    } else {
        panic!("Invalid llvm version")
    }
}

#[inline(always)]
const fn get_current_arch() -> Arch {
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
}

#[inline(always)]
const fn get_current_os() -> OS {
    if cfg!(target_os = "windows") {
        OS::Windows
    } else if cfg!(target_os = "linux") {
        OS::Linux
    } else if cfg!(target_os = "macos") {
        OS::MacOS
    } else {
        panic!("Unsupported OS.");
    }
}
