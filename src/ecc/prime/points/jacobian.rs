extern crate num;

use self::num::{pow, BigInt, Integer, Num, Zero};
use self::num::bigint::ParseBigIntError;
use super::super::curves::ECCurve;

use std::fmt;

use super::{AffinePoint, Point, PointCalculation, PointFrom, PointInto};
use super::super::super::ECCValue;

#[derive(Debug, Clone)]
/// Jacobian Coordinates are used to represent elliptic curve points on prime curves
/// `y^2 = x^3 + ax + b`.
pub struct JacobianPoint {
   pub x: BigInt,
   pub y: BigInt,
   pub z: BigInt,
}

impl JacobianPoint {
   // fn add(&self, other: &JacobianPoint) -> JacobianPoint {
   //    let u1 = self.x.clone() * pow(other.z.clone(), 2);
   //    let u2 = other.x.clone() * pow(self.z.clone(), 2);
   //    let s1 = self.y.clone() * pow(other.z.clone(), 3);
   //    let s2 = other.y.clone() * pow(self.z.clone(), 3);

   //    if u1 == u2 {
   //       if s1 != s2 {
   //          return JacobianPoint::point_at_infinity();
   //       } else {
   //          return self.double();
   //       }
   //    }

   //    let h = u1.clone() - u2.clone();
   //    let r = s2.clone() - s1.clone();
   // let x3 = pow(r.clone(), 2) - pow(h.clone(), 3) - 2_usize * u1.clone() *
   // pow(h.clone(), 2); let y3 = r.clone() * (u1 * pow(h.clone(), 2) -
   // x3.clone()) - s1.clone() * pow(h.clone(), 3); let z3 = h.clone() *
   // self.z.clone() * other.z.clone();

   //    return JacobianPoint {
   //       x: x3,
   //       y: y3,
   //       z: z3,
   //    };
   // }

   // fn double(&self) -> JacobianPoint {
   //    if self.y.is_zero() {
   //       return JacobianPoint::from(ECCValue::Infinity);
   //    }

   //    let s = 4_usize * self.x.clone() * pow(self.y.clone(), 2);
   // let m = 3_usize * pow(self.x.clone(), 2) + JacobianPoint::a() *
   // pow(self.z.clone(), 4); let x = pow(m.clone(), 2) - 2_usize *
   // s.clone();    println!("s {}, x {}", s, x);
   //    let y1 = m.clone() * (s.clone() - x.clone());
   //    let y2 = 8_usize * pow(self.y.clone(), 4);
   //    println!("y1 {}, y2 {}", y1, y2);
   //    let y = y1 - y2;
   //    let z = 2_usize * self.y.clone() * self.z.clone();

   //    return JacobianPoint { x, y, z };
   // }
}

impl<Curve> PointCalculation<Curve> for JacobianPoint
where
   Curve: ECCurve,
{
   /// Returns a function that takes a curve and return the result point.
   fn point_addition(curve: &Curve, former: &Self, latter: &Self) -> Self {
      let u1 = former.x.clone() * pow(latter.z.clone(), 2);
      let u2 = latter.x.clone() * pow(former.z.clone(), 2);
      let s1 = former.y.clone() * pow(latter.z.clone(), 3);
      let s2 = latter.y.clone() * pow(former.z.clone(), 3);

      let u1 = u1.mod_floor(&curve.p());
      let u2 = u2.mod_floor(&curve.p());
      let s1 = s1.mod_floor(&curve.p());
      let s2 = s2.mod_floor(&curve.p());

      debug!("u1: {:x}, u2: {:x}", u1, u2);
      if u1 == u2 {
         debug!("u1: {:x}, u2: {:x}", s1, s2);
         if s1 != s2 {
            return JacobianPoint::from(ECCValue::Infinity);
         } else {
            return Self::point_doublation(curve, former);
         }
      }

      let h = u1.clone() - u2.clone();
      let r = s2.clone() - s1.clone();

      let x = pow(r.clone(), 2) - pow(h.clone(), 3) - 2_usize * u1.clone() * pow(h.clone(), 2);
      let x = x.mod_floor(&curve.p());

      let y = r.clone() * (u1 * pow(h.clone(), 2) - x.clone()) - s1.clone() * pow(h.clone(), 3);
      let y = y.mod_floor(&curve.p());

      let z = h.clone() * former.z.clone() * latter.z.clone();
      let z = z.mod_floor(&curve.p());

      return JacobianPoint { x, y, z };
   }

   fn point_doublation(curve: &Curve, point: &Self) -> Self {
      if point.y.is_zero() {
         return JacobianPoint::from(ECCValue::Infinity);
      }

      let s = 4_usize * point.x.clone() * pow(point.y.clone(), 2);
      let s = s.mod_floor(&curve.p());

      let m = 3_usize * pow(point.x.clone(), 2) + curve.a() * pow(point.z.clone(), 4);
      let m = m.mod_floor(&curve.p());

      let x = pow(m.clone(), 2) - 2_usize * s.clone();
      let x = x.mod_floor(&curve.p());
      debug!("s {}, x {}", s, x);

      let y1 = m.clone() * (s.clone() - x.clone());
      let y2 = 8_usize * pow(point.y.clone(), 4);
      debug!("y1 {}, y2 {}", y1, y2);
      let y = y1 - y2;
      let y = y.mod_floor(&curve.p());

      let z = 2_usize * point.y.clone() * point.z.clone();
      let z = z.mod_floor(&curve.p());

      return JacobianPoint { x, y, z };
   }
}

/* -- Formatter impls -- */
impl fmt::Display for JacobianPoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "JacobianPoint(x: {}, y: {}, z: {})",
         self.x, self.y, self.z
      )
   }
}

impl fmt::LowerHex for JacobianPoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "JacobianPoint(x: {:x}, y: {:x}, z: {:x})",
         self.x, self.y, self.z
      )
   }
}

impl fmt::UpperHex for JacobianPoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "JacobianPoint(x: {:X}, y: {:X}, z: {:X})",
         self.x, self.y, self.z
      )
   }
}

impl fmt::Octal for JacobianPoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "JacobianPoint(x: {:o}, y: {:o}, z: {:o})",
         self.x, self.y, self.z
      )
   }
}
/* -- Formatter impls -- */

impl Point for JacobianPoint {}

/* -- Point Convertion impls -- */
impl PointFrom<AffinePoint> for JacobianPoint {
   fn convert_from(point: &AffinePoint, _i: &BigInt) -> JacobianPoint {
      JacobianPoint {
         x: point.x.clone(),
         y: point.y.clone(),
         z: BigInt::from(1_u8),
      }
   }
}

impl PointFrom<JacobianPoint> for JacobianPoint {
   fn convert_from(point: &JacobianPoint, _i: &BigInt) -> JacobianPoint { point.clone() }
}

impl From<ECCValue> for JacobianPoint {
   fn from(val: ECCValue) -> JacobianPoint {
      use self::ECCValue::{Finite, Infinity};

      match val {
         Finite { x, y } => {
            JacobianPoint {
               x,
               y,
               z: BigInt::from(1_u8),
            }
         },
         Infinity => {
            JacobianPoint {
               x: BigInt::zero(),
               y: BigInt::zero(),
               z: BigInt::zero(),
            }
         },
      }
   }
}
/* -- Point Convertion impls -- */

pub trait NewPoint<T, U>
where
   Self: Sized,
{
   type Error;
   fn try_new(x_str: T, y_str: T, z_str: T, base: U) -> Result<Self, Self::Error>;
}

impl NewPoint<&'static str, u32> for JacobianPoint {
   type Error = ParseBigIntError;

   fn try_new(s1: &str, s2: &str, s3: &str, base: u32) -> Result<Self, Self::Error> {
      let x = BigInt::from_str_radix(s1, base);
      let y = BigInt::from_str_radix(s2, base);
      let z = BigInt::from_str_radix(s3, base);

      match (x, y, z) {
         (Ok(x), Ok(y), Ok(z)) => Ok(JacobianPoint { x, y, z }),
         _ => Err(ParseBigIntError::Other),
      }
   }
}

impl PartialEq for JacobianPoint {
   fn eq(&self, other: &Self) -> bool {
      let i = BigInt::zero();
      AffinePoint::convert_from(self, &i) == other.convert_into(&i)
   }
}

#[cfg(test)]
#[allow(unused_qualifications)]
mod tests {
   use super::*;
   use super::super::point::TryPointFrom;
   use super::super::jacobian_point::TryPointFrom as JacobianTry;

   #[test]
   fn inf_plus_inf_is_inf() {
      let a: AffinePoint = AffinePoint::try_from("0", "0", 16).unwrap();
      let b: AffinePoint = AffinePoint::try_from("0", "0", 16).unwrap();
      let r: AffinePoint = AffinePoint::try_from("0", "0", 16).unwrap();

      let a = JacobianPoint::from(&a);
      let b = JacobianPoint::from(&b);
      let r = JacobianPoint::from(&r);

      let z = &a + &b;

      assert!(r == z);
   }

   #[test]
   fn g_plus_inf_is_g() {
      let a: JacobianPoint = JacobianTry::try_from(
         "18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c",
         "8571ff1825885d85d2e88688dd21f3258b4ab8e4ba19e45cddf25357ce95560a",
         "00000000fffffffeffffffffffffffffffffffff000000000000000000000001",
         16,
      ).unwrap();
      let b = JacobianPoint::point_at_infinity();

      let z = &a + &b;

      assert!(a == z);
   }
}
