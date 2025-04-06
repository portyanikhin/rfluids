//! Compressibility coefficient (base unit 1 / pascal, m · s² · kg⁻¹).

use uom::{
    si::{ISQ, Quantity, SI},
    typenum::{N1, P1, P2, Z0},
};

/// Compressibility coefficient (base unit 1 / pascal, m · s² · kg⁻¹).
pub type CompressibilityCoefficient = Quantity<ISQ<P1, N1, P2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;
