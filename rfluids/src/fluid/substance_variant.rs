use crate::fluid::BackendName;
use crate::substance::{CustomMix, Substance};
use std::fmt::Debug;

/// CoolProp substance variant.
pub trait SubstanceVariant: BackendName + Debug + Clone {}

impl SubstanceVariant for Substance {}

impl SubstanceVariant for CustomMix {}
