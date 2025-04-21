/// Keyed input.
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub struct Input<T: Copy> {
    /// Specified key.
    pub key: T,
    /// Specified value **\[SI units\]**.
    pub value: f64,
}
