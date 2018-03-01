extern crate num;

use self::num::BigUint;
use std::{any, fmt};

/// An infinity point (for the sake of calculation).
/// An infinity point only appears within ECCuves.
#[derive(Debug, Clone, Copy)]
pub struct Infinity {}

/* -- Formatter impls -- */
impl fmt::Display for Infinity {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "PointAtInfinity") }
}

impl fmt::LowerHex for Infinity {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "PointAtInfinity") }
}

impl fmt::UpperHex for Infinity {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "PointAtInfinity") }
}

impl fmt::Octal for Infinity {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "PointAtInfinity") }
}
/* -- Formatter impls -- */
