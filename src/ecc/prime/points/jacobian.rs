extern crate num;

use self::num::{pow, BigInt, Integer, Num, One, ToPrimitive, Zero};
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
   fn is_point_at_infinity(&self) -> bool { self.z.is_zero() }
}

impl<Curve> PointCalculation<Curve> for JacobianPoint
where
   Curve: ECCurve,
{
   /// Returns a function that takes a curve and return the result point.
   fn point_addition(curve: &Curve, former: &Self, latter: &Self) -> Self {
      // fast return
      if former.is_point_at_infinity() {
         return JacobianPoint::from(latter);
      }
      if latter.is_point_at_infinity() {
         return JacobianPoint::from(former);
      }

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

   fn point_subtraction(curve: &Curve, former: &Self, latter: &Self) -> Self {
      let latter = Self {
         x: latter.x.clone(),
         y: latter.y.clone() * -1,
         z: latter.z.clone(),
      };
      Self::point_addition(curve, former, &latter)
   }

   #[allow(non_snake_case)]
   fn point_doublation(curve: &Curve, point: &Self) -> Self {
      if point.y.is_zero() || point.z.is_zero() {
         return JacobianPoint::from(ECCValue::Infinity);
      }

      let A = pow(point.y.clone(), 2).mod_floor(&curve.p());
      let B = (BigInt::from(4) * point.x.clone() * A.clone()).mod_floor(&curve.p());
      let C = (BigInt::from(8) * pow(A.clone(), 2)).mod_floor(&curve.p());
      let D = (BigInt::from(3) * pow(point.x.clone(), 2) + curve.a() * pow(point.z.clone(), 4))
         .mod_floor(&curve.p());

      info!("** Point Doubling!");
      debug!("\n * A: {}, \n * B: {}, \n * C: {}, \n * D: {}", A, B, C, D);

      let x = (pow(D.clone(), 2) - BigInt::from(2) * B.clone()).mod_floor(&curve.p());
      let y = (D * (B - x.clone()) - C).mod_floor(&curve.p());
      let z = (BigInt::from(4) * point.y.clone() * point.z.clone()).mod_floor(&curve.p());

      return JacobianPoint { x, y, z };
   }

   #[allow(non_snake_case)]
   fn point_multipication(curve: &Curve, point: &Self, k: BigInt) -> Self {
      /// NAF(k), Algorithm 3.30
      let NAF = |mut k: BigInt| -> Vec<i8> {
         let mut vec = Vec::new();
         while k >= BigInt::one() {
            if k.is_odd() {
               let mod4 = (k.mod_floor(&BigInt::from(4))).to_i64().unwrap();
               let ki = 2 - (mod4 as i8);
               vec.push(ki);
               k = k - ki;
            } else {
               vec.push(0_i8);
            }
            k = k / 2;
         }
         return vec;
      };

      // Algorithm 3.31
      let mut stack = NAF(k);
      debug!("\n{} {:?}", "  *  NAF(k):", stack);
      let mut Q = JacobianPoint::from(ECCValue::Infinity);
      while let Some(top) = stack.pop() {
         Q = Self::point_doublation(curve, &Q);
         match top {
            1 => Q = Self::point_addition(curve, &Q, &point),
            -1 => Q = Self::point_subtraction(curve, &Q, &point),
            _ => (),
         }
      }
      return Q;
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
         z: BigInt::one(),
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
               z: BigInt::one(),
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

impl<'a> From<&'a JacobianPoint> for JacobianPoint {
   fn from(val: &JacobianPoint) -> JacobianPoint {
      JacobianPoint {
         x: val.x.clone(),
         y: val.y.clone(),
         z: val.z.clone(),
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
