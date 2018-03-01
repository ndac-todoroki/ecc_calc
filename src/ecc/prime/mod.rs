extern crate num;

use self::num::Integer;
use self::num::pow;
use self::curves::ECCurve;

use super::ECCValue;

pub mod curves;
pub mod points;
use self::points::{Point, PointFrom};

/// Functions for points on finite prime eccurves.
/// ### Example
/// ```
/// let jp: JacobianPoint = curve.convert_point_to<JacobianPoint>(&point);
/// ```
pub trait ECCurvePoint<P: points::Point>: ECCurve {
   fn point_is_on_curve(&self, point: &P) -> bool
   where
      points::AffinePoint: points::PointFrom<P>,
   {
      use self::points::AffinePoint;

      let AffinePoint {
         x: point_x,
         y: point_y,
      } = AffinePoint::convert_from(point, &self.p());

      let left = pow(point_y.clone(), 2).mod_floor(&self.p());
      let right =
         (pow(point_x.clone(), 3) + self.a() * point_x.clone() + self.b()).mod_floor(&self.p());

      /* -- DEBUG -- */
      info!(
         "Calculating y^2 mod p = x^3 + ax + b mod p on {} ...",
         self.name()
      );
      debug!("  LEFT:  {:x}", left);
      debug!("  RIGHT: {:x}", right);
      /* -- DEBUG -- */

      left == right
   }

   /// Type P to Type N
   fn convert_point_to<N: points::Point>(&self, point: &P) -> Result<N, points::ConvertionError>
   where
      N: points::PointFrom<P>,
   {
      Ok(N::convert_from(point, &self.a()))
   }

   // /// Needs definition
   // fn point_addition<N: point::Point>(
   //    &self,
   //    ECCValue<P>,
   //    ECCValue<P>,
   // ) -> Result<ECCValue<N>, point::ConvertionError>
   // where
   //    N: point::PointFrom<P> + TryFrom<point::Infinity>;
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
