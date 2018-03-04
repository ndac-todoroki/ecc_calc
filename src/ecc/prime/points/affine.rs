extern crate num;

use self::num::BigInt;
use self::num::bigint::ParseBigIntError;
use self::num::{Integer, Num, One, Zero};

use std;
use std::fmt;
use std::convert::TryFrom;

use super::{JacobianPoint, Point, PointFrom};
use super::super::super::ECCValue;

/// The `AffinePoint` struct represents a certain point on the elliptic curve,
/// which are also called _Affine Coordinate_ Points.
#[derive(Debug, Clone, PartialEq)]
pub struct AffinePoint {
   pub x: BigInt,
   pub y: BigInt,
}

impl AffinePoint {}

/* -- Formatter impls -- */
impl fmt::Display for AffinePoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "AffinePoint(x: {}, y: {})", self.x, self.y)
   }
}

impl fmt::LowerHex for AffinePoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "AffinePoint(x: {:x}, y: {:x})", self.x, self.y)
   }
}

impl fmt::UpperHex for AffinePoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "AffinePoint(x: {:X}, y: {:X})", self.x, self.y)
   }
}

impl fmt::Octal for AffinePoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "AffinePoint(x: {:o}, y: {:o})", self.x, self.y)
   }
}
/* -- Formatter impls -- */

impl Point for AffinePoint {}

/* -- Point Convertion impls -- */
impl PointFrom<JacobianPoint> for AffinePoint {
   fn convert_from(jacob: &JacobianPoint, p: &BigInt) -> AffinePoint {
      // fast fail
      if jacob.z.is_zero() {
         panic!("Zero division!")
      }

      #[allow(non_snake_case)]
      // Function to calculate 1/Z^n mod p as a multipication.
      let inv_Zn_over_p = |z: &BigInt, n: usize, p: &BigInt| {
         if z.is_one() {
            BigInt::one()
         } else {
            let exp = p - (n + 1);
            z.modpow(&exp, p)
         }
      };

      let inv_z2 = inv_Zn_over_p(&jacob.z, 2, p);
      let inv_z3 = inv_Zn_over_p(&jacob.z, 3, p);

      let x = (&jacob.x * &inv_z2).mod_floor(p);
      let y = (&jacob.y * &inv_z3).mod_floor(p);

      AffinePoint { x, y }
   }
}

impl PointFrom<AffinePoint> for AffinePoint {
   fn convert_from(point: &AffinePoint, _i: &BigInt) -> Self { point.clone() }
}

impl TryFrom<ECCValue> for AffinePoint {
   type Error = super::ConvertionError;

   fn try_from(value: ECCValue) -> Result<Self, Self::Error> {
      use self::ECCValue::{Finite, Infinity};
      match value {
         Finite { x, y } => Ok(AffinePoint { x, y }),
         Infinity => Err(super::ConvertionError),
      }
   }
}

impl From<AffinePoint> for ECCValue {
   fn from(point: AffinePoint) -> ECCValue {
      ECCValue::Finite {
         x: point.x.clone(),
         y: point.y.clone(),
      }
   }
}
/* -- Point Convertion impls -- */

pub trait NewPoint<T, U>
where
   Self: std::marker::Sized,
{
   type Error;
   fn try_new(x_str: T, y_str: T, base: U) -> Result<Self, Self::Error>;
}

impl NewPoint<&'static str, u32> for AffinePoint {
   type Error = ParseBigIntError;

   fn try_new(s1: &str, s2: &str, base: u32) -> Result<Self, Self::Error> {
      match BigInt::from_str_radix(s1, base) {
         Ok(u1) => {
            match BigInt::from_str_radix(s2, base) {
               Ok(u2) => Ok(AffinePoint { x: u1, y: u2 }),
               Err(err) => Err(err),
            }
         },
         Err(err) => Err(err),
      }
   }
}

impl NewPoint<String, u32> for AffinePoint {
   type Error = ParseBigIntError;

   fn try_new(s1: String, s2: String, base: u32) -> Result<Self, Self::Error> {
      match BigInt::from_str_radix(&s1, base) {
         Ok(u1) => {
            match BigInt::from_str_radix(&s2, base) {
               Ok(u2) => Ok(AffinePoint { x: u1, y: u2 }),
               Err(err) => Err(err),
            }
         },
         Err(err) => Err(err),
      }
   }
}
