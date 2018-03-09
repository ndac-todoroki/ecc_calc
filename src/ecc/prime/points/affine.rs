extern crate num;

use self::num::BigInt;
use self::num::bigint::ParseBigIntError;
use self::num::Num;

use std;
use std::fmt;
use std::convert::TryFrom;

use super::{Point, PointFrom};
use super::super::super::ECCValue;

/// The `AffineCoordinates` struct represents a certain point on the elliptic curve,
/// which are also called _Affine Coordinate_ Points.
#[derive(Debug, Clone, PartialEq)]
pub struct AffineCoordinates {
   pub x: BigInt,
   pub y: BigInt,
}

impl AffineCoordinates {}

/* -- Formatter impls -- */
impl fmt::Display for AffineCoordinates {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "AffineCoordinates(x: ")?;
      self.x.fmt(f)?;
      write!(f, ", y: ")?;
      self.y.fmt(f)?;
      write!(f, ")")?;
      Ok(())
   }
}

impl fmt::LowerHex for AffineCoordinates {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "AffineCoordinates(x: ")?;
      self.x.fmt(f)?;
      write!(f, ", y: ")?;
      self.y.fmt(f)?;
      write!(f, ")")?;
      Ok(())
   }
}

impl fmt::UpperHex for AffineCoordinates {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "AffineCoordinates(x: ")?;
      self.x.fmt(f)?;
      write!(f, ", y: ")?;
      self.y.fmt(f)?;
      write!(f, ")")?;
      Ok(())
   }
}

impl fmt::Octal for AffineCoordinates {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "AffineCoordinates(x: ")?;
      self.x.fmt(f)?;
      write!(f, ", y: ")?;
      self.y.fmt(f)?;
      write!(f, ")")?;
      Ok(())
   }
}
/* -- Formatter impls -- */

impl Point for AffineCoordinates {}

/* -- Point Convertion impls -- */
impl PointFrom<AffineCoordinates> for AffineCoordinates {
   fn convert_from(point: &AffineCoordinates, _i: &BigInt) -> Self { point.clone() }
}

impl TryFrom<ECCValue> for AffineCoordinates {
   type Error = super::ConvertionError;

   fn try_from(value: ECCValue) -> Result<Self, Self::Error> {
      use self::ECCValue::{Finite, Infinity};
      match value {
         Finite { x, y } => Ok(AffineCoordinates { x, y }),
         Infinity => Err(super::ConvertionError),
      }
   }
}

impl From<AffineCoordinates> for ECCValue {
   fn from(point: AffineCoordinates) -> ECCValue {
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

impl NewPoint<&'static str, u32> for AffineCoordinates {
   type Error = ParseBigIntError;

   fn try_new(s1: &str, s2: &str, base: u32) -> Result<Self, Self::Error> {
      match BigInt::from_str_radix(s1, base) {
         Ok(u1) => {
            match BigInt::from_str_radix(s2, base) {
               Ok(u2) => Ok(AffineCoordinates { x: u1, y: u2 }),
               Err(err) => Err(err),
            }
         },
         Err(err) => Err(err),
      }
   }
}

impl NewPoint<String, u32> for AffineCoordinates {
   type Error = ParseBigIntError;

   fn try_new(s1: String, s2: String, base: u32) -> Result<Self, Self::Error> {
      match BigInt::from_str_radix(&s1, base) {
         Ok(u1) => {
            match BigInt::from_str_radix(&s2, base) {
               Ok(u2) => Ok(AffineCoordinates { x: u1, y: u2 }),
               Err(err) => Err(err),
            }
         },
         Err(err) => Err(err),
      }
   }
}
