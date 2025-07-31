use std::{collections::HashSet, sync::LazyLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LLVMFeatures {
    LLVM19,
}

impl LLVMFeatures {
    pub const fn as_str(self) -> &'static str {
        match self {
            LLVMFeatures::LLVM19 => "llvm-19",
        }
    }
}

static CURRENT_LLVM_FEATURE: LazyLock<LLVMFeatures> = LazyLock::new(|| {
    let mut features = HashSet::<LLVMFeatures>::new();

    #[cfg(feature = "llvm-19")]
    features.insert(LLVMFeatures::LLVM19);

    assert!(!features.is_empty(), "No LLVM features specified.");
    assert!(features.len() == 1, "Multiple LLVM features specified.");

    features.into_iter().next().unwrap()
});

pub fn get_current_llvm_feature() -> &'static LLVMFeatures {
    &CURRENT_LLVM_FEATURE
}
