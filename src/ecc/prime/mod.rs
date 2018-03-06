extern crate num;

use self::num::{BigInt, Integer};
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

      let b2 = BigInt::from(2_u8);
      let b3 = BigInt::from(3_u8);

      let left = point_y.modpow(&b2, &self.p());
      let right =
         (point_x.modpow(&b3, &self.p()) + self.a() * &point_x + self.b()).mod_floor(&self.p());

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
      Ok(N::convert_from(point, &self.p()))
   }
}
