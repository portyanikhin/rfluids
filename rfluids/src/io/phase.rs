use super::try_from;
use strum_macros::{AsRefStr, EnumString, FromRepr};

/// Phase states of fluids and mixtures.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::prelude::*;
///
/// assert_eq!(Phase::Liquid.as_ref(), "phase_liquid");
/// assert_eq!(Phase::from_str("phase_liquid"), Ok(Phase::Liquid));
/// assert_eq!(Phase::try_from("liquid"), Ok(Phase::Liquid));
/// ```
///
/// Conversion between [`u8`]:
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(u8::from(Phase::Gas), 5);
/// assert_eq!(Phase::try_from(5), Ok(Phase::Gas));
/// ```
///
/// Conversion between [`f64`]:
///
/// ```
/// use rfluids::prelude::*;
///
/// assert_eq!(Phase::try_from(5.0), Ok(Phase::Gas));
/// ```
///
/// # See also
///
/// - [Imposing the phase (optional)](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
#[derive(AsRefStr, Copy, Clone, Debug, EnumString, Eq, FromRepr, PartialEq)]
#[strum(ascii_case_insensitive)]
#[repr(u8)]
pub enum Phase {
    /// Liquid _([`P`](crate::io::FluidParam::P) <
    /// [`PCritical`](crate::io::FluidTrivialParam::PCritical) &
    /// [`T`](crate::io::FluidParam::T) <
    /// [`TCritical`](crate::io::FluidTrivialParam::TCritical); above saturation)_.
    #[strum(to_string = "phase_liquid", serialize = "liquid")]
    Liquid = 0,

    /// Supercritical fluid _([`P`](crate::io::FluidParam::P) >
    /// [`PCritical`](crate::io::FluidTrivialParam::PCritical) &
    /// [`T`](crate::io::FluidParam::T) >
    /// [`TCritical`](crate::io::FluidTrivialParam::TCritical))_.
    #[strum(to_string = "phase_supercritical", serialize = "supercritical")]
    Supercritical = 1,

    /// Supercritical gas _([`P`](crate::io::FluidParam::P) <
    /// [`PCritical`](crate::io::FluidTrivialParam::PCritical) &
    /// [`T`](crate::io::FluidParam::T) >
    /// [`TCritical`](crate::io::FluidTrivialParam::TCritical))_.
    #[strum(
        to_string = "phase_supercritical_gas",
        serialize = "supercritical_gas",
        serialize = "SupercriticalGas"
    )]
    SupercriticalGas = 2,

    /// Supercritical liquid _([`P`](crate::io::FluidParam::P) >
    /// [`PCritical`](crate::io::FluidTrivialParam::PCritical) &
    /// [`T`](crate::io::FluidParam::T) <
    /// [`TCritical`](crate::io::FluidTrivialParam::TCritical))_.
    #[strum(
        to_string = "phase_supercritical_liquid",
        serialize = "supercritical_liquid",
        serialize = "SupercriticalLiquid"
    )]
    SupercriticalLiquid = 3,

    /// Critical point _([`P`](crate::io::FluidParam::P) =
    /// [`PCritical`](crate::io::FluidTrivialParam::PCritical) &
    /// [`T`](crate::io::FluidParam::T) =
    /// [`TCritical`](crate::io::FluidTrivialParam::TCritical))_.
    #[strum(
        to_string = "phase_critical_point",
        serialize = "critical_point",
        serialize = "CriticalPoint"
    )]
    CriticalPoint = 4,

    /// Gas _([`P`](crate::io::FluidParam::P) <
    /// [`PCritical`](crate::io::FluidTrivialParam::PCritical) &
    /// [`T`](crate::io::FluidParam::T) <
    /// [`TCritical`](crate::io::FluidTrivialParam::TCritical); below saturation)_.
    #[strum(to_string = "phase_gas", serialize = "gas")]
    Gas = 5,

    /// Two-phase fluid _([`P`](crate::io::FluidParam::P) <
    /// [`PCritical`](crate::io::FluidTrivialParam::PCritical) &
    /// [`T`](crate::io::FluidParam::T) <
    /// [`TCritical`](crate::io::FluidTrivialParam::TCritical); mixed liquid/gas)_.
    #[strum(
        to_string = "phase_twophase",
        serialize = "phase_two_phase",
        serialize = "two_phase",
        serialize = "TwoPhase"
    )]
    TwoPhase = 6,

    /// Unknown phase.
    #[strum(to_string = "phase_unknown", serialize = "unknown")]
    Unknown = 7,

    /// `CoolProp` to determine phase.
    #[strum(
        to_string = "phase_not_imposed",
        serialize = "not_imposed",
        serialize = "NotImposed"
    )]
    NotImposed = 8,
}

impl From<Phase> for u8 {
    fn from(value: Phase) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for Phase {
    type Error = strum::ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Phase::from_repr(value).ok_or(strum::ParseError::VariantNotFound)
    }
}

impl TryFrom<f64> for Phase {
    type Error = strum::ParseError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        try_from(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Phase::*, *};
    use rstest::*;
    use std::str::FromStr;

    #[rstest]
    #[case(Liquid, "phase_liquid")]
    #[case(Supercritical, "phase_supercritical")]
    #[case(SupercriticalGas, "phase_supercritical_gas")]
    #[case(SupercriticalLiquid, "phase_supercritical_liquid")]
    #[case(CriticalPoint, "phase_critical_point")]
    #[case(Gas, "phase_gas")]
    #[case(TwoPhase, "phase_twophase")]
    #[case(Unknown, "phase_unknown")]
    #[case(NotImposed, "phase_not_imposed")]
    fn as_ref(#[case] sut: Phase, #[case] expected: &str) {
        // When
        let res = sut.as_ref();

        // Then
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(vec!["phase_liquid", "liquid"], Liquid)]
    #[case(vec!["phase_supercritical", "supercritical"], Supercritical)]
    #[case(vec!["phase_supercritical_gas", "supercritical_gas", "SupercriticalGas"], SupercriticalGas)]
    #[case(vec!["phase_supercritical_liquid", "supercritical_liquid", "SupercriticalLiquid"], SupercriticalLiquid)]
    #[case(vec!["phase_critical_point", "critical_point", "CriticalPoint"], CriticalPoint)]
    #[case(vec!["phase_gas", "gas"], Gas)]
    #[case(vec!["phase_twophase", "phase_two_phase", "two_phase", "TwoPhase"], TwoPhase)]
    #[case(vec!["phase_unknown", "unknown"], Unknown)]
    #[case(vec!["phase_not_imposed", "not_imposed", "NotImposed"], NotImposed)]
    fn from_valid_str(#[case] valid: Vec<&str>, #[case] expected: Phase) {
        for s in valid {
            // When
            let res1 = Phase::from_str(s).unwrap();
            let res2 = Phase::try_from(s).unwrap();

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
        let res1 = Phase::from_str(invalid);
        let res2 = Phase::try_from(invalid);

        // Then
        assert!(res1.is_err());
        assert!(res2.is_err());
    }

    #[rstest]
    #[case(Liquid, 0)]
    #[case(Supercritical, 1)]
    #[case(SupercriticalGas, 2)]
    #[case(SupercriticalLiquid, 3)]
    #[case(CriticalPoint, 4)]
    #[case(Gas, 5)]
    #[case(TwoPhase, 6)]
    #[case(Unknown, 7)]
    #[case(NotImposed, 8)]
    fn into_u8(#[case] sut: Phase, #[case] expected: u8) {
        // When
        let res: u8 = sut.into();

        // Then
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(0, Liquid)]
    #[case(1, Supercritical)]
    #[case(2, SupercriticalGas)]
    #[case(3, SupercriticalLiquid)]
    #[case(4, CriticalPoint)]
    #[case(5, Gas)]
    #[case(6, TwoPhase)]
    #[case(7, Unknown)]
    #[case(8, NotImposed)]
    fn try_from_valid_u8_or_f64(#[case] valid: u8, #[case] expected: Phase) {
        // When
        let res1 = Phase::try_from(valid).unwrap();
        let res2 = Phase::try_from(f64::from(valid)).unwrap();

        // Then
        assert_eq!(res1, expected);
        assert_eq!(res2, expected);
    }

    #[rstest]
    #[case(254)]
    #[case(255)]
    fn try_from_invalid_u8(#[case] invalid: u8) {
        // When
        let res = Phase::try_from(invalid);

        // Then
        assert!(res.is_err());
    }

    #[rstest]
    #[case(-1.0)]
    #[case(255.0)]
    #[case(100e3)]
    fn try_from_invalid_f64(#[case] invalid: f64) {
        // When
        let res = Phase::try_from(invalid);

        // Then
        assert!(res.is_err());
    }
}
