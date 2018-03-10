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
   pub fn to_uncompressed(&self) -> String {
      match self {
         &ECCValue::Finite { ref x, ref y } => format!("04{:064x}{:064x}", x, y),
         &ECCValue::Infinity => format!("00{:064x}", 0),
      }
   }

   pub fn to_compressed(&self) -> String {
      match self {
         &ECCValue::Finite { ref x, ref y } => {
            if y.is_even() {
               format!("02{:064x}", x)
            } else {
               format!("03{:064x}", x)
            }
         },
         &ECCValue::Infinity => format!("00{:064x}", 0),
      }
   }
}
