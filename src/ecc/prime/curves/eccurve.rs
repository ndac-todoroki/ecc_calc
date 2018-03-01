extern crate num;

use self::num::BigUint;
use super::super::ECCValue;
use ecc::prime::points::AffinePoint;

/// Implement basic curve related functions and lookups.
pub trait ECCurve {
   /// Return an copy of the curve.
   fn new() -> Self;

   /// Return the curve friendly name.
   fn name(&self) -> &str;

   /// Return the field `p` value where `E: y2 = x3 + ax + b over Fp`
   fn p(&self) -> BigUint;

   /// Return the `a` value where `E: y2 = x3 + ax + b over Fp`
   fn a(&self) -> BigUint;

   /// Return the `b` value where `E: y2 = x3 + ax + b over Fp`
   fn b(&self) -> BigUint;

   /// Return the `n` value where `E: y2 = x3 + ax + b over Fp`
   fn n(&self) -> BigUint;

   /// Return the `AffinePoint` representing the base point of the given curve.
   fn base_point(&self) -> AffinePoint;

   // /// decode "04.." "03.." "02.." into point.
   // fn decode_public_key(&self, String) -> Result<ECCValue, ParseError>
}
