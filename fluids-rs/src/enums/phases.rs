use crate::native::CoolPropError;
use std::str::FromStr;

/// Phase states of fluids and mixtures.
///
/// # Examples
///
/// How to convert [`Phase`] into [`&str`](str):
/// ```
/// use fluids_rs::enums::Phase;
///
/// let result = Phase::Liquid.as_ref();
/// assert_eq!(result, "phase_liquid");
/// ```
///
/// How to parse [`Phase`] from [`&str`](str):
/// ```
/// use std::str::FromStr;
/// use fluids_rs::enums::Phase;
///
/// let result = Phase::from_str("phase_liquid").unwrap();
/// assert_eq!(result, Phase::Liquid);
///
/// // or
///
/// let result = Phase::try_from("liquid").unwrap();
/// assert_eq!(result, Phase::Liquid);
/// ```
///
/// How to convert [`Phase`] into [`u8`]:
/// ```
/// use fluids_rs::enums::Phase;
///
/// let result: u8 = Phase::Gas.into();
/// assert_eq!(result, 5);
/// ```
///
/// How to parse [`Phase`] from [`u8`]:
/// ```
/// use fluids_rs::enums::Phase;
///
/// let result = Phase::try_from(5).unwrap();
/// assert_eq!(result, Phase::Gas);
/// ```
///
/// How to parse [`Phase`] from [`f64`]:
/// ```
/// use fluids_rs::enums::Phase;
///
/// let result = Phase::try_from(5.0).unwrap();
/// assert_eq!(result, Phase::Gas);
/// ```
///
/// # See also
///
/// - [Imposing the phase (optional)](https://coolprop.github.io/CoolProp/coolprop/HighLevelAPI.html#imposing-the-phase-optional)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Phase {
    /// Liquid _([`P`](crate::enums::Parameter::P) <
    /// [`PCritical`](crate::enums::Parameter::PCritical) &
    /// [`T`](crate::enums::Parameter::T) <
    /// [`TCritical`](crate::enums::Parameter::TCritical); above saturation)_.
    Liquid = 0,

    /// Supercritical fluid _([`P`](crate::enums::Parameter::P) >
    /// [`PCritical`](crate::enums::Parameter::PCritical) &
    /// [`T`](crate::enums::Parameter::T) >
    /// [`TCritical`](crate::enums::Parameter::TCritical))_.
    Supercritical = 1,

    /// Supercritical gas _([`P`](crate::enums::Parameter::P) <
    /// [`PCritical`](crate::enums::Parameter::PCritical) &
    /// [`T`](crate::enums::Parameter::T) >
    /// [`TCritical`](crate::enums::Parameter::TCritical))_.
    SupercriticalGas = 2,

    /// Supercritical liquid _([`P`](crate::enums::Parameter::P) >
    /// [`PCritical`](crate::enums::Parameter::PCritical) &
    /// [`T`](crate::enums::Parameter::T) <
    /// [`TCritical`](crate::enums::Parameter::TCritical))_.
    SupercriticalLiquid = 3,

    /// Critical point _([`P`](crate::enums::Parameter::P) =
    /// [`PCritical`](crate::enums::Parameter::PCritical) &
    /// [`T`](crate::enums::Parameter::T) =
    /// [`TCritical`](crate::enums::Parameter::TCritical))_.
    CriticalPoint = 4,

    /// Gas _([`P`](crate::enums::Parameter::P) <
    /// [`PCritical`](crate::enums::Parameter::PCritical) &
    /// [`T`](crate::enums::Parameter::T) <
    /// [`TCritical`](crate::enums::Parameter::TCritical); below saturation)_.
    Gas = 5,

    /// Two-phase fluid _([`P`](crate::enums::Parameter::P) <
    /// [`PCritical`](crate::enums::Parameter::PCritical) &
    /// [`T`](crate::enums::Parameter::T) <
    /// [`TCritical`](crate::enums::Parameter::TCritical); mixed liquid/gas)_.
    TwoPhase = 6,

    /// Unknown phase.
    Unknown = 7,

    /// CoolProp to determine phase.
    NotImposed = 8,
}

impl AsRef<str> for Phase {
    //noinspection SpellCheckingInspection
    fn as_ref(&self) -> &str {
        match self {
            Phase::Liquid => "phase_liquid",
            Phase::Supercritical => "phase_supercritical",
            Phase::SupercriticalGas => "phase_supercritical_gas",
            Phase::SupercriticalLiquid => "phase_supercritical_liquid",
            Phase::CriticalPoint => "phase_critical_point",
            Phase::Gas => "phase_gas",
            Phase::TwoPhase => "phase_twophase",
            Phase::Unknown => "phase_unknown",
            Phase::NotImposed => "phase_not_imposed",
        }
    }
}

impl FromStr for Phase {
    type Err = CoolPropError;

    //noinspection SpellCheckingInspection
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "phase_liquid" | "liquid" => Ok(Phase::Liquid),
            "phase_supercritical" | "supercritical" => Ok(Phase::Supercritical),
            "phase_supercritical_gas" | "supercritical_gas" | "supercriticalgas" => {
                Ok(Phase::SupercriticalGas)
            }
            "phase_supercritical_liquid" | "supercritical_liquid" | "supercriticalliquid" => {
                Ok(Phase::SupercriticalLiquid)
            }
            "phase_critical_point" | "critical_point" | "criticalpoint" => Ok(Phase::CriticalPoint),
            "phase_gas" | "gas" => Ok(Phase::Gas),
            "phase_twophase" | "phase_two_phase" | "two_phase" | "twophase" => Ok(Phase::TwoPhase),
            "phase_unknown" | "unknown" => Ok(Phase::Unknown),
            "phase_not_imposed" | "not_imposed" | "notimposed" => Ok(Phase::NotImposed),
            _ => Err(CoolPropError(format!(
                "'{}' has no matching phase state!",
                s
            ))),
        }
    }
}

impl TryFrom<&str> for Phase {
    type Error = CoolPropError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Phase::from_str(value)
    }
}

impl From<Phase> for u8 {
    fn from(value: Phase) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for Phase {
    type Error = CoolPropError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Phase::Liquid),
            1 => Ok(Phase::Supercritical),
            2 => Ok(Phase::SupercriticalGas),
            3 => Ok(Phase::SupercriticalLiquid),
            4 => Ok(Phase::CriticalPoint),
            5 => Ok(Phase::Gas),
            6 => Ok(Phase::TwoPhase),
            7 => Ok(Phase::Unknown),
            8 => Ok(Phase::NotImposed),
            _ => Err(CoolPropError(format!(
                "'{}' has no matching phase state!",
                value
            ))),
        }
    }
}

impl TryFrom<f64> for Phase {
    type Error = CoolPropError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let val = value.trunc();
        if val < u8::MIN as f64 || val > u8::MAX as f64 {
            return Err(CoolPropError(format!(
                "'{}' has no matching phase state!",
                val
            )));
        }
        Phase::try_from(val as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case(Phase::Liquid, "phase_liquid")]
    #[case(Phase::Supercritical, "phase_supercritical")]
    #[case(Phase::SupercriticalGas, "phase_supercritical_gas")]
    #[case(Phase::SupercriticalLiquid, "phase_supercritical_liquid")]
    #[case(Phase::CriticalPoint, "phase_critical_point")]
    #[case(Phase::Gas, "phase_gas")]
    #[case(Phase::TwoPhase, "phase_twophase")]
    #[case(Phase::Unknown, "phase_unknown")]
    #[case(Phase::NotImposed, "phase_not_imposed")]
    fn as_ref_always_returns_expected_str(#[case] phase: Phase, #[case] expected: &str) {
        let result = phase.as_ref();
        assert_eq!(result, expected);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case("phase_liquid", Phase::Liquid)]
    #[case("liquid", Phase::Liquid)]
    #[case("phase_supercritical", Phase::Supercritical)]
    #[case("supercritical", Phase::Supercritical)]
    #[case("phase_supercritical_gas", Phase::SupercriticalGas)]
    #[case("supercritical_gas", Phase::SupercriticalGas)]
    #[case("supercriticalgas", Phase::SupercriticalGas)]
    #[case("phase_supercritical_liquid", Phase::SupercriticalLiquid)]
    #[case("supercritical_liquid", Phase::SupercriticalLiquid)]
    #[case("supercriticalliquid", Phase::SupercriticalLiquid)]
    #[case("phase_critical_point", Phase::CriticalPoint)]
    #[case("critical_point", Phase::CriticalPoint)]
    #[case("criticalpoint", Phase::CriticalPoint)]
    #[case("phase_gas", Phase::Gas)]
    #[case("gas", Phase::Gas)]
    #[case("phase_twophase", Phase::TwoPhase)]
    #[case("phase_two_phase", Phase::TwoPhase)]
    #[case("two_phase", Phase::TwoPhase)]
    #[case("twophase", Phase::TwoPhase)]
    #[case("phase_unknown", Phase::Unknown)]
    #[case("unknown", Phase::Unknown)]
    #[case("phase_not_imposed", Phase::NotImposed)]
    #[case("not_imposed", Phase::NotImposed)]
    #[case("notimposed", Phase::NotImposed)]
    fn from_valid_str_returns_ok(#[case] valid_value: &str, #[case] expected: Phase) {
        let mut result = Phase::from_str(valid_value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
        result = Phase::try_from(valid_value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str_returns_err(#[case] invalid_value: &str) {
        let result = Phase::from_str(invalid_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("'{}' has no matching phase state!", invalid_value)
        );
    }

    #[rstest]
    #[case(Phase::Liquid, 0)]
    #[case(Phase::Supercritical, 1)]
    #[case(Phase::SupercriticalGas, 2)]
    #[case(Phase::SupercriticalLiquid, 3)]
    #[case(Phase::CriticalPoint, 4)]
    #[case(Phase::Gas, 5)]
    #[case(Phase::TwoPhase, 6)]
    #[case(Phase::Unknown, 7)]
    #[case(Phase::NotImposed, 8)]
    fn into_u8_always_returns_expected_value(#[case] parameter: Phase, #[case] expected: u8) {
        let result: u8 = parameter.into();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0, Phase::Liquid)]
    #[case(1, Phase::Supercritical)]
    #[case(2, Phase::SupercriticalGas)]
    #[case(3, Phase::SupercriticalLiquid)]
    #[case(4, Phase::CriticalPoint)]
    #[case(5, Phase::Gas)]
    #[case(6, Phase::TwoPhase)]
    #[case(7, Phase::Unknown)]
    #[case(8, Phase::NotImposed)]
    fn try_from_valid_u8_returns_ok(#[case] valid_value: u8, #[case] expected: Phase) {
        let result = Phase::try_from(valid_value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case(254)]
    #[case(255)]
    fn try_from_invalid_u8_returns_err(#[case] invalid_value: u8) {
        let result = Phase::try_from(invalid_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("'{}' has no matching phase state!", invalid_value)
        );
    }

    #[rstest]
    #[case(0.0, Phase::Liquid)]
    #[case(1.0, Phase::Supercritical)]
    #[case(2.0, Phase::SupercriticalGas)]
    #[case(3.0, Phase::SupercriticalLiquid)]
    #[case(4.0, Phase::CriticalPoint)]
    #[case(5.0, Phase::Gas)]
    #[case(6.0, Phase::TwoPhase)]
    #[case(7.0, Phase::Unknown)]
    #[case(8.0, Phase::NotImposed)]
    fn try_from_valid_f64_returns_ok(#[case] valid_value: f64, #[case] expected: Phase) {
        let result = Phase::try_from(valid_value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case(-1.0)]
    #[case(255.0)]
    #[case(100e3)]
    fn try_from_invalid_f64_returns_err(#[case] invalid_value: f64) {
        let result = Phase::try_from(invalid_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("'{}' has no matching phase state!", invalid_value)
        );
    }
}
