use super::super::prime;
use std::fmt;
use super::num::BigUint;

pub trait PointCalculation {
   type Curve: prime::ECCurve;

   /// Returns a function that takes a curve and return the result point.
   fn point_addition(&Self, &Self, &Self::Curve) -> Self;
}

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

pub mod affine;
pub mod jacobian;

pub use self::affine::AffinePoint;
pub use self::jacobian::JacobianPoint;

mod errors;
pub use self::errors::convertion::ConvertionError;
