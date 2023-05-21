#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum ValidationResult {
    Success,
    Error(String)
}

pub trait Validate {
    fn validate(&self) -> ValidationResult;
}
