/// Keyed input.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Input<T: Copy>(pub(crate) T, pub(crate) f64);

impl<T: Copy> Input<T> {
    /// Specified key.
    pub fn key(&self) -> T {
        self.0
    }

    /// Specified value _(in SI units)_.
    pub fn si_value(&self) -> f64 {
        self.1
    }
}
