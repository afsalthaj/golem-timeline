// Trait representing types that support negation
pub trait Negatable {
    fn negate(&self) -> Self;
}

// Implement Negatable for boolean type
impl Negatable for bool {
    fn negate(&self) -> Self {
        !self
    }
}
