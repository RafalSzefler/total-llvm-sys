use std::{collections::HashSet, sync::LazyLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LLVMFeatures {
    LLVM19,
    LLVM20,
    LLVM21,
}

impl LLVMFeatures {
    pub const fn as_str(self) -> &'static str {
        match self {
            LLVMFeatures::LLVM19 => "llvm-19",
            LLVMFeatures::LLVM20 => "llvm-20",
            LLVMFeatures::LLVM21 => "llvm-21",
        }
    }
}

static CURRENT_LLVM_FEATURE: LazyLock<LLVMFeatures> = LazyLock::new(|| {
    let mut features = HashSet::<LLVMFeatures>::new();

    if cfg!(feature = "llvm-19") {
        features.insert(LLVMFeatures::LLVM19);
    }

    if cfg!(feature = "llvm-20") {
        features.insert(LLVMFeatures::LLVM20);
    }

    if cfg!(feature = "llvm-21") {
        features.insert(LLVMFeatures::LLVM21);
    }

    assert!(!features.is_empty(), "No [llvm-*] features specified.");
    assert!(
        features.len() == 1,
        "Multiple [llvm-*] features specified. Only one is allowed."
    );

    features.into_iter().next().unwrap()
});

pub fn get_current_llvm_feature() -> &'static LLVMFeatures {
    &CURRENT_LLVM_FEATURE
}
