use std::fmt::Debug;

/// Thermodynamic state variant.
pub trait StateVariant: Debug {}

impl StateVariant for Defined {}

impl StateVariant for Undefined {}

/// A marker that determines the _presence_ of a defined thermodynamic state.
#[derive(Debug)]
pub struct Defined;

/// A marker that determines the _absence_ of a defined thermodynamic state.
#[derive(Debug)]
pub struct Undefined;
