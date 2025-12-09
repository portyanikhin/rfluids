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
    ///
    /// # See Also
    ///
    /// - [`CoolProp::set_debug_level`](Self::set_debug_level)
    #[must_use]
    pub fn get_debug_level() -> i32 {
        unsafe { COOLPROP.lock().unwrap().get_debug_level() }
    }

    /// Sets the `CoolProp` debug level.
    ///
    /// Controls the verbosity of debugging output for subsequent `CoolProp` operations.
    ///
    /// # Arguments
    ///
    /// - `level` -- debug level to set (valid range: `0-10`, larger values produce more verbose
    ///   output)
    ///   - `0` -- disable debugging output
    ///   - `> 0` -- enable debugging output with corresponding verbosity level
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// // Set maximum verbosity
    /// CoolProp::set_debug_level(10);
    /// assert_eq!(CoolProp::get_debug_level(), 10);
    ///
    /// // Disable debugging output
    /// CoolProp::set_debug_level(0);
    /// assert_eq!(CoolProp::get_debug_level(), 0);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`CoolProp::get_debug_level`](Self::get_debug_level)
    pub fn set_debug_level(level: i32) {
        unsafe { COOLPROP.lock().unwrap().set_debug_level(level) }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[test]
    fn get_debug_level() {
        // When
        let res = CoolProp::get_debug_level();

        // Then
        assert_eq!(res, 0);
    }

    #[rstest]
    #[case(3)]
    #[case(-42)]
    #[case(0)]
    fn set_debug_level(#[case] value: i32) {
        // When
        CoolProp::set_debug_level(value);
        let res = CoolProp::get_debug_level();

        // Then
        assert_eq!(res, value);
    }
}
