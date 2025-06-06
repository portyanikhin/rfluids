use super::{HumidAirParam, Input};
use thiserror::Error;

/// [`HumidAir`](crate::humid_air::HumidAir) input parameter with specified value.
///
/// # Examples
///
/// ```
/// use rfluids::prelude::*;
///
/// let pressure = HumidAirInput::pressure(101_325.0);
/// ```
pub type HumidAirInput = Input<HumidAirParam>;

impl HumidAirInput {
    /// Absolute humidity **\[kg water/kg dry air\]**.
    #[must_use]
    pub fn abs_humidity(value: f64) -> Self {
        Self {
            key: HumidAirParam::W,
            value,
        }
    }

    /// Altitude above sea level **\[m\]**.
    ///
    /// **NB.** It will be converted to the [`pressure`](Self::pressure)
    /// _(according to ASHRAE Fundamentals Handbook)_,
    /// since there is no specific [`HumidAirParam`] for this.
    ///
    /// # Errors
    ///
    /// For values out of possible range _\[-5 000; 10 000\] m_,
    /// an [`AltitudeError`] will be returned.
    pub fn altitude(value: f64) -> Result<Self, AltitudeError> {
        if !(-5_000.0..=10_000.0).contains(&value) {
            return Err(AltitudeError::OutOfRange(value));
        }
        Ok(Self::pressure(
            101_325.0 * (1.0 - 2.255_77e-5 * value).powf(5.255_9),
        ))
    }

    /// Mass density per unit of humid air **\[kg humid air/m³\]**.
    ///
    /// **NB.** It will be converted to the [`specific_volume`](Self::specific_volume),
    /// since there is no specific [`HumidAirParam`] for this.
    #[must_use]
    pub fn density(value: f64) -> Self {
        Self::specific_volume(1.0 / value)
    }

    /// Mass density per unit of dry air **\[kg dry air/m³\]**.
    ///
    /// **NB.** It will be converted to the [`specific_volume_da`](Self::specific_volume_da),
    /// since there is no specific [`HumidAirParam`] for this.
    #[must_use]
    pub fn density_da(value: f64) -> Self {
        Self::specific_volume_da(1.0 / value)
    }

    /// Dew-point temperature **\[K\]**.
    #[must_use]
    pub fn dew_temperature(value: f64) -> Self {
        Self {
            key: HumidAirParam::TDew,
            value,
        }
    }

    /// Specific enthalpy per unit of humid air **\[J/kg humid air\]**.
    #[must_use]
    pub fn enthalpy(value: f64) -> Self {
        Self {
            key: HumidAirParam::Hha,
            value,
        }
    }

    /// Specific enthalpy per unit of dry air **\[J/kg dry air\]**.
    #[must_use]
    pub fn enthalpy_da(value: f64) -> Self {
        Self {
            key: HumidAirParam::Hda,
            value,
        }
    }

    /// Specific entropy per unit of humid air **\[J/kg humid air/K\]**.
    #[must_use]
    pub fn entropy(value: f64) -> Self {
        Self {
            key: HumidAirParam::Sha,
            value,
        }
    }

    /// Specific entropy per unit of dry air **\[J/kg dry air/K\]**.
    #[must_use]
    pub fn entropy_da(value: f64) -> Self {
        Self {
            key: HumidAirParam::Sda,
            value,
        }
    }

    /// Pressure **\[Pa\]**.
    #[must_use]
    pub fn pressure(value: f64) -> Self {
        Self {
            key: HumidAirParam::P,
            value,
        }
    }

    /// Relative humidity **\[dimensionless, from 0 to 1\]**.
    #[must_use]
    pub fn rel_humidity(value: f64) -> Self {
        Self {
            key: HumidAirParam::R,
            value,
        }
    }

    /// Specific volume per unit of humid air **\[m³/kg humid air\]**.
    #[must_use]
    pub fn specific_volume(value: f64) -> Self {
        Self {
            key: HumidAirParam::Vha,
            value,
        }
    }

    /// Specific volume per unit of dry air **\[m³/kg dry air\]**.
    #[must_use]
    pub fn specific_volume_da(value: f64) -> Self {
        Self {
            key: HumidAirParam::Vda,
            value,
        }
    }

    /// Dry-bulb temperature **\[K\]**.
    #[must_use]
    pub fn temperature(value: f64) -> Self {
        Self {
            key: HumidAirParam::T,
            value,
        }
    }

    /// Water mole fraction **\[mol water/mol humid air\]**.
    #[must_use]
    pub fn water_mole_fraction(value: f64) -> Self {
        Self {
            key: HumidAirParam::PsiW,
            value,
        }
    }

    /// Partial pressure of water vapor **\[Pa\]**.
    #[must_use]
    pub fn water_partial_pressure(value: f64) -> Self {
        Self {
            key: HumidAirParam::Pw,
            value,
        }
    }

    /// Wet-bulb temperature **\[K\]**.
    #[must_use]
    pub fn wet_bulb_temperature(value: f64) -> Self {
        Self {
            key: HumidAirParam::TWetBulb,
            value,
        }
    }
}

/// Error during [`HumidAirInput::altitude`].
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AltitudeError {
    /// Altitude value is out of possible range.
    #[error("Altitude value `{0:?} m` is out of possible range [-5 000; 10 000] m!")]
    OutOfRange(f64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{assert_relative_eq, test_input};
    use rstest::*;

    test_input!(HumidAirInput: abs_humidity, key: HumidAirParam::W);

    #[rstest]
    #[case(10_000.0, 26_436.098_351_416_622)]
    #[case(0.0, 101_325.0)]
    #[case(-5_000.0, 177_687.447_332_308_8)]
    fn altitude_valid_value_returns_ok(#[case] altitude: f64, #[case] pressure: f64) {
        let sut = HumidAirInput::altitude(altitude).unwrap();
        assert_eq!(sut.key, HumidAirParam::P);
        assert_relative_eq!(sut.value, pressure);
    }

    #[rstest]
    #[case(10_000.0 + 1e-6)]
    #[case(-5_000.0 - 1e-6)]
    fn altitude_invalid_value_returns_err(#[case] invalid_value: f64) {
        let result = HumidAirInput::altitude(invalid_value);
        assert_eq!(
            result.unwrap_err(),
            AltitudeError::OutOfRange(invalid_value)
        );
    }

    test_input!(HumidAirInput: density, key: HumidAirParam::Vha, reciprocal);
    test_input!(HumidAirInput: density_da, key: HumidAirParam::Vda, reciprocal);
    test_input!(HumidAirInput: dew_temperature, key: HumidAirParam::TDew);
    test_input!(HumidAirInput: enthalpy, key: HumidAirParam::Hha);
    test_input!(HumidAirInput: enthalpy_da, key: HumidAirParam::Hda);
    test_input!(HumidAirInput: entropy, key: HumidAirParam::Sha);
    test_input!(HumidAirInput: entropy_da, key: HumidAirParam::Sda);
    test_input!(HumidAirInput: pressure, key: HumidAirParam::P);
    test_input!(HumidAirInput: rel_humidity, key: HumidAirParam::R);
    test_input!(HumidAirInput: specific_volume, key: HumidAirParam::Vha);
    test_input!(HumidAirInput: specific_volume_da, key: HumidAirParam::Vda);
    test_input!(HumidAirInput: temperature, key: HumidAirParam::T);
    test_input!(HumidAirInput: water_mole_fraction, key: HumidAirParam::PsiW);
    test_input!(HumidAirInput: water_partial_pressure, key: HumidAirParam::Pw);
    test_input!(HumidAirInput: wet_bulb_temperature, key: HumidAirParam::TWetBulb);
}
