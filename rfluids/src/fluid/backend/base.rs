use super::{Backend, TabularMethod};

/// Base `CoolProp` backend.
///
/// Represents the base backends available in `CoolProp`, each using a different
/// equation of state or calculation method for thermophysical property calculations.
///
/// By default, [`Fluid`](crate::fluid::Fluid) uses [`Heos`](BaseBackend::Heos)
/// for most pure fluids and mixtures. You can override this using
/// [`Fluid::builder`](crate::fluid::Fluid::builder).
///
/// # Examples
///
/// For example, to use the `IF97` backend for water:
///
/// ```
/// use rfluids::prelude::*;
///
/// let water = Fluid::builder().substance(Pure::Water).with_backend(BaseBackend::If97).build()?;
///
/// assert_eq!(water.backend().name(), "IF97");
/// # Ok::<(), rfluids::Error>(())
/// ```
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
///
/// use rfluids::prelude::*;
///
/// assert_eq!(BaseBackend::Heos.as_ref(), "HEOS");
/// assert_eq!(BaseBackend::from_str("HEOS"), Ok(BaseBackend::Heos));
/// assert_eq!(BaseBackend::try_from("heos"), Ok(BaseBackend::Heos));
/// ```
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[non_exhaustive]
pub enum BaseBackend {
    /// Helmholtz Equation of State.
    ///
    /// The default backend for pure fluids and mixtures in `CoolProp`.
    /// Uses high-accuracy multi-parameter Helmholtz energy equations of state.
    ///
    /// Nearly all fluids in `CoolProp` are based on Helmholtz energy formulations,
    /// which use temperature and density as independent properties.
    ///
    /// # See Also
    ///
    /// - [Pure and Pseudo-Pure Fluid Properties](https://coolprop.org/fluid_properties/PurePseudoPure.html)
    #[strum(to_string = "HEOS")]
    Heos,

    /// `REFPROP` backend.
    ///
    /// Provides access to the [`NIST REFPROP`](https://www.nist.gov/srd/refprop) library,
    /// _"the gold standard in thermophysical properties"_.
    ///
    /// Requires `REFPROP` to be installed separately and accessible at runtime.
    ///
    /// # See Also
    ///
    /// - [REFPROP Interface](https://coolprop.org/coolprop/REFPROP.html)
    #[strum(to_string = "REFPROP")]
    Refprop,

    /// Incompressible fluid backend.
    ///
    /// Used for incompressible pure fluids and binary mixtures
    /// _(both mass-based and volume-based)_.
    ///
    /// Supports aqueous solutions, heat transfer oils, and other incompressible
    /// fluids modeled with the assumption that density is a function of
    /// temperature and fraction only, independent of pressure.
    ///
    /// # See Also
    ///
    /// - [Incompressible Fluids](https://coolprop.org/fluid_properties/Incompressibles.html)
    #[strum(to_string = "INCOMP")]
    Incomp,

    /// IAPWS-IF97 formulation for water and steam.
    ///
    /// The _"IAPWS Industrial Formulation 1997 for the Thermodynamic Properties
    /// of Water and Steam"_ is designed for industrial use, prioritizing
    /// computational speed over maximum accuracy.
    ///
    /// Provides much faster calculation than `HEOS`, with properties within 1%
    /// uncertainty _(often within 0.1%)_ except near the critical point. Only
    /// supports water.
    ///
    /// # See Also
    ///
    /// - [IF97 Steam/Water Properties](https://coolprop.org/fluid_properties/IF97.html)
    #[strum(to_string = "IF97")]
    If97,

    /// Soave-Redlich-Kwong cubic equation of state.
    ///
    /// Requires only critical temperature, critical pressure, and acentric factor.
    /// Applicable to most fluids in `CoolProp`.
    ///
    /// Dramatically faster than multi-parameter Helmholtz models,
    /// with the trade-off of reduced accuracy.
    ///
    /// # See Also
    ///
    /// - [Cubic Equations of State](https://coolprop.org/coolprop/Cubics.html)
    #[strum(to_string = "SRK")]
    Srk,

    /// Peng-Robinson cubic equation of state.
    ///
    /// Requires only critical temperature, critical pressure, and acentric factor.
    /// Applicable to most fluids in `CoolProp`.
    ///
    /// Dramatically faster than multi-parameter Helmholtz models,
    /// with the trade-off of reduced accuracy.
    ///
    /// # See Also
    ///
    /// - [Cubic Equations of State](https://coolprop.org/coolprop/Cubics.html)
    #[strum(to_string = "PR")]
    Pr,

    /// Volume-translated Peng-Robinson equation of state.
    ///
    /// An improved version of the Peng-Robinson equation ([`BaseBackend::Pr`])
    /// with volume translation for better liquid density predictions.
    /// Uses UNIFAC group contribution methods for enhanced accuracy.
    ///
    /// Requires the `VTPR_UNIFAC_PATH` environment variable (or `CoolProp` configuration variable)
    /// to be set with the path to the UNIFAC JSON files.
    ///
    /// # See Also
    ///
    /// - [Cubic Equations of State](https://coolprop.org/coolprop/Cubics.html)
    /// - [Configuration Variables](https://coolprop.org/coolprop/Configuration.html)
    #[strum(to_string = "VTPR")]
    VtPr,

    /// PC-SAFT equation of state.
    ///
    /// Perturbed-Chain Statistical Associating Fluid Theory.
    ///
    /// Available for more than 100 fluids and supports mixtures using binary
    /// interaction parameters.
    ///
    /// # See Also
    ///
    /// - [PC-SAFT Equations of State](https://coolprop.org/coolprop/PCSAFT.html)
    #[strum(to_string = "PCSAFT")]
    PcSaft,
}

impl BaseBackend {
    /// Combines this base backend with a tabular interpolation method ([`TabularMethod`]).
    ///
    /// Creates a [`Backend::Tabular`] that uses pre-computed gridded data
    /// for dramatically faster property calculations.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let backend = BaseBackend::Heos.with(TabularMethod::Ttse);
    /// let water = Fluid::builder().substance(Pure::Water).with_backend(backend).build()?;
    ///
    /// assert_eq!(water.backend().name(), "TTSE&HEOS");
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    #[must_use]
    pub fn with(self, tabular_method: TabularMethod) -> Backend {
        Backend::Tabular { base: self, method: tabular_method }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::*;

    use super::{BaseBackend::*, *};

    #[rstest]
    #[case(Heos, "HEOS")]
    #[case(Refprop, "REFPROP")]
    #[case(Incomp, "INCOMP")]
    #[case(If97, "IF97")]
    #[case(Srk, "SRK")]
    #[case(Pr, "PR")]
    #[case(VtPr, "VTPR")]
    #[case(PcSaft, "PCSAFT")]
    fn as_str(#[case] sut: BaseBackend, #[case] expected: &str) {
        // When
        let str = sut.as_ref();
        let static_str: &'static str = sut.into();

        // Then
        assert_eq!(str, expected);
        assert_eq!(static_str, expected);
    }

    #[rstest]
    #[case(vec!["HEOS", "heos"], Heos)]
    #[case(vec!["REFPROP", "refprop"], Refprop)]
    #[case(vec!["INCOMP", "incomp"], Incomp)]
    #[case(vec!["IF97", "if97"], If97)]
    #[case(vec!["SRK", "srk"], Srk)]
    #[case(vec!["PR", "pr"], Pr)]
    #[case(vec!["VTPR", "vtpr"], VtPr)]
    #[case(vec!["PCSAFT", "pcsaft"], PcSaft)]
    fn from_valid_str(#[case] valid: Vec<&str>, #[case] expected: BaseBackend) {
        for s in valid {
            // When
            let res1 = BaseBackend::from_str(s).unwrap();
            let res2 = BaseBackend::try_from(s).unwrap();

            // Then
            assert_eq!(res1, expected);
            assert_eq!(res2, expected);
        }
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str(#[case] invalid: &str) {
        // When
        let res1 = BaseBackend::from_str(invalid);
        let res2 = BaseBackend::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }
}
