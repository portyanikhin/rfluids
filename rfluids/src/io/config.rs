/// `CoolProp` configuration value.
#[derive(Clone, Debug, PartialEq)]
pub enum ConfigValue {
    /// Boolean value.
    Bool(bool),
    /// Floating-point value.
    Float(f64),
    /// String value.
    String(String),
}

impl From<bool> for ConfigValue {
    fn from(value: bool) -> Self {
        ConfigValue::Bool(value)
    }
}
impl From<f64> for ConfigValue {
    fn from(value: f64) -> Self {
        ConfigValue::Float(value)
    }
}

impl From<String> for ConfigValue {
    fn from(value: String) -> Self {
        ConfigValue::String(value)
    }
}

impl From<&str> for ConfigValue {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
