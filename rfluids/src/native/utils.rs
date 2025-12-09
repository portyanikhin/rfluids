use coolprop_sys::COOLPROP;

use super::CoolProp;

impl CoolProp {
    /// Returns the current `CoolProp` debug level.
    ///
    /// The debug level controls the verbosity of debugging output from `CoolProp`.
    /// Valid range is `0-10`:
    ///
    /// - `0` -- debugging output is disabled
    /// - `> 0` -- debugging output is enabled, with larger values producing more verbose output
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// // By default, debugging output is disabled
    /// let debug_level = CoolProp::get_debug_level();
    /// assert_eq!(debug_level, 0);
    /// ```
    #[must_use]
    pub fn get_debug_level() -> i32 {
        unsafe { COOLPROP.lock().unwrap().get_debug_level() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_debug_level() {
        // When
        let res = CoolProp::get_debug_level();

        // Then
        assert_eq!(res, 0);
    }
}
