extern crate num;

use self::num::{pow, BigInt, Integer, Num, One, ToPrimitive, Zero};
use self::num::bigint::ParseBigIntError;
use super::super::curves::ECCurve;

use std::fmt;

use super::{AffinePoint, Point, PointCalculation, PointFrom, PointInto};
use super::super::super::ECCValue;

#[derive(Debug, Clone)]
/// Standard Projective Coordinates are used to represent elliptic curve points on prime curves
/// `y^2 = x^3 + ax + b` where (X, Y, Z) -> (X/Z, Y/Z).
pub struct StandardProjectivePoint {
   pub x: BigInt,
   pub y: BigInt,
   pub z: BigInt,
}

impl StandardProjectivePoint {
   fn is_point_at_infinity(&self) -> bool { self.z.is_zero() }
}

#[allow(non_snake_case)]
impl<Curve> PointCalculation<Curve> for StandardProjectivePoint
where
   Curve: ECCurve,
{
   /// Returns a function that takes a curve and return the result point.
   fn point_addition(curve: &Curve, former: &Self, latter: &Self) -> Self {
      // U1 = Y2*Z1
      // U2 = Y1*Z2
      // V1 = X2*Z1
      // V2 = X1*Z2
      // if (V1 == V2)
      // if (U1 != U2)
      //    return POINT_AT_INFINITY
      // else
      //    return POINT_DOUBLE(X1, Y1, Z1)
      // U = U1 - U2
      // V = V1 - V2
      // W = Z1*Z2
      // A = U^2*W - V^3 - 2*V^2*V2
      // X3 = V*A
      // Y3 = U*(V^2*V2 - A) - V^3*U2
      // Z3 = V^3*W
      // return (X3, Y3, Z3)

      // fast return
      if former.is_point_at_infinity() {
         return latter.clone();
      }
      if latter.is_point_at_infinity() {
         return former.clone();
      }

      let TWO = BigInt::from(2_u8);
      let THREE = BigInt::from(3_u8);

      debug!("former: {:x}", former);
      debug!("latter: {:x}", latter);

      let U1 = (&latter.y * &former.z).mod_floor(&curve.p());
      let U2 = (&former.y * &latter.z).mod_floor(&curve.p());
      let V1 = (&latter.x * &former.z).mod_floor(&curve.p());
      let V2 = (&former.x * &latter.z).mod_floor(&curve.p());

      debug!("V1: {:x}, V2: {:x}", V1, V2);
      debug!("U1: {:x}, U2: {:x}", U1, U2);
      if U1 == U2 {
         if V1 == V2 {
            return Self::point_doublation(curve, former);
         } else {
            return StandardProjectivePoint::from(ECCValue::Infinity);
         }
      }

      let U = (&U1 - &U2).mod_floor(&curve.p());
      let V = (&V1 - &V2).mod_floor(&curve.p());
      let W = (&former.z * &latter.z).mod_floor(&curve.p());
      let V_ = (V.modpow(&TWO, &curve.p()) * &V2).mod_floor(&curve.p());
      let A = (U.modpow(&TWO, &curve.p()) * &W - V.modpow(&THREE, &curve.p()) - &TWO * &V_)
         .mod_floor(&curve.p());

      debug!("\n * U: {}, \n * V: {}, \n * W: {}, \n * A: {}", U, V, W, A);

      let x = (&V * &A).mod_floor(&curve.p());
      let y = (&U * (&V_ - &A) - V.modpow(&THREE, &curve.p()) * &U2).mod_floor(&curve.p());
      let z = (V.modpow(&THREE, &curve.p()) * &W).mod_floor(&curve.p());

      return StandardProjectivePoint { x, y, z };
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
      // if (Y == 0)
      // return POINT_AT_INFINITY
      // W = a*Z^2 + 3*X^2
      // S = Y*Z
      // B = X*Y*S
      // H = W^2 - 8*B
      // X' = 2*H*S
      // Y' = W*(4*B - H) - 8*Y^2*S^2
      // Z' = 8*S^3
      // return (X', Y', Z')

      let TWO = BigInt::from(2_u8);

      if point.is_point_at_infinity() {
         return StandardProjectivePoint::from(ECCValue::Infinity);
      }

      let W: BigInt = &curve.a() * pow(point.z.clone(), 2) + 3 * pow(point.x.clone(), 2);
      let W = W.mod_floor(&curve.p());
      let S = (&point.y * &point.z).mod_floor(&curve.p());
      let B = (&point.x * &point.y * &S).mod_floor(&curve.p());
      let H: BigInt = W.modpow(&TWO, &curve.p()) - 8 * &B;
      let H = H.mod_floor(&curve.p());

      let x: BigInt = 2 * &H * &S;
      let x = x.mod_floor(&curve.p());
      let y: BigInt =
         &W * (4 * &B - &H) - 8 * point.y.modpow(&TWO, &curve.p()) * S.modpow(&TWO, &curve.p());
      let y = y.mod_floor(&curve.p());
      let z: BigInt = 8 * pow(S, 3);
      debug!("{}", z);
      let z = z.mod_floor(&curve.p());

      return StandardProjectivePoint { x, y, z };
   }

   #[allow(non_snake_case)]
   fn point_multipication(curve: &Curve, point: &Self, k: BigInt) -> Self {
      // NAF(k), Algorithm 3.30
      let NAF = |mut k: BigInt| -> Vec<i8> {
         let mut vec = Vec::new();
         while k >= BigInt::one() {
            if k.is_odd() {
               let mod4 = (k.mod_floor(&BigInt::from(4))).to_i64().unwrap();
               let ki = 2 - (mod4 as i8);
               assert!(
                  (-1..2).contains(ki),
                  "NAF: Unexpected Ki number error: {}",
                  ki
               );

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
      let mut Q = StandardProjectivePoint::from(ECCValue::Infinity);
      while let Some(top) = stack.pop() {
         debug!("\n * Q: {:x}", Q);
         debug!("top: {}", top);
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
impl fmt::Display for StandardProjectivePoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "StandardProjectivePoint(x: {}, y: {}, z: {})",
         self.x, self.y, self.z
      )
   }
}

impl fmt::LowerHex for StandardProjectivePoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "StandardProjectivePoint(x: {:x}, y: {:x}, z: {:x})",
         self.x, self.y, self.z
      )
   }
}

impl fmt::UpperHex for StandardProjectivePoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "StandardProjectivePoint(x: {:X}, y: {:X}, z: {:X})",
         self.x, self.y, self.z
      )
   }
}

impl fmt::Octal for StandardProjectivePoint {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "StandardProjectivePoint(x: {:o}, y: {:o}, z: {:o})",
         self.x, self.y, self.z
      )
   }
}
/* -- Formatter impls -- */

impl Point for StandardProjectivePoint {}

/* -- Point Convertion impls -- */
impl PointFrom<AffinePoint> for StandardProjectivePoint {
   fn convert_from(point: &AffinePoint, _i: &BigInt) -> StandardProjectivePoint {
      StandardProjectivePoint {
         x: point.x.clone(),
         y: point.y.clone(),
         z: BigInt::one(),
      }
   }
}

impl PointFrom<StandardProjectivePoint> for StandardProjectivePoint {
   fn convert_from(point: &StandardProjectivePoint, _i: &BigInt) -> StandardProjectivePoint {
      point.clone()
   }
}

impl PointFrom<StandardProjectivePoint> for AffinePoint {
   fn convert_from(jacob: &StandardProjectivePoint, p: &BigInt) -> AffinePoint {
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

      let inv_z = inv_Zn_over_p(&jacob.z, 1, p);

      let x = (&jacob.x * &inv_z).mod_floor(p);
      let y = (&jacob.y * &inv_z).mod_floor(p);

      AffinePoint { x, y }
   }
}

impl From<ECCValue> for StandardProjectivePoint {
   fn from(val: ECCValue) -> StandardProjectivePoint {
      use self::ECCValue::{Finite, Infinity};

      match val {
         Finite { x, y } => {
            StandardProjectivePoint {
               x,
               y,
               z: BigInt::one(),
            }
         },
         Infinity => {
            StandardProjectivePoint {
               x: BigInt::zero(),
               y: BigInt::one(),
               z: BigInt::zero(),
            }
         },
      }
   }
}

impl<'a> From<&'a StandardProjectivePoint> for StandardProjectivePoint {
   fn from(val: &StandardProjectivePoint) -> StandardProjectivePoint {
      StandardProjectivePoint {
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

impl NewPoint<&'static str, u32> for StandardProjectivePoint {
   type Error = ParseBigIntError;

   fn try_new(s1: &str, s2: &str, s3: &str, base: u32) -> Result<Self, Self::Error> {
      let x = BigInt::from_str_radix(s1, base);
      let y = BigInt::from_str_radix(s2, base);
      let z = BigInt::from_str_radix(s3, base);

      match (x, y, z) {
         (Ok(x), Ok(y), Ok(z)) => Ok(StandardProjectivePoint { x, y, z }),
         _ => Err(ParseBigIntError::Other),
      }
   }
}

impl PartialEq for StandardProjectivePoint {
   fn eq(&self, other: &Self) -> bool {
      let i = BigInt::zero();
      AffinePoint::convert_from(self, &i) == other.convert_into(&i)
   }
}
