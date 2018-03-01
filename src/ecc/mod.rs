extern crate num;

use self::num::{BigUint, Integer};
use self::num::pow;
use super::point;
use super::point::PointFrom;
use super::point::affine::AffinePoint;
use std::marker::Sized;
use std::any;
use std::convert::TryFrom;

mod secp256r1;
pub use self::secp256r1::Secp256r1;

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
}

/// Value wil be defined as
/// - a point on curve
/// - infinity (not a point)
#[derive(Debug)]
pub enum ECCValue<P: point::Point> {
   Point(P),
   Infinity(point::Infinity),
}

impl<P: point::Point> From<P> for ECCValue<P> {
   fn from(point: P) -> Self { ECCValue::Point(point) }
}

impl<P: point::Point> From<point::Infinity> for ECCValue<P> {
   fn from(inf: point::Infinity) -> Self { ECCValue::Infinity(inf) }
}

/// Functions like
/// ```
/// let jp: JacobianPoint = curve.convert_point_to<JacobianPoint>(&point);
/// ```
pub trait ECCurvePoint<P: point::Point>: ECCurve {
   fn point_is_on_curve(&self, val: ECCValue<P>) -> Result<ECCValue<P>, point::ConvertionError>
   where
      AffinePoint: point::PointFrom<P>,
   {
      match val {
         ECCValue::Infinity(_) => Err(point::ConvertionError),
         ECCValue::Point(point) => {
            let AffinePoint {
               x: point_x,
               y: point_y,
            } = AffinePoint::convert_from(&point, &self.p());

            let left = pow(point_y.clone(), 2).mod_floor(&self.p());
            let right = (pow(point_x.clone(), 3) + self.a() * point_x.clone() + self.b())
               .mod_floor(&self.p());

            /* -- DEBUG -- */
            println!(
               "Calculating y^2 mod Fp = x^3 + ax + b mod Fp on {} ...",
               self.name()
            );
            println!("  LEFT:  {:x}", left);
            println!("  RIGHT: {:x}", right);
            /* -- DEBUG -- */

            if left == right {
               Ok(ECCValue::Point(point))
            } else {
               Err(point::ConvertionError)
            }
         },
      }
   }

   /// Type P to Type N
   fn convert_point_to<N: point::Point>(
      &self,
      val: ECCValue<P>,
   ) -> Result<ECCValue<N>, point::ConvertionError>
   where
      N: point::PointFrom<P> + TryFrom<point::Infinity>,
   {
      match val {
         ECCValue::Infinity(inf) => {
            match N::try_from(inf) {
               Ok(point) => Ok(ECCValue::Point(point)),
               Err(_) => Err(point::ConvertionError),
            }
         },
         ECCValue::Point(point) => Ok(ECCValue::Point(N::convert_from(&point, &self.a()))),
      }
   }
}

// trait ECCurvePoint<T, U>
// where
//    Self: Sized,
// {
//    type Error;

//    /// Return point at the given curve, by the given `x` and `y` strings.
//    /// If the point is not on the curve, it should return a Err.
// fn try_create_point_at(&self, x_str: T, y_str: T, base: U) ->
// Result<AffinePoint, Self::Error>; }

// trait ECCurvePointChecker<P> {
//    type Error;

//    /// Check if the point is on the curve.
//    fn verify_point(&self, point: P) -> Result<AffinePoint, Self::Error>;
// }

// Errors

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ECCurvePointError;

impl fmt::Display for ECCurvePointError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Usually a point and curve mismatch.")
   }
}

impl Error for ECCurvePointError {
   fn description(&self) -> &str { "AffinePoint error on EC Curve" }

   fn cause(&self) -> Option<&Error> { None }
}
