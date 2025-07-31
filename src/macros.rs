macro_rules! valid_targets {
    ($($target:literal),*) => {
        #[cfg(not(any($(target_arch = $target),*)))]
        compile_error!("total-llvm-sys does not support current target architecture.");
    };
}

pub(crate) use valid_targets;
