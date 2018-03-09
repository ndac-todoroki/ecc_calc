extern crate num;

use self::num::BigInt;
use ecc::prime::points::{AffineCoordinates, PointCalculation};
use std;

/// Implement basic curve related functions and lookups.
pub trait ECCurve {
   /// Return an copy of the curve.
   fn new() -> Self;

   /// Return the curve friendly name.
   fn name(&self) -> &str;

   /// Return the field `p` value where `E: y2 = x3 + ax + b over Fp`
   fn p(&self) -> BigInt;

   /// Return the `a` value where `E: y2 = x3 + ax + b over Fp`
   fn a(&self) -> BigInt;

   /// Return the `b` value where `E: y2 = x3 + ax + b over Fp`
   fn b(&self) -> BigInt;

   /// Return the `n` value where `E: y2 = x3 + ax + b over Fp`
   fn n(&self) -> BigInt;

   /// Return the `AffineCoordinates` representing the base point of the given
   /// curve.
   fn base_point(&self) -> AffineCoordinates;

   // /// decode "04.." "03.." "02.." into point.
   // fn decode_public_key(&self, String) -> Result<ECCValue, ParseError>
}

pub trait ECCurveCalculation<P>: ECCurve
where
   P: PointCalculation<Self>,
   Self: std::marker::Sized,
{
   fn add_points(&self, former: &P, latter: &P) -> P {
      PointCalculation::point_addition(self, former, latter)
   }

   fn subtract_points(&self, former: &P, latter: &P) -> P {
      PointCalculation::point_subtraction(self, former, latter)
   }

   fn double_point(&self, point: &P) -> P { PointCalculation::point_doublation(self, point) }

   fn multipy_point(&self, point: &P, b: BigInt) -> P {
      PointCalculation::point_multipication(self, point, b)
   }
}
