extern crate num;

use self::num::bigint::ParseBigIntError;
use self::num::{BigInt, Integer, Num, One, ToPrimitive, Zero};
use super::super::curves::ECCurve;

use std::fmt;

use super::super::super::ECCValue;
use super::{AffineCoordinates, Point, PointCalculation, PointFrom, PointInto};

#[derive(Debug, Clone)]
/// Jacobian Coordinates are used to represent elliptic curve points on prime curves
/// `y^2 = x^3 + ax + b`.
pub struct JacobianCoordinates {
   pub x: BigInt,
   pub y: BigInt,
   pub z: BigInt,
}

impl JacobianCoordinates {
   fn is_point_at_infinity(&self) -> bool {
      self.z.is_zero()
   }
}

#[allow(non_snake_case)]
impl<Curve> PointCalculation<Curve> for JacobianCoordinates
where
   Curve: ECCurve,
{
   /// Returns a function that takes a curve and return the result point.
   fn point_addition(curve: &Curve, former: &Self, latter: &Self) -> Self {
      // fast return
      if former.is_point_at_infinity() {
         return JacobianCoordinates::from(latter);
      }
      if latter.is_point_at_infinity() {
         return JacobianCoordinates::from(former);
      }

      let TWO = BigInt::from(2_u8);
      let THREE = BigInt::from(3_u8);

      let u1 = &former.x * latter.z.modpow(&TWO, &curve.p());
      let u2 = &latter.x * former.z.modpow(&TWO, &curve.p());
      let s1 = &former.y * latter.z.modpow(&THREE, &curve.p());
      let s2 = &latter.y * former.z.modpow(&THREE, &curve.p());
      // let u1 = former.x.clone() * pow(latter.z.clone(), 2);
      // let u2 = latter.x.clone() * pow(former.z.clone(), 2);
      // let s1 = former.y.clone() * pow(latter.z.clone(), 3);
      // let s2 = latter.y.clone() * pow(former.z.clone(), 3);

      let u1 = u1.mod_floor(&curve.p());
      let u2 = u2.mod_floor(&curve.p());
      let s1 = s1.mod_floor(&curve.p());
      let s2 = s2.mod_floor(&curve.p());

      debug!("u1: {:x}, u2: {:x}", u1, u2);
      if u1 == u2 {
         debug!("s1: {:x}, s2: {:x}", s1, s2);
         if s1 != s2 {
            return JacobianCoordinates::from(ECCValue::Infinity);
         } else {
            return Self::point_doublation(curve, former);
         }
      }

      if latter.z.is_one() {
         info!("** Point Mixed Addition!");
         let A = former.z.modpow(&TWO, &curve.p());
         let B = (&former.z * &A).mod_floor(&curve.p());
         let C = (&latter.x * &A).mod_floor(&curve.p());
         let D = (&latter.y * &B).mod_floor(&curve.p());
         let E = (&C - &former.x).mod_floor(&curve.p());
         let F = (&D - &former.y).mod_floor(&curve.p());
         let G = E.modpow(&TWO, &curve.p());
         let H = (&G * &E).mod_floor(&curve.p());
         let I = (&former.x * &G).mod_floor(&curve.p());
         let x = (F.modpow(&TWO, &curve.p()) - (&H + &TWO * &I)).mod_floor(&curve.p());
         let y = (&F * (I - &x) - &former.y * &H).mod_floor(&curve.p());
         let z = (&former.z * E).mod_floor(&curve.p());

         return JacobianCoordinates { x, y, z };
      } else {
         info!("** Point Addition!");

         let h = (&u1 - &u2).mod_floor(&curve.p());
         let r = (&s2 - &s1).mod_floor(&curve.p());

         let x = r.modpow(&TWO, &curve.p())
            - h.modpow(&THREE, &curve.p())
            - 2_usize * &u1 * h.modpow(&TWO, &curve.p());
         let x = x.mod_floor(&curve.p());

         let y = &r * (u1 * h.modpow(&TWO, &curve.p()) - &x) - &s1 * h.modpow(&THREE, &curve.p());
         let y = y.mod_floor(&curve.p());

         let z = &h * &former.z * &latter.z;
         let z = z.mod_floor(&curve.p());

         return JacobianCoordinates { x, y, z };
      }
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
      if point.is_point_at_infinity() {
         // point.y.is_zero() || point.z.is_zero() {
         return JacobianCoordinates::from(ECCValue::Infinity);
      }

      let TWO = BigInt::from(2_u8);
      let FOUR = BigInt::from(4_u8);

      let A = point.y.modpow(&TWO, &curve.p());
      let B = (BigInt::from(4) * &point.x * &A).mod_floor(&curve.p());
      let C = (BigInt::from(8) * A.modpow(&TWO, &curve.p())).mod_floor(&curve.p());
      let D = (BigInt::from(3) * point.x.modpow(&TWO, &curve.p())
         + curve.a() * point.z.modpow(&FOUR, &curve.p()))
      .mod_floor(&curve.p());

      info!("** Point Doubling!");
      debug!("\n * A: {}, \n * B: {}, \n * C: {}, \n * D: {}", A, B, C, D);

      let x = (D.modpow(&TWO, &curve.p()) - BigInt::from(2) * &B).mod_floor(&curve.p());
      let y = (&D * (&B - &x) - &C).mod_floor(&curve.p());
      let z = (BigInt::from(4) * &point.y * &point.z).mod_floor(&curve.p());

      return JacobianCoordinates { x, y, z };
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
                  (-1..2).contains(&ki),
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
      let mut Q = JacobianCoordinates::from(ECCValue::Infinity);
      while let Some(top) = stack.pop() {
         debug!("\n * Q: {:x}", Q);
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
impl fmt::Display for JacobianCoordinates {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "JacobianCoordinates(x: ")?;
      self.x.fmt(f)?;
      write!(f, ", y: ")?;
      self.y.fmt(f)?;
      write!(f, ", z: ")?;
      self.z.fmt(f)?;
      write!(f, ")")?;
      Ok(())
   }
}

impl fmt::LowerHex for JacobianCoordinates {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "JacobianCoordinates(x: ")?;
      self.x.fmt(f)?;
      write!(f, ", y: ")?;
      self.y.fmt(f)?;
      write!(f, ", z: ")?;
      self.z.fmt(f)?;
      write!(f, ")")?;
      Ok(())
   }
}

impl fmt::UpperHex for JacobianCoordinates {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "JacobianCoordinates(x: ")?;
      self.x.fmt(f)?;
      write!(f, ", y: ")?;
      self.y.fmt(f)?;
      write!(f, ", z: ")?;
      self.z.fmt(f)?;
      write!(f, ")")?;
      Ok(())
   }
}

impl fmt::Octal for JacobianCoordinates {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "JacobianCoordinates(x: ")?;
      self.x.fmt(f)?;
      write!(f, ", y: ")?;
      self.y.fmt(f)?;
      write!(f, ", z: ")?;
      self.z.fmt(f)?;
      write!(f, ")")?;
      Ok(())
   }
}
/* -- Formatter impls -- */

impl Point for JacobianCoordinates {}

/* -- Point Convertion impls -- */
impl PointFrom<AffineCoordinates> for JacobianCoordinates {
   fn convert_from(point: &AffineCoordinates, _i: &BigInt) -> JacobianCoordinates {
      JacobianCoordinates {
         x: point.x.clone(),
         y: point.y.clone(),
         z: BigInt::one(),
      }
   }
}

impl PointFrom<JacobianCoordinates> for AffineCoordinates {
   fn convert_from(jacob: &JacobianCoordinates, p: &BigInt) -> AffineCoordinates {
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

      AffineCoordinates { x, y }
   }
}

impl PointFrom<JacobianCoordinates> for JacobianCoordinates {
   fn convert_from(point: &JacobianCoordinates, _i: &BigInt) -> JacobianCoordinates {
      point.clone()
   }
}

impl From<ECCValue> for JacobianCoordinates {
   fn from(val: ECCValue) -> JacobianCoordinates {
      use self::ECCValue::{Finite, Infinity};

      match val {
         Finite { x, y } => JacobianCoordinates {
            x,
            y,
            z: BigInt::one(),
         },
         Infinity => JacobianCoordinates {
            x: BigInt::one(),
            y: BigInt::one(),
            z: BigInt::zero(),
         },
      }
   }
}

impl<'a> From<&'a JacobianCoordinates> for JacobianCoordinates {
   fn from(val: &JacobianCoordinates) -> JacobianCoordinates {
      JacobianCoordinates {
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

impl NewPoint<&'static str, u32> for JacobianCoordinates {
   type Error = ParseBigIntError;

   fn try_new(s1: &str, s2: &str, s3: &str, base: u32) -> Result<Self, Self::Error> {
      let x = BigInt::from_str_radix(s1, base);
      let y = BigInt::from_str_radix(s2, base);
      let z = BigInt::from_str_radix(s3, base);

      match (x, y, z) {
         (Ok(x), Ok(y), Ok(z)) => Ok(JacobianCoordinates { x, y, z }),
         _ => Err(ParseBigIntError::Other),
      }
   }
}

impl PartialEq for JacobianCoordinates {
   fn eq(&self, other: &Self) -> bool {
      let i = BigInt::zero();
      AffineCoordinates::convert_from(self, &i) == other.convert_into(&i)
   }
}
