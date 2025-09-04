macro_rules! valid_targets {
    ($($target:literal),*) => {
        #[cfg(not(any($(target_arch = $target),*)))]
        compile_error!("total-llvm-sys does not support current target architecture.");
    };
}

pub(crate) use valid_targets;

#[cfg(feature = "llvm-19")]
macro_rules! reexport_llvm {
    () => {
        pub extern crate llvm_sys_19;
        pub extern crate llvm_sys_19 as llvm_sys;
    };
}

#[cfg(feature = "llvm-20")]
macro_rules! reexport_llvm {
    () => {
        pub extern crate llvm_sys_20;
        pub extern crate llvm_sys_20 as llvm_sys;
    };
}

#[cfg(feature = "llvm-21")]
macro_rules! reexport_llvm {
    () => {
        pub extern crate llvm_sys_21;
        pub extern crate llvm_sys_21 as llvm_sys;
    };
}

pub(crate) use reexport_llvm;
