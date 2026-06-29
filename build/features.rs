#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum LLVMFeatures {
    LLVM_22,
}

impl LLVMFeatures {
    pub const fn as_str(self) -> &'static str {
        match self {
            LLVMFeatures::LLVM_22 => "llvm-22",
        }
    }

    #[inline]
    fn reverse_cargo_feature_name(name: &str) -> Option<LLVMFeatures> {
        match name {
            "CARGO_FEATURE_LLVM_22" => Some(LLVMFeatures::LLVM_22),
            _ => None,
        }
    }
}

pub fn get_current_llvm_feature() -> LLVMFeatures {
    let mut feature: Option<LLVMFeatures> = None;

    for (key, value) in std::env::vars() {
        if value.trim() != "1" {
            continue;
        }

        if let Some(set_feature) = LLVMFeatures::reverse_cargo_feature_name(key.as_str()) {
            if feature.is_some() {
                panic!("Multiple [llvm-*] features specified. Only one is allowed.");
            }
            feature = Some(set_feature);
        }
    }

    feature.unwrap_or_else(|| panic!("No [llvm-*] feature specified."))
}
