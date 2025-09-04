/// Initializes the native target.
///
/// # Notes
///
/// This function is `static inline` in the LLVM source code. And thus
/// it doesn't end up in the final binary. Therefore we provide our
/// own wrapper.
///
/// # Safety
///
/// This function is unsafe because it calls LLVM functions that are not safe.
///
/// # Panics
///
/// This function panics if the native target couldn't be detected. This error
/// should be detected at compile time though, this check here is just a fallback.
#[inline(always)]
pub unsafe fn initialize_target() {
    unsafe {
        if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
            llvm_sys::target::LLVMInitializeX86Target();
            llvm_sys::target::LLVMInitializeX86TargetInfo();
            llvm_sys::target::LLVMInitializeX86TargetMC();
        } else if cfg!(target_arch = "aarch64") {
            llvm_sys::target::LLVMInitializeAArch64Target();
            llvm_sys::target::LLVMInitializeAArch64TargetInfo();
            llvm_sys::target::LLVMInitializeAArch64TargetMC();
        } else if cfg!(target_arch = "arm") {
            llvm_sys::target::LLVMInitializeARMTarget();
            llvm_sys::target::LLVMInitializeARMTargetInfo();
            llvm_sys::target::LLVMInitializeARMTargetMC();
        } else {
            panic!("initialize_target: unsupported architecture.");
        }
    }
}

/// Initializes the native asm printer.
///
/// # Notes
///
/// This function is `static inline` in the LLVM source code. And thus
/// it doesn't end up in the final binary. Therefore we provide our
/// own wrapper.
///
/// # Safety
///
/// This function is unsafe because it calls LLVM functions that are not safe.
///
/// # Panics
///
/// This function panics if the native target couldn't be detected. This error
/// should be detected at compile time though, this check here is just a fallback.
#[inline(always)]
pub unsafe fn initialize_asm_printer() {
    unsafe {
        if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
            llvm_sys::target::LLVMInitializeX86AsmPrinter();
        } else if cfg!(target_arch = "aarch64") {
            llvm_sys::target::LLVMInitializeAArch64AsmPrinter();
        } else if cfg!(target_arch = "arm") {
            llvm_sys::target::LLVMInitializeARMAsmPrinter();
        } else {
            panic!("initialize_asm_printer: unsupported architecture.");
        }
    }
}

/// Initializes the native asm parser.
///
/// # Notes
///
/// This function is `static inline` in the LLVM source code. And thus
/// it doesn't end up in the final binary. Therefore we provide our
/// own wrapper.
///
/// # Safety
///
/// This function is unsafe because it calls LLVM functions that are not safe.
///
/// # Panics
///
/// This function panics if the native target couldn't be detected. This error
/// should be detected at compile time though, this check here is just a fallback.
#[inline(always)]
pub unsafe fn initialize_asm_parser() {
    unsafe {
        if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
            llvm_sys::target::LLVMInitializeX86AsmParser();
        } else if cfg!(target_arch = "aarch64") {
            llvm_sys::target::LLVMInitializeAArch64AsmParser();
        } else if cfg!(target_arch = "arm") {
            llvm_sys::target::LLVMInitializeARMAsmParser();
        } else {
            panic!("initialize_asm_parser: unsupported architecture.");
        }
    }
}

/// Initializes the native disassembler.
///     
/// # Notes
///
/// This function is `static inline` in the LLVM source code. And thus
/// it doesn't end up in the final binary. Therefore we provide our
/// own wrapper.
///
/// # Safety
///
/// This function is unsafe because it calls LLVM functions that are not safe.
///
/// # Panics
///
/// This function panics if the native target couldn't be detected. This error
/// should be detected at compile time though, this check here is just a fallback.
#[inline(always)]
pub unsafe fn initialize_disassembler() {
    unsafe {
        if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
            llvm_sys::target::LLVMInitializeX86Disassembler();
        } else if cfg!(target_arch = "aarch64") {
            llvm_sys::target::LLVMInitializeAArch64Disassembler();
        } else if cfg!(target_arch = "arm") {
            llvm_sys::target::LLVMInitializeARMDisassembler();
        } else {
            panic!("initialize_disassembler: unsupported architecture.");
        }
    }
}
