use crate::errors::EnumParseError;
use std::str::FromStr;

/// CoolProp humid air input/output parameters.
///
/// # Examples
///
/// How to convert [`HumidAirParameter`] into [`&str`](str):
///
/// ```
/// use fluids_rs::enums::HumidAirParameter;
///
/// let result = HumidAirParameter::TDew.as_ref();
/// assert_eq!(result, "D");
/// ```
///
/// How to parse [`HumidAirParameter`] from [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use fluids_rs::enums::HumidAirParameter;
///
/// let result = HumidAirParameter::from_str("D").unwrap();
/// assert_eq!(result, HumidAirParameter::TDew);
///
/// // or
///
/// let result = HumidAirParameter::try_from("Tdp").unwrap();
/// assert_eq!(result, HumidAirParameter::TDew);
/// ```
/// # See also
///
/// - [CoolProp humid air input/output parameters](https://coolprop.github.io/CoolProp/fluid_properties/HumidAir.html#table-of-inputs-outputs-to-hapropssi)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HumidAirParameter {
    /// Wet-bulb temperature _(K)_.
    TWetBulb,

    /// Specific heat at constant pressure per unit of dry air _(J/kg dry air/K)_.
    Cp,

    /// Specific heat at constant pressure per unit of humid air _(J/kg humid air/K)_.
    CpHA,

    /// Specific heat at constant volume per unit of dry air _(J/kg dry air/K)_.
    Cv,

    /// Specific heat at constant volume per unit of humid air _(J/kg humid air/K)_.
    CvHA,

    /// Dew-point temperature _(K)_.
    TDew,

    /// Specific enthalpy per unit of dry air _(J/kg dry air)_.
    H,

    /// Specific enthalpy per unit of humid air _(J/kg humid air)_.
    Hha,

    /// Thermal conductivity _(W/m/K)_.
    Conductivity,

    /// Dynamic viscosity _(Pa*s)_.
    DynamicViscosity,

    /// Water mole fraction _(mol water/mol humid air)_.
    PsiW,

    /// Pressure _(Pa)_.
    P,

    /// Partial pressure of water vapor _(Pa)_.
    Pw,

    /// Relative humidity _(dimensionless, from 0 to 1)_.
    R,

    /// Specific entropy per unit of dry air _(J/kg dry air/K)_.
    S,

    /// Specific entropy per unit of humid air _(J/kg humid air/K)_.
    SHA,

    /// Dry-bulb temperature _(K)_.
    T,

    /// Specific volume per unit of dry air _(m3/kg dry air)_.
    V,

    /// Specific volume per unit of humid air _(m3/kg humid air)_.
    VHA,

    /// Humidity ratio _(kg water/kg dry air)_.
    W,

    /// Compressibility factor _(dimensionless)_.
    Z,
}

impl AsRef<str> for HumidAirParameter {
    fn as_ref(&self) -> &str {
        match self {
            HumidAirParameter::TWetBulb => "B",
            HumidAirParameter::Cp => "C",
            HumidAirParameter::CpHA => "Cha",
            HumidAirParameter::Cv => "CV",
            HumidAirParameter::CvHA => "CVha",
            HumidAirParameter::TDew => "D",
            HumidAirParameter::H => "H",
            HumidAirParameter::Hha => "Hha",
            HumidAirParameter::Conductivity => "K",
            HumidAirParameter::DynamicViscosity => "M",
            HumidAirParameter::PsiW => "psi_w",
            HumidAirParameter::P => "P",
            HumidAirParameter::Pw => "P_w",
            HumidAirParameter::R => "R",
            HumidAirParameter::S => "S",
            HumidAirParameter::SHA => "Sha",
            HumidAirParameter::T => "T",
            HumidAirParameter::V => "V",
            HumidAirParameter::VHA => "Vha",
            HumidAirParameter::W => "W",
            HumidAirParameter::Z => "Z",
        }
    }
}

impl FromStr for HumidAirParameter {
    type Err = EnumParseError;

    //noinspection SpellCheckingInspection
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "b" | "twb" | "t_wb" | "wetbulb" => Ok(HumidAirParameter::TWetBulb),
            "c" | "cp" => Ok(HumidAirParameter::Cp),
            "cha" | "cp_ha" => Ok(HumidAirParameter::CpHA),
            "cv" => Ok(HumidAirParameter::Cv),
            "cvha" | "cv_ha" => Ok(HumidAirParameter::CvHA),
            "d" | "tdp" | "dewpoint" | "t_dp" => Ok(HumidAirParameter::TDew),
            "h" | "hda" | "enthalpy" => Ok(HumidAirParameter::H),
            "hha" => Ok(HumidAirParameter::Hha),
            "k" | "conductivity" => Ok(HumidAirParameter::Conductivity),
            "m" | "visc" | "mu" => Ok(HumidAirParameter::DynamicViscosity),
            "psi_w" | "y" => Ok(HumidAirParameter::PsiW),
            "p" => Ok(HumidAirParameter::P),
            "p_w" => Ok(HumidAirParameter::Pw),
            "r" | "rh" | "relhum" => Ok(HumidAirParameter::R),
            "s" | "sda" | "entropy" => Ok(HumidAirParameter::S),
            "sha" => Ok(HumidAirParameter::SHA),
            "t" | "tdb" | "t_db" => Ok(HumidAirParameter::T),
            "v" | "vda" => Ok(HumidAirParameter::V),
            "vha" => Ok(HumidAirParameter::VHA),
            "w" | "omega" | "humrat" => Ok(HumidAirParameter::W),
            "z" => Ok(HumidAirParameter::Z),
            _ => Err(EnumParseError::new::<HumidAirParameter>(s)),
        }
    }
}

impl TryFrom<&str> for HumidAirParameter {
    type Error = EnumParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        HumidAirParameter::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(HumidAirParameter::TWetBulb, "B")]
    #[case(HumidAirParameter::Cp, "C")]
    #[case(HumidAirParameter::CpHA, "Cha")]
    #[case(HumidAirParameter::Cv, "CV")]
    #[case(HumidAirParameter::CvHA, "CVha")]
    #[case(HumidAirParameter::TDew, "D")]
    #[case(HumidAirParameter::H, "H")]
    #[case(HumidAirParameter::Hha, "Hha")]
    #[case(HumidAirParameter::Conductivity, "K")]
    #[case(HumidAirParameter::DynamicViscosity, "M")]
    #[case(HumidAirParameter::PsiW, "psi_w")]
    #[case(HumidAirParameter::P, "P")]
    #[case(HumidAirParameter::Pw, "P_w")]
    #[case(HumidAirParameter::R, "R")]
    #[case(HumidAirParameter::S, "S")]
    #[case(HumidAirParameter::SHA, "Sha")]
    #[case(HumidAirParameter::T, "T")]
    #[case(HumidAirParameter::V, "V")]
    #[case(HumidAirParameter::VHA, "Vha")]
    #[case(HumidAirParameter::W, "W")]
    #[case(HumidAirParameter::Z, "Z")]
    fn as_ref_always_returns_expected_str(
        #[case] parameter: HumidAirParameter,
        #[case] expected: &str,
    ) {
        let result = parameter.as_ref();
        assert_eq!(result, expected);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case("B", HumidAirParameter::TWetBulb)]
    #[case("Twb", HumidAirParameter::TWetBulb)]
    #[case("T_wb", HumidAirParameter::TWetBulb)]
    #[case("WetBulb", HumidAirParameter::TWetBulb)]
    #[case("C", HumidAirParameter::Cp)]
    #[case("cp", HumidAirParameter::Cp)]
    #[case("Cha", HumidAirParameter::CpHA)]
    #[case("cp_ha", HumidAirParameter::CpHA)]
    #[case("CV", HumidAirParameter::Cv)]
    #[case("CVha", HumidAirParameter::CvHA)]
    #[case("cv_ha", HumidAirParameter::CvHA)]
    #[case("D", HumidAirParameter::TDew)]
    #[case("Tdp", HumidAirParameter::TDew)]
    #[case("DewPoint", HumidAirParameter::TDew)]
    #[case("T_dp", HumidAirParameter::TDew)]
    #[case("H", HumidAirParameter::H)]
    #[case("Hda", HumidAirParameter::H)]
    #[case("Enthalpy", HumidAirParameter::H)]
    #[case("Hha", HumidAirParameter::Hha)]
    #[case("K", HumidAirParameter::Conductivity)]
    #[case("k", HumidAirParameter::Conductivity)]
    #[case("Conductivity", HumidAirParameter::Conductivity)]
    #[case("M", HumidAirParameter::DynamicViscosity)]
    #[case("Visc", HumidAirParameter::DynamicViscosity)]
    #[case("mu", HumidAirParameter::DynamicViscosity)]
    #[case("psi_w", HumidAirParameter::PsiW)]
    #[case("Y", HumidAirParameter::PsiW)]
    #[case("P", HumidAirParameter::P)]
    #[case("P_w", HumidAirParameter::Pw)]
    #[case("R", HumidAirParameter::R)]
    #[case("RH", HumidAirParameter::R)]
    #[case("RelHum", HumidAirParameter::R)]
    #[case("S", HumidAirParameter::S)]
    #[case("Sda", HumidAirParameter::S)]
    #[case("Entropy", HumidAirParameter::S)]
    #[case("Sha", HumidAirParameter::SHA)]
    #[case("T", HumidAirParameter::T)]
    #[case("Tdb", HumidAirParameter::T)]
    #[case("T_db", HumidAirParameter::T)]
    #[case("V", HumidAirParameter::V)]
    #[case("Vda", HumidAirParameter::V)]
    #[case("Vha", HumidAirParameter::VHA)]
    #[case("W", HumidAirParameter::W)]
    #[case("Omega", HumidAirParameter::W)]
    #[case("HumRat", HumidAirParameter::W)]
    #[case("Z", HumidAirParameter::Z)]
    fn from_valid_str_returns_ok(#[case] valid_value: &str, #[case] expected: HumidAirParameter) {
        let mut result = HumidAirParameter::from_str(valid_value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
        result = HumidAirParameter::try_from(valid_value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str_returns_err(#[case] invalid_value: &str) {
        let result = HumidAirParameter::from_str(invalid_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("'{}' has no matching 'HumidAirParameter'!", invalid_value)
        );
    }
}
