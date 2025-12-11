use coolprop_sys::COOLPROP;

use super::{
    CoolProp,
    common::{MessageBuffer, const_ptr_c_char},
};

const MIN_DEBUG_LEVEL: u8 = 0;
const MAX_DEBUG_LEVEL: u8 = 10;

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
    /// ```no_run
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
    pub fn get_debug_level() -> u8 {
        let level = unsafe { COOLPROP.lock().unwrap().get_debug_level() };
        level.clamp(MIN_DEBUG_LEVEL.into(), MAX_DEBUG_LEVEL.into()) as u8
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
    /// ```no_run
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
    pub fn set_debug_level(level: u8) {
        unsafe {
            COOLPROP
                .lock()
                .unwrap()
                .set_debug_level(level.clamp(MIN_DEBUG_LEVEL, MAX_DEBUG_LEVEL).into());
        }
    }

    /// Returns a global string parameter from `CoolProp`.
    ///
    /// Returns [`None`] if the parameter key is invalid or the value could not be retrieved.
    ///
    /// # Arguments
    ///
    /// - `param` -- global parameter key _(raw [`&str`](str) or
    ///   [`GlobalParam`](crate::io::GlobalParam))_
    ///
    /// Known parameter keys:
    ///
    /// - `"version"` -- `CoolProp` library version
    /// - `"gitrevision"` -- `git` commit hash of the `CoolProp` build
    /// - `"HOME"` -- `CoolProp` home directory path
    /// - `"REFPROP_version"` -- `REFPROP` version, if available
    /// - `"errstring"` -- pending error message
    /// - `"warnstring"` -- pending warning message
    /// - `"fluids_list"` -- list of all available pure and pseudo-pure substances
    ///   _(comma-separated)_
    /// - `"incompressible_list_pure"` -- list of all available pure incompressible substances
    ///   _(comma-separated)_
    /// - `"incompressible_list_solution"` -- list of all available incompressible binary mixtures
    ///   _(comma-separated)_
    /// - `"predefined_mixtures"` -- list of all available predefined mixtures _(comma-separated)_
    /// - `"cubic_fluids_list"` -- list of all substances for which a cubic EOS is applicable
    ///   _(comma-separated)_
    /// - `"mixture_binary_pairs_list"` -- list of binary pairs for mixtures _(comma-separated)_
    /// - `"parameter_list"` -- list of all available fluid parameters _(comma-separated)_
    /// - `"cubic_fluids_schema"` -- JSON schema for substances with cubic EOS
    /// - `"pcsaft_fluids_schema"` -- JSON schema for substances with PC-SAFT EOS
    ///
    /// # See Also
    ///
    /// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
    /// - [`CoolPropLib.h` Reference](https://coolprop.org/_static/doxygen/html/_cool_prop_lib_8h.html)
    /// - [`GlobalParam`](crate::io::GlobalParam)
    pub fn get_global_param(param: impl AsRef<str>) -> Option<String> {
        let param = param.as_ref().trim();
        let capacity = if ["version", "gitrevision", "HOME", "REFPROP_version"].contains(&param) {
            100
        } else {
            30_000
        };
        let res = MessageBuffer::with_capacity(capacity);
        let lock = COOLPROP.lock().unwrap();
        let status = unsafe {
            lock.get_global_param_string(const_ptr_c_char!(param), res.buffer, res.capacity)
        };
        let res: String = res.into();
        if status != 1 || res.trim().is_empty() { None } else { Some(res) }
    }

    /// Returns a string parameter for a specific substance from `CoolProp`.
    ///
    /// Returns [`None`] if the input is invalid or the value could not be retrieved.
    ///
    /// # Arguments
    ///
    /// - `substance_name` -- name of the substance _(raw [`&str`](str) or
    ///   [`Substance`](crate::substance::Substance) subset)_
    /// - `param` -- substance parameter key _(raw [`&str`](str) or
    ///   [`SubstanceParam`](crate::io::SubstanceParam))_
    ///
    /// Known parameter keys:
    ///
    /// - `"aliases"` -- list of aliases for the substance _(comma-separated)_
    /// - `"CAS"` -- CAS number
    /// - `"ASHRAE34"` -- ASHRAE Standard 34 safety rating
    /// - `"REFPROP_name"` -- substance name used in `REFPROP`
    /// - `"BibTeX-XXX"` -- BibTeX key, where `XXX` is one of the following:
    ///     - `EOS` -- equation of state reference
    ///     - `CP0` -- ideal gas heat capacity equation reference
    ///     - `CONDUCTIVITY` -- thermal conductivity equation reference
    ///     - `MELTING_LINE` -- melting line equation reference
    ///     - `SURFACE_TENSION` -- surface tension equation reference
    ///     - `VISCOSITY` -- viscosity equation reference
    /// - `"pure"` -- `"true"` if the substance is pure, `"false"` otherwise
    /// - `"formula"` -- chemical formula of the substance in LaTeX form _(if available)_
    ///
    /// # See Also
    ///
    /// - [CoolProp High-Level API](https://coolprop.org/coolprop/HighLevelAPI.html)
    /// - [Substance Information](https://coolprop.org/coolprop/HighLevelAPI.html#fluid-information)
    /// - [`CoolPropLib.h` Reference](https://coolprop.org/_static/doxygen/html/_cool_prop_lib_8h.html)
    /// - [`SubstanceParam`](crate::io::SubstanceParam)
    pub fn get_fluid_param_string(
        substance_name: impl AsRef<str>,
        param: impl AsRef<str>,
    ) -> Option<String> {
        let res = MessageBuffer::default();
        let lock = COOLPROP.lock().unwrap();
        let status = unsafe {
            lock.get_fluid_param_string(
                const_ptr_c_char!(substance_name.as_ref().trim()),
                const_ptr_c_char!(param.as_ref().trim()),
                res.buffer,
                res.capacity,
            )
        };
        let res: String = res.into();
        if status != 1 || res.trim().is_empty() { None } else { Some(res) }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::{io::GlobalParam::*, substance::Pure};

    #[test]
    fn get_debug_level() {
        // When
        let res = CoolProp::get_debug_level();

        // Then
        assert_eq!(res, MIN_DEBUG_LEVEL);
    }

    #[test]
    fn set_debug_level() {
        // When
        CoolProp::set_debug_level(MIN_DEBUG_LEVEL);
        let res = CoolProp::get_debug_level();

        // Then
        assert_eq!(res, MIN_DEBUG_LEVEL);
    }

    #[rstest]
    #[case(Version, true)]
    #[case(GitRevision, true)]
    #[case(HomePath, true)]
    #[case(PureList, true)]
    #[case(IncompPureList, true)]
    #[case(BinaryMixList, true)]
    #[case(PredefinedMixList, true)]
    #[case(CubicList, true)]
    #[case(MixBinaryPairsList, true)]
    #[case(ParamList, true)]
    #[case(CubicJsonSchema, true)]
    #[case(PcSaftJsonSchema, true)]
    #[case("version", true)]
    #[case("gitrevision", true)]
    #[case("HOME", true)]
    #[case("fluids_list", true)]
    #[case("incompressible_list_pure", true)]
    #[case("incompressible_list_solution", true)]
    #[case("predefined_mixtures", true)]
    #[case("cubic_fluids_list", true)]
    #[case("mixture_binary_pairs_list", true)]
    #[case("parameter_list", true)]
    #[case("cubic_fluids_schema", true)]
    #[case("pcsaft_fluids_schema", true)]
    #[case("", false)]
    #[case(" ", false)]
    #[case("Hello, World!", false)]
    fn get_global_param(#[case] param: impl AsRef<str>, #[case] is_some: bool) {
        // When
        let res = CoolProp::get_global_param(param);

        // Then
        assert_eq!(res.is_some(), is_some);
    }

    #[rstest]
    #[case("aliases", true)]
    #[case("CAS", true)]
    #[case("ASHRAE34", true)]
    #[case("REFPROP_name", true)]
    #[case("BibTeX-EOS", true)]
    #[case("BibTeX-CP0", false)]
    #[case("BibTeX-CONDUCTIVITY", true)]
    #[case("BibTeX-MELTING_LINE", true)]
    #[case("BibTeX-SURFACE_TENSION", true)]
    #[case("BibTeX-VISCOSITY", true)]
    #[case("pure", true)]
    #[case("formula", true)]
    #[case("", false)]
    #[case(" ", false)]
    #[case("Hello, World!", false)]
    fn get_fluid_param_string(#[case] param: &str, #[case] is_some: bool) {
        // Given
        let fluid = Pure::Water;

        // When
        let res = CoolProp::get_fluid_param_string(fluid, param);

        // Then
        assert_eq!(res.is_some(), is_some);
    }
}
