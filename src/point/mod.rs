extern crate num;

use self::num::BigUint;

use std::fmt;
use std::marker::Sized;
use std::clone::Clone;

pub mod affine;
pub mod jacobian;
mod infinity;
pub use self::infinity::Infinity;

mod errors;
pub use self::errors::convertion::ConvertionError;

/// ## trait Point
/// defines basic methods points must implement.
/// Points should be displayable
pub trait Point
   : fmt::Debug + fmt::Display + fmt::LowerHex + fmt::UpperHex + Clone {
   // fn point_at_infinity() -> Self;
   // fn double() -> Self;
}

pub trait PointFrom<P: Point>: Point {
   /// `p`: FIELD
   fn convert_from(point: &P, p: &BigUint) -> Self;
}

pub trait PointInto<T: Point>: Sized + Point {
   /// Performs the conversion.
   fn convert_into(&self, p: &BigUint) -> T;
}

impl<T, U> PointInto<U> for T
where
   U: PointFrom<T>,
   T: Point,
{
   fn convert_into(&self, i: &BigUint) -> U { U::convert_from(self, &i) }
}
