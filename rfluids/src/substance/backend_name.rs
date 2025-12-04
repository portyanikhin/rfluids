use super::{BinaryMixKind, CustomMix, IncompPure, PredefinedMix, Pure, Substance};

const HELMHOLTZ_EOS_BACKEND_NAME: &str = "HEOS";
const INCOMP_BACKEND_NAME: &str = "INCOMP";

/// `CoolProp` backend name.
pub trait BackendName {
    /// Returns `CoolProp` backend name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::prelude::*;
    ///
    /// assert_eq!(Pure::Water.backend_name(), "HEOS");
    /// assert_eq!(IncompPure::Water.backend_name(), "INCOMP");
    /// assert_eq!(PredefinedMix::R444A.backend_name(), "HEOS");
    /// assert_eq!(BinaryMixKind::MPG.backend_name(), "INCOMP");
    /// assert_eq!(
    ///     CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4),])?.backend_name(),
    ///     "HEOS"
    /// );
    /// # Ok::<(), rfluids::Error>(())
    /// ```
    fn backend_name(&self) -> &'static str;
}

impl BackendName for Substance {
    fn backend_name(&self) -> &'static str {
        match self {
            Substance::Pure(pure) => pure.backend_name(),
            Substance::IncompPure(incomp_pure) => incomp_pure.backend_name(),
            Substance::PredefinedMix(predefined_mix) => predefined_mix.backend_name(),
            Substance::BinaryMix(binary_mix) => binary_mix.kind.backend_name(),
            Substance::CustomMix(mix) => mix.backend_name(),
        }
    }
}

impl BackendName for Pure {
    fn backend_name(&self) -> &'static str {
        HELMHOLTZ_EOS_BACKEND_NAME
    }
}

impl BackendName for IncompPure {
    fn backend_name(&self) -> &'static str {
        INCOMP_BACKEND_NAME
    }
}

impl BackendName for PredefinedMix {
    fn backend_name(&self) -> &'static str {
        HELMHOLTZ_EOS_BACKEND_NAME
    }
}

impl BackendName for BinaryMixKind {
    fn backend_name(&self) -> &'static str {
        INCOMP_BACKEND_NAME
    }
}

impl BackendName for CustomMix {
    fn backend_name(&self) -> &'static str {
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
        let res = sut.backend_name();

        // Then
        assert_eq!(res, expected);
    }

    #[test]
    fn pure() {
        for substance in Pure::iter() {
            // When
            let res = substance.backend_name();

            // Then
            assert_eq!(res, HELMHOLTZ_EOS_BACKEND_NAME);
        }
    }

    #[test]
    fn incomp_pure() {
        for substance in IncompPure::iter() {
            // When
            let res = substance.backend_name();

            // Then
            assert_eq!(res, INCOMP_BACKEND_NAME);
        }
    }

    #[test]
    fn predefined_mix() {
        for substance in PredefinedMix::iter() {
            // When
            let res = substance.backend_name();

            // Then
            assert_eq!(res, HELMHOLTZ_EOS_BACKEND_NAME);
        }
    }

    #[test]
    fn binary_mix_kind() {
        for substance in BinaryMixKind::iter() {
            // When
            let res = substance.backend_name();

            // Then
            assert_eq!(res, INCOMP_BACKEND_NAME);
        }
    }

    #[test]
    fn custom_mix() {
        // Given
        let mix = CustomMix::mass_based([(Pure::Water, 0.6), (Pure::Ethanol, 0.4)]).unwrap();

        // When
        let res = mix.backend_name();

        // Then
        assert_eq!(res, HELMHOLTZ_EOS_BACKEND_NAME);
    }
}
