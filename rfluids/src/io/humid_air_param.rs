// cSpell:disable

use strum_macros::{AsRefStr, EnumString};

/// `CoolProp` humid air input/output parameters.
///
/// # Examples
///
/// Conversion between [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::io::HumidAirParam;
///
/// assert_eq!(HumidAirParam::TDew.as_ref(), "D");
/// assert_eq!(HumidAirParam::from_str("D"), Ok(HumidAirParam::TDew));
/// assert_eq!(HumidAirParam::try_from("Tdp"), Ok(HumidAirParam::TDew));
/// ```
///
/// # See also
///
/// - [CoolProp humid air input/output parameters](https://coolprop.github.io/CoolProp/fluid_properties/HumidAir.html#table-of-inputs-outputs-to-hapropssi)
#[derive(AsRefStr, Clone, Copy, Debug, EnumString, Eq, Hash, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum HumidAirParam {
    /// Wet-bulb temperature **\[K\]**.
    #[strum(
        to_string = "B",
        serialize = "Twb",
        serialize = "T_wb",
        serialize = "WetBulb",
        serialize = "TWetBulb"
    )]
    TWetBulb,

    /// Specific heat at constant pressure per unit of dry air **\[J/kg dry air/K\]**.
    #[strum(
        to_string = "C",
        serialize = "Cp",
        serialize = "Cpda",
        serialize = "Cp_da"
    )]
    Cpda,

    /// Specific heat at constant pressure per unit of humid air **\[J/kg humid air/K\]**.
    #[strum(to_string = "Cha", serialize = "Cpha", serialize = "Cp_ha")]
    Cpha,

    /// Specific heat at constant volume per unit of dry air **\[J/kg dry air/K\]**.
    #[strum(to_string = "CV", serialize = "Cvda", serialize = "Cv_da")]
    Cvda,

    /// Specific heat at constant volume per unit of humid air **\[J/kg humid air/K\]**.
    #[strum(to_string = "CVha", serialize = "Cv_ha")]
    Cvha,

    /// Dew-point temperature **\[K\]**.
    #[strum(
        to_string = "D",
        serialize = "Tdp",
        serialize = "T_dp",
        serialize = "DewPoint",
        serialize = "TDew"
    )]
    TDew,

    /// Specific enthalpy per unit of dry air **\[J/kg dry air\]**.
    #[strum(
        to_string = "H",
        serialize = "Hda",
        serialize = "H_da",
        serialize = "Enthalpy"
    )]
    Hda,

    /// Specific enthalpy per unit of humid air **\[J/kg humid air\]**.
    #[strum(to_string = "Hha", serialize = "H_ha")]
    Hha,

    /// Thermal conductivity **\[W/m/K\]**.
    #[strum(to_string = "K", serialize = "Conductivity")]
    Conductivity,

    /// Dynamic viscosity **\[Pa·s\]**.
    #[strum(
        to_string = "M",
        serialize = "Visc",
        serialize = "mu",
        serialize = "viscosity",
        serialize = "DynamicViscosity"
    )]
    DynamicViscosity,

    /// Water mole fraction **\[mol water/mol humid air\]**.
    #[strum(to_string = "psi_w", serialize = "Y", serialize = "PsiW")]
    PsiW,

    /// Pressure **\[Pa\]**.
    #[strum(to_string = "P", serialize = "Pressure")]
    P,

    /// Partial pressure of water vapor **\[Pa\]**.
    #[strum(to_string = "P_w", serialize = "Pw")]
    Pw,

    /// Relative humidity **\[dimensionless, from 0 to 1\]**.
    #[strum(to_string = "R", serialize = "RH", serialize = "RelHum")]
    R,

    /// Specific entropy per unit of dry air **\[J/kg dry air/K\]**.
    #[strum(
        to_string = "S",
        serialize = "Sda",
        serialize = "S_da",
        serialize = "Entropy"
    )]
    Sda,

    /// Specific entropy per unit of humid air **\[J/kg humid air/K\]**.
    #[strum(to_string = "Sha", serialize = "S_ha")]
    Sha,

    /// Dry-bulb temperature **\[K\]**.
    #[strum(
        to_string = "T",
        serialize = "Tdb",
        serialize = "T_db",
        serialize = "Temperature"
    )]
    T,

    /// Specific volume per unit of dry air **\[m³/kg dry air\]**.
    #[strum(to_string = "V", serialize = "Vda", serialize = "V_da")]
    Vda,

    /// Specific volume per unit of humid air **\[m³/kg humid air\]**.
    #[strum(to_string = "Vha", serialize = "V_ha")]
    Vha,

    /// Absolute humidity **\[kg water/kg dry air\]**.
    #[strum(to_string = "W", serialize = "Omega", serialize = "HumRat")]
    W,

    /// Compressibility factor **\[dimensionless\]**.
    #[strum(to_string = "Z", serialize = "Compressibility")]
    Z,
}

#[cfg(test)]
mod tests {
    use super::{HumidAirParam::*, *};
    use rstest::*;
    use std::str::FromStr;

    #[rstest]
    #[case(TWetBulb, "B")]
    #[case(Cpda, "C")]
    #[case(Cpha, "Cha")]
    #[case(Cvda, "CV")]
    #[case(Cvha, "CVha")]
    #[case(TDew, "D")]
    #[case(Hda, "H")]
    #[case(Hha, "Hha")]
    #[case(Conductivity, "K")]
    #[case(DynamicViscosity, "M")]
    #[case(PsiW, "psi_w")]
    #[case(P, "P")]
    #[case(Pw, "P_w")]
    #[case(R, "R")]
    #[case(Sda, "S")]
    #[case(Sha, "Sha")]
    #[case(T, "T")]
    #[case(Vda, "V")]
    #[case(Vha, "Vha")]
    #[case(W, "W")]
    #[case(Z, "Z")]
    fn as_ref_returns_expected_str(#[case] param: HumidAirParam, #[case] expected: &str) {
        assert_eq!(param.as_ref(), expected);
    }

    #[rstest]
    #[case(vec!["B", "Twb", "T_wb", "WetBulb", "TWetBulb"], TWetBulb)]
    #[case(vec!["C", "Cp", "Cpda", "Cp_da"], Cpda)]
    #[case(vec!["Cha", "Cpha", "Cp_ha"], Cpha)]
    #[case(vec!["CV", "Cvda", "Cv_da"], Cvda)]
    #[case(vec!["CVha", "Cv_ha"], Cvha)]
    #[case(vec!["D", "Tdp", "T_dp", "DewPoint", "TDew"],  TDew)]
    #[case(vec!["H", "Hda", "H_da", "Enthalpy"], Hda)]
    #[case(vec!["Hha", "H_ha"], Hha)]
    #[case(vec!["K", "Conductivity"], Conductivity)]
    #[case(vec!["M", "Visc", "mu", "viscosity", "DynamicViscosity"], DynamicViscosity)]
    #[case(vec!["psi_w", "Y", "PsiW"], PsiW)]
    #[case(vec!["P", "Pressure"], P)]
    #[case(vec!["P_w", "Pw"], Pw)]
    #[case(vec!["R", "RH", "RelHum"], R)]
    #[case(vec!["S", "Sda", "S_da", "Entropy"], Sda)]
    #[case(vec!["Sha", "S_ha"], Sha)]
    #[case(vec!["T", "Tdb", "T_db", "Temperature"], T)]
    #[case(vec!["V", "Vda", "V_da"], Vda)]
    #[case(vec!["Vha", "V_ha"], Vha)]
    #[case(vec!["W", "Omega", "HumRat"], W)]
    #[case(vec!["Z", "Compressibility"], Z)]
    fn from_valid_str_returns_ok(#[case] valid_values: Vec<&str>, #[case] expected: HumidAirParam) {
        for s in valid_values {
            assert_eq!(HumidAirParam::from_str(s), Ok(expected));
            assert_eq!(HumidAirParam::try_from(s), Ok(expected));
        }
    }

    #[rstest]
    #[case("")]
    #[case("Hello, World!")]
    fn from_invalid_str_returns_err(#[case] invalid_value: &str) {
        assert!(HumidAirParam::from_str(invalid_value).is_err());
        assert!(HumidAirParam::try_from(invalid_value).is_err());
    }
}
