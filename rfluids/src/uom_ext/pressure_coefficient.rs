//! Pressure coefficient (base unit 1 / pascal, m · s² · kg⁻¹).

use crate::uom::si::{Quantity, ISQ, SI};
use crate::uom::typenum::{N1, P1, P2, Z0};

/// Pressure coefficient (base unit 1 / pascal, m · s² · kg⁻¹).
pub type PressureCoefficient = Quantity<ISQ<P1, N1, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;
