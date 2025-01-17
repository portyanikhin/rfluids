//! CoolProp substances.

#![allow(missing_docs, non_camel_case_types)]

pub use binary_mix::*;
pub use incomp_pure::*;
pub use predefined_mix::*;
pub use pure::*;
pub use refrigerant::*;

mod binary_mix;
mod incomp_pure;
mod predefined_mix;
mod pure;
mod refrigerant;

/// CoolProp backend name.
pub trait BackendName {
    /// Returns CoolProp backend name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rfluids::substance;
    /// use rfluids::substance::BackendName;
    ///
    /// assert_eq!(substance::Pure::Water.backend_name(), "HEOS");
    /// assert_eq!(substance::IncompPure::Water.backend_name(), "INCOMP");
    /// assert_eq!(substance::Refrigerant::R32.backend_name(), "HEOS");
    /// assert_eq!(substance::PredefinedMix::TypicalNaturalGas.backend_name(), "HEOS");
    /// assert_eq!(substance::BinaryMix::MPG.backend_name(), "INCOMP");
    /// ```
    fn backend_name(&self) -> &'static str;
}
