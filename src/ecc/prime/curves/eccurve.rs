extern crate num;

use self::num::{BigInt, Integer, Num};
use ecc::prime::points::{AffineCoordinates, PointCalculation};
use ecc::ECCValue;
use std;
use super::errors;

/// Implement basic curve related functions and lookups.
pub trait ECCurve {
   /// Return an copy of the curve.
   fn new() -> Self;

   /// Return the curve friendly name.
   fn name(&self) -> &str;

   /// Return the field `p` value where `E: y2 = x3 + ax + b over Fp`
   fn p(&self) -> BigInt;

   /// Return the `a` value where `E: y2 = x3 + ax + b over Fp`
   fn a(&self) -> BigInt;

   /// Return the `b` value where `E: y2 = x3 + ax + b over Fp`
   fn b(&self) -> BigInt;

   /// Return the `n` value where `E: y2 = x3 + ax + b over Fp`
   fn n(&self) -> BigInt;

   /// Return the `AffineCoordinates` representing the base point of the given
   /// curve.
   fn base_point(&self) -> AffineCoordinates;

   // /// decode "04.." "03.." "02.." into point.
   // fn decode_public_key(&self, String) -> Result<ECCValue, ParseError>

   fn parse_public_key<S: Into<String> + Copy>(
      &self,
      key: S,
   ) -> Result<ECCValue, errors::PublicKeyParseError> {
      let key_string: String = key.into();

      // `E: y2 = x3 + ax + b over Fp`
      //
      // r^2 mod m where m = 3 mod 4 (as secp256k1's p does)
      // then
      // r = +-r2^((m+1)/4) mod m
      #[allow(non_snake_case)]
      let y_calc = |x: &BigInt| {
         let THREE = BigInt::from(3_u8);
         let y2 = (x.modpow(&THREE, &self.p()) + &self.a() * x + self.b()).mod_floor(&self.p());
         // FIXME: this should vary by p's value
         let tmp: BigInt = (&self.p() + 1) / 4;
         let y = y2.modpow(&tmp, &self.p());
         return y;
      };

      match &key_string[..2] {
         "00" => Ok(ECCValue::Infinity),
         "02" => {
            let x = BigInt::from_str_radix(&key_string[2..], 16).unwrap();
            let y = y_calc(&x);
            let val = if y.is_odd() {
               let y = &self.p() - &y;
               ECCValue::Finite { x, y }
            } else {
               ECCValue::Finite { x, y }
            };
            Ok(val)
         },
         "03" => {
            let x = BigInt::from_str_radix(&key_string[2..], 16).unwrap();
            let y = y_calc(&x);
            let val = if y.is_even() {
               let y = &self.p() - &y;
               ECCValue::Finite { x, y }
            } else {
               ECCValue::Finite { x, y }
            };
            Ok(val)
         },
         "04" => {
            let x = BigInt::from_str_radix(&key_string[2..34], 16).unwrap();
            let y = BigInt::from_str_radix(&key_string[34..], 16).unwrap();
            Ok(ECCValue::Finite { x, y })
         },
         _ => Err(errors::PublicKeyParseError),
      }
   }
}

pub trait ECCurveCalculation<P>: ECCurve
where
   P: PointCalculation<Self>,
   Self: std::marker::Sized,
{
   fn add_points(&self, former: &P, latter: &P) -> P {
      PointCalculation::point_addition(self, former, latter)
   }

   fn subtract_points(&self, former: &P, latter: &P) -> P {
      PointCalculation::point_subtraction(self, former, latter)
   }

   fn double_point(&self, point: &P) -> P { PointCalculation::point_doublation(self, point) }

   fn multipy_point(&self, point: &P, b: BigInt) -> P {
      PointCalculation::point_multipication(self, point, b)
   }
}

#[test]
fn try_parse_public_key() {
   let curve = super::Secp256k1::new();
   let val = curve
      .parse_public_key("020F031CA83F3FB372BD6C2430119E0B947CF059D19CDEA98F4CEFFEF620C584F9")
      .unwrap();
   assert_eq!(val.to_uncompressed(), "040F031CA83F3FB372BD6C2430119E0B947CF059D19CDEA98F4CEFFEF620C584F9F064F1FDE4BC07D4F48C5114680AD1ADAF5F6EAA2166F7E4B4887703A681B548".to_lowercase())
}
