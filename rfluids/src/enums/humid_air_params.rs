use strum_macros::{AsRefStr, EnumString};

/// CoolProp humid air input/output parameters.
///
/// # Examples
///
/// How to convert [`HumidAirParam`] into [`&str`](str):
///
/// ```
/// use rfluids::enums::HumidAirParam;
///
/// assert_eq!(HumidAirParam::TDew.as_ref(), "D");
/// ```
///
/// How to parse [`HumidAirParam`] from [`&str`](str):
///
/// ```
/// use std::str::FromStr;
/// use rfluids::enums::HumidAirParam;
///
/// assert_eq!(HumidAirParam::from_str("D"), Ok(HumidAirParam::TDew));
///
/// // or
///
/// assert_eq!(HumidAirParam::try_from("Tdp"), Ok(HumidAirParam::TDew));
/// ```
/// # See also
///
/// - [CoolProp humid air input/output parameters](https://coolprop.github.io/CoolProp/fluid_properties/HumidAir.html#table-of-inputs-outputs-to-hapropssi)
//noinspection SpellCheckingInspection
#[derive(AsRefStr, EnumString, Debug, Copy, Clone, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum HumidAirParam {
    /// Wet-bulb temperature _(K)_.
    #[strum(
        to_string = "B",
        serialize = "Twb",
        serialize = "T_wb",
        serialize = "WetBulb"
    )]
    TWetBulb,

    /// Specific heat at constant pressure per unit of dry air _(J/kg dry air/K)_.
    #[strum(to_string = "C", serialize = "Cp")]
    Cp,

    /// Specific heat at constant pressure per unit of humid air _(J/kg humid air/K)_.
    #[strum(to_string = "Cha", serialize = "Cp_ha")]
    CpHA,

    /// Specific heat at constant volume per unit of dry air _(J/kg dry air/K)_.
    #[strum(to_string = "CV")]
    Cv,

    /// Specific heat at constant volume per unit of humid air _(J/kg humid air/K)_.
    #[strum(to_string = "CVha", serialize = "Cv_ha")]
    CvHA,

    /// Dew-point temperature _(K)_.
    #[strum(
        to_string = "D",
        serialize = "Tdp",
        serialize = "T_dp",
        serialize = "DewPoint"
    )]
    TDew,

    /// Specific enthalpy per unit of dry air _(J/kg dry air)_.
    #[strum(to_string = "H", serialize = "Hda", serialize = "Enthalpy")]
    H,

    /// Specific enthalpy per unit of humid air _(J/kg humid air)_.
    #[strum(to_string = "Hha")]
    Hha,

    /// Thermal conductivity _(W/m/K)_.
    #[strum(to_string = "K", serialize = "Conductivity")]
    Conductivity,

    /// Dynamic viscosity _(Pa*s)_.
    #[strum(to_string = "M", serialize = "Visc", serialize = "mu")]
    DynamicViscosity,

    /// Water mole fraction _(mol water/mol humid air)_.
    #[strum(to_string = "psi_w", serialize = "Y")]
    PsiW,

    /// Pressure _(Pa)_.
    #[strum(to_string = "P")]
    P,

    /// Partial pressure of water vapor _(Pa)_.
    #[strum(to_string = "P_w")]
    Pw,

    /// Relative humidity _(dimensionless, from 0 to 1)_.
    #[strum(to_string = "R", serialize = "RH", serialize = "RelHum")]
    R,

    /// Specific entropy per unit of dry air _(J/kg dry air/K)_.
    #[strum(to_string = "S", serialize = "Sda", serialize = "Entropy")]
    S,

    /// Specific entropy per unit of humid air _(J/kg humid air/K)_.
    #[strum(to_string = "Sha")]
    SHA,

    /// Dry-bulb temperature _(K)_.
    #[strum(to_string = "T", serialize = "Tdb", serialize = "T_db")]
    T,

    /// Specific volume per unit of dry air _(m3/kg dry air)_.
    #[strum(to_string = "V", serialize = "Vda")]
    V,

    /// Specific volume per unit of humid air _(m3/kg humid air)_.
    #[strum(to_string = "Vha")]
    VHA,

    /// Humidity ratio _(kg water/kg dry air)_.
    #[strum(to_string = "W", serialize = "Omega", serialize = "HumRat")]
    W,

    /// Compressibility factor _(dimensionless)_.
    #[strum(to_string = "Z")]
    Z,
}

#[cfg(test)]
mod tests {
    use super::HumidAirParam::*;
    use super::*;
    use rstest::*;
    use std::str::FromStr;

    #[rstest]
    #[case(TWetBulb, "B")]
    #[case(Cp, "C")]
    #[case(CpHA, "Cha")]
    #[case(Cv, "CV")]
    #[case(CvHA, "CVha")]
    #[case(TDew, "D")]
    #[case(H, "H")]
    #[case(Hha, "Hha")]
    #[case(Conductivity, "K")]
    #[case(DynamicViscosity, "M")]
    #[case(PsiW, "psi_w")]
    #[case(P, "P")]
    #[case(Pw, "P_w")]
    #[case(R, "R")]
    #[case(S, "S")]
    #[case(SHA, "Sha")]
    #[case(T, "T")]
    #[case(V, "V")]
    #[case(VHA, "Vha")]
    #[case(W, "W")]
    #[case(Z, "Z")]
    fn as_ref_always_returns_expected_str(#[case] param: HumidAirParam, #[case] expected: &str) {
        assert_eq!(param.as_ref(), expected);
    }

    //noinspection SpellCheckingInspection
    #[rstest]
    #[case(vec!["B", "Twb", "T_wb", "WetBulb"], TWetBulb)]
    #[case(vec!["C", "Cp"], Cp)]
    #[case(vec!["Cha", "Cp_ha"], CpHA)]
    #[case(vec!["CV"], Cv)]
    #[case(vec!["CVha", "Cv_ha"], CvHA)]
    #[case(vec!["D", "Tdp", "T_dp", "DewPoint"],  TDew)]
    #[case(vec!["H", "Hda", "Enthalpy"], H)]
    #[case(vec!["Hha"], Hha)]
    #[case(vec!["K", "Conductivity"], Conductivity)]
    #[case(vec!["M", "Visc", "mu"], DynamicViscosity)]
    #[case(vec!["psi_w", "Y"], PsiW)]
    #[case(vec!["P"], P)]
    #[case(vec!["P_w"], Pw)]
    #[case(vec!["R", "RH", "RelHum"], R)]
    #[case(vec!["S", "Sda", "Entropy"], S)]
    #[case(vec!["Sha"], SHA)]
    #[case(vec!["T", "Tdb", "T_db"], T)]
    #[case(vec!["V", "Vda"], V)]
    #[case(vec!["Vha"], VHA)]
    #[case(vec!["W", "Omega", "HumRat"], W)]
    #[case(vec!["Z"], Z)]
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
