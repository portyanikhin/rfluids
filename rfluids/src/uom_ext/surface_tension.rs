//! Surface tension (base unit newton per meter, kg · s⁻²).

use crate::uom::si::{ISQ, Quantity, SI};
use crate::uom::typenum::{N2, P1, Z0};

/// Surface tension (base unit newton per meter, kg · s⁻²).
pub type SurfaceTension = Quantity<ISQ<Z0, P1, N2, Z0, Z0, Z0, Z0>, SI<f64>, f64>;
