extern crate num;

use self::num::BigUint;
use self::num::bigint::ParseBigIntError;
use self::num::Num;
use self::num::Integer;

use std;
use std::fmt;
use std::convert::TryFrom;

use super::{JacobianPoint, Point, PointFrom};
use super::super::super::ECCValue;

/// The `AffinePoint` struct represents a certain point on the elliptic curve,
/// which are also called _Affine Coordinate_ Points.
#[derive(Debug, Clone, PartialEq)]
pub struct AffinePoint {
   pub x: BigUint,
   pub y: BigUint,
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
   fn convert_from(jacob: &JacobianPoint, p: &BigUint) -> AffinePoint {
      let pm2 = p.clone() - BigUint::from(2_u8);
      let inv_z: BigUint = if jacob.z.clone() == BigUint::from(0_u8) {
         panic!("Zero division!")
      } else if jacob.z.clone() == BigUint::from(1_u8) {
         BigUint::from(1_u8)
      } else {
         jacob.z.clone().modpow(&pm2, &p.clone())
      };

      AffinePoint {
         x: jacob.x.clone() * inv_z.clone() * inv_z.clone(),
         y: jacob.y.clone() * inv_z.clone() * inv_z.clone() * inv_z.clone(),
      }
   }
}

impl PointFrom<AffinePoint> for AffinePoint {
   fn convert_from(point: &AffinePoint, _i: &BigUint) -> Self { point.clone() }
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
      match BigUint::from_str_radix(s1, base) {
         Ok(u1) => {
            match BigUint::from_str_radix(s2, base) {
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
      match BigUint::from_str_radix(&s1, base) {
         Ok(u1) => {
            match BigUint::from_str_radix(&s2, base) {
               Ok(u2) => Ok(AffinePoint { x: u1, y: u2 }),
               Err(err) => Err(err),
            }
         },
         Err(err) => Err(err),
      }
   }
}
