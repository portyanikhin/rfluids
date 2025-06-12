/// Keyed input.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct Input<T: Copy> {
    /// Specified key.
    pub key: T,
    /// Specified value **\[SI units\]**.
    pub value: f64,
}
