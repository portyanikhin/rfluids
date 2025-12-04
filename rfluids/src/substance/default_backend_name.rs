use super::{BinaryMixKind, CustomMix, IncompPure, PredefinedMix, Pure, Substance};

const HELMHOLTZ_EOS_BACKEND_NAME: &str = "HEOS";
const INCOMP_BACKEND_NAME: &str = "INCOMP";

/// Default `CoolProp` backend name for a [`Substance`].
pub trait DefaultBackendName {
    /// Returns default `CoolProp` backend name for this [`Substance`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// let water = Pure::Water;
    /// assert_eq!(water.default_backend_name(), "HEOS");
    ///
    /// let incomp_water = IncompPure::Water;
    /// assert_eq!(incomp_water.default_backend_name(), "INCOMP");
    ///
    /// let r444a = PredefinedMix::R444A;
    /// assert_eq!(r444a.default_backend_name(), "HEOS");
    ///
    /// let propylene_glycol = BinaryMixKind::MPG;
    /// assert_eq!(propylene_glycol.default_backend_name(), "INCOMP");
    ///
    /// let custom_mix = CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)])?;
    /// assert_eq!(custom_mix.default_backend_name(), "HEOS");
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    fn default_backend_name(&self) -> &'static str;
}

impl DefaultBackendName for Substance {
    fn default_backend_name(&self) -> &'static str {
        match self {
            Substance::Pure(pure) => pure.default_backend_name(),
            Substance::IncompPure(incomp_pure) => incomp_pure.default_backend_name(),
            Substance::PredefinedMix(predefined_mix) => predefined_mix.default_backend_name(),
            Substance::BinaryMix(binary_mix) => binary_mix.kind.default_backend_name(),
            Substance::CustomMix(mix) => mix.default_backend_name(),
        }
    }
}

impl DefaultBackendName for Pure {
    fn default_backend_name(&self) -> &'static str {
        HELMHOLTZ_EOS_BACKEND_NAME
    }
}

impl DefaultBackendName for IncompPure {
    fn default_backend_name(&self) -> &'static str {
        INCOMP_BACKEND_NAME
    }
}

impl DefaultBackendName for PredefinedMix {
    fn default_backend_name(&self) -> &'static str {
        HELMHOLTZ_EOS_BACKEND_NAME
    }
}

impl DefaultBackendName for BinaryMixKind {
    fn default_backend_name(&self) -> &'static str {
        INCOMP_BACKEND_NAME
    }
}

impl DefaultBackendName for CustomMix {
    fn default_backend_name(&self) -> &'static str {
        HELMHOLTZ_EOS_BACKEND_NAME
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use strum::IntoEnumIterator;

    use super::*;

    #[rstest]
    #[case(Pure::Water, HELMHOLTZ_EOS_BACKEND_NAME)]
    #[case(IncompPure::Water, INCOMP_BACKEND_NAME)]
    #[case(PredefinedMix::R444A, HELMHOLTZ_EOS_BACKEND_NAME)]
    #[case(BinaryMixKind::MPG.with_fraction(0.4).unwrap(), INCOMP_BACKEND_NAME)]
    #[case(
        CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)])
            .unwrap(),
        HELMHOLTZ_EOS_BACKEND_NAME
    )]
    fn substance(#[case] value: impl Into<Substance>, #[case] expected: &str) {
        // Given
        let sut: Substance = value.into();

        // When
        let res = sut.default_backend_name();

        // Then
        assert_eq!(res, expected);
    }

    #[test]
    fn pure() {
        for substance in Pure::iter() {
            // When
            let res = substance.default_backend_name();

            // Then
            assert_eq!(res, HELMHOLTZ_EOS_BACKEND_NAME);
        }
    }

    #[test]
    fn incomp_pure() {
        for substance in IncompPure::iter() {
            // When
            let res = substance.default_backend_name();

            // Then
            assert_eq!(res, INCOMP_BACKEND_NAME);
        }
    }

    #[test]
    fn predefined_mix() {
        for substance in PredefinedMix::iter() {
            // When
            let res = substance.default_backend_name();

            // Then
            assert_eq!(res, HELMHOLTZ_EOS_BACKEND_NAME);
        }
    }

    #[test]
    fn binary_mix_kind() {
        for substance in BinaryMixKind::iter() {
            // When
            let res = substance.default_backend_name();

            // Then
            assert_eq!(res, INCOMP_BACKEND_NAME);
        }
    }

    #[test]
    fn custom_mix() {
        // Given
        let mix = CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)]).unwrap();

        // When
        let res = mix.default_backend_name();

        // Then
        assert_eq!(res, HELMHOLTZ_EOS_BACKEND_NAME);
    }
}
