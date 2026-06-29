#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum LLVMVersion {
    #[default]
    LLVM_22,
}

impl std::fmt::Display for LLVMVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            LLVMVersion::LLVM_22 => "22",
        };
        write!(f, "{}", text)
    }
}

impl std::str::FromStr for LLVMVersion {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "22" => Ok(LLVMVersion::LLVM_22),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid LLVM version")),
        }
    }
}