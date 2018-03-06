extern crate num;

use self::num::{BigInt, Integer};
use self::curves::ECCurve;

pub mod curves;
pub mod points;

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
      } = self.convert_point_to::<AffinePoint>(point).unwrap();

      #[allow(non_snake_case)]
      let TWO = BigInt::from(2_u8);
      #[allow(non_snake_case)]
      let THREE = BigInt::from(3_u8);

      let left = point_y.modpow(&TWO, &self.p());
      let right =
         (point_x.modpow(&THREE, &self.p()) + self.a() * &point_x + self.b()).mod_floor(&self.p());

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
