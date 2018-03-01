extern crate num;
use self::num::{BigInt, Integer};
use super::infinity_error::InfinityError;

/// Value wil be defined as
/// - a point on curve
/// - infinity (not a point)
#[derive(Debug, Clone)]
pub enum ECCValue {
   Finite { x: BigInt, y: BigInt },
   Infinity,
}

/// This is supposed to replace the current `ECCValue`.
/// Calculating ECCValue::Finate everywhere is very annoying.
///
/// This enum aims to act like the built-in `Result<T>`.
/// Functions will state the `T`, so the value will be useful.
pub enum ECCValueRes<T> {
   Finite(T),
   Infinity,
}

impl ECCValue {
   pub fn to_uncompressed(&self) -> Result<String, InfinityError> {
      match self {
         &ECCValue::Finite { ref x, ref y } => Ok(format!("04{:x}{:x}", x, y)),
         &ECCValue::Infinity => Err(InfinityError),
      }
   }

   pub fn to_compressed(&self) -> Result<String, InfinityError> {
      match self {
         &ECCValue::Finite { ref x, ref y } => {
            Ok(if y.is_even() {
               format!("02{:x}", x)
            } else {
               format!("03{:x}", x)
            })
         },
         &ECCValue::Infinity => Err(InfinityError),
      }
   }
}

// impl<S> From<S> for ECCValue
// where
//    S: Into<String> + Copy,
// {
//    fn from(s: S) -> Self {
//       let ls = s.into().to_lowercase();
//       if ls == "inf" || ls == "infinity" {
//          ECCValue::Infinity
//       } else {
//          let first2bits = &(s.into()[..2]);
//          match first2bits {
//             "02" => ECCValue::Infinity,
//             "03" => ECCValue::Infinity,
//             "04" => ECCValue::Infinity,
// _ => panic!("should be inf or 02.. or 03.. or 04.., but wasn't
// any of them"),          }
//       }
//    }
// }
