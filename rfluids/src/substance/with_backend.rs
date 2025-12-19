use super::Substance;
use crate::fluid::backend::Backend;

/// Substance with specified `CoolProp` backend.
///
/// # See Also
///
/// - [`Substance`]
/// - [`Backend`]
/// - [`DefaultBackend`](crate::fluid::backend::DefaultBackend)
/// - [`Substance::with_backend`]
/// - [`Substance::with_default_backend`]
/// - [`Substance::into_with_backend`]
/// - [`Substance::into_with_default_backend`]
#[derive(Clone, Debug, PartialEq)]
pub struct SubstanceWithBackend {
    /// Substance.
    pub substance: Substance,
    /// `CoolProp` backend.
    pub backend: Backend,
}

impl SubstanceWithBackend {
    /// Returns name in the format `"<backend>::<substance>"`.
    ///
    /// This name is compatible with the `CoolProp` high level API
    /// ([`CoolProp`](crate::native::CoolProp)), so it can be used directly there.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let water = Substance::from(Pure::Water).into_with_default_backend();
    /// assert_eq!(water.name(), "HEOS::Water");
    ///
    /// let if97_water = Substance::from(Pure::Water).into_with_backend(BaseBackend::If97);
    /// assert_eq!(if97_water.name(), "IF97::Water");
    ///
    /// let incomp_water = Substance::from(IncompPure::Water).into_with_default_backend();
    /// assert_eq!(incomp_water.name(), "INCOMP::Water");
    /// ```
    ///
    /// # See Also
    ///
    /// - [`CoolProp::props_si`](crate::native::CoolProp::props_si)
    /// - [`CoolProp::props1_si`](crate::native::CoolProp::props1_si)
    /// - [`CoolProp::phase_si`](crate::native::CoolProp::phase_si)
    /// - [`Substance::name`]
    /// - [`Backend::name`]
    #[must_use]
    pub fn name(&self) -> String {
        format!("{}::{}", self.backend.name(), self.substance.name())
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::{fluid::backend::*, substance::*};

    #[rstest]
    #[case(Substance::from(Pure::R32).into_with_default_backend(), "HEOS::R32")]
    #[case(Substance::from(Pure::Water).into_with_backend(BaseBackend::If97), "IF97::Water")]
    #[case(
        Substance::from(Pure::Water).into_with_backend(BaseBackend::Srk.with(TabularMethod::Ttse)),
        "TTSE&SRK::Water"
    )]
    #[case(Substance::from(IncompPure::Water).into_with_default_backend(), "INCOMP::Water")]
    #[case(Substance::from(PredefinedMix::R444A).into_with_default_backend(), "HEOS::R444A.mix")]
    #[case(
        Substance::from(BinaryMixKind::MPG.with_fraction(0.4).unwrap()).into_with_default_backend(),
        "INCOMP::MPG[0.4]"
    )]
    #[case(
        Substance::from(CustomMix::mole_based(
            [(Pure::Methanol, 0.1), (Pure::Ethanol, 0.1), (Pure::Water, 0.8)]
        ).unwrap()).into_with_default_backend(),
        "HEOS::Water[0.8]&Ethanol[0.1]&Methanol[0.1]"
    )]
    fn name(#[case] sut: SubstanceWithBackend, #[case] expected: &str) {
        // When
        let res = sut.name();

        // Then
        assert_eq!(res, expected);
    }
}
