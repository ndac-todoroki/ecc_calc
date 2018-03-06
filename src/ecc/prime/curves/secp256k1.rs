extern crate num;

use self::num::{BigInt, Num};

use super::super::ECCurvePoint;
use super::super::points;
use self::points::Point;
use self::points::affine::{AffinePoint, NewPoint};
use super::super::curves::{ECCurve, ECCurveCalculation};

/// filed `p` where `E: y2 = x3 + ax + b over Fp`
const P: &str = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F";
/// const `a` where `E: y2 = x3 + ax + b over Fp`
const A: &str = "00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000";
/// const `b` where `E: y2 = x3 + ax + b over Fp`
const B: &str = "00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000007";
/// order `n`
const N: &str = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE_BAAEDCE6_AF48A03B_BFD25E8C_D0364141";
///Base point `G` in uncompressed form.
#[allow(dead_code)]
const G: &str = "04_79BE667E_F9DCBBAC_55A06295_CE870B07_029BFCDB_2DCE28D9_59F2815B_16F81798_483ADA77_26A3C465_5DA4FBFC_0E1108A8_FD17B448_A6855419_9C47D08F_FB10D4B8";
#[allow(non_upper_case_globals)]
const Gx: &str = "79BE667E_F9DCBBAC_55A06295_CE870B07_029BFCDB_2DCE28D9_59F2815B_16F81798";
#[allow(non_upper_case_globals)]
const Gy: &str = "483ADA77_26A3C465_5DA4FBFC_0E1108A8_FD17B448_A6855419_9C47D08F_FB10D4B8";

/// ## Secp256k1
/// @see http://www.secg.org/sec2-v2.pdf 2.4.2
pub struct Secp256k1 {}

impl Secp256k1 {
   #[inline]
   fn p() -> BigInt {
      // We know this will succeed.
      return BigInt::from_str_radix(P, 16).unwrap();
   }

   #[inline]
   fn a() -> BigInt {
      // We know this will succeed.
      return BigInt::from_str_radix(A, 16).unwrap();
   }

   #[inline]
   fn b() -> BigInt {
      // We know this will succeed.
      return BigInt::from_str_radix(B, 16).unwrap();
   }

   #[inline]
   fn n() -> BigInt {
      // We know this will succeed.
      return BigInt::from_str_radix(N, 16).unwrap();
   }

   #[inline]
   fn base_point() -> AffinePoint {
      // We know this will succeed.
      AffinePoint::try_new(Gx, Gy, 16).unwrap()
   }
}

impl ECCurve for Secp256k1 {
   #[inline]
   fn new() -> Secp256k1 { return Secp256k1 {}; }

   #[inline]
   fn name(&self) -> &str { "Secp256k1" }

   #[inline]
   fn p(&self) -> BigInt { return Self::p(); }

   #[inline]
   fn a(&self) -> BigInt { return Self::a(); }

   #[inline]
   fn b(&self) -> BigInt { return Self::b(); }

   #[inline]
   fn n(&self) -> BigInt { return Self::n(); }

   #[inline]
   fn base_point(&self) -> AffinePoint { return Self::base_point(); }
}

impl<P: Point> ECCurvePoint<P> for Secp256k1 {}
// impl ECCurvePoint<point::affine::AffinePoint> for Secp256k1 {}
// impl ECCurvePoint<point::jacobian::JacobianPoint> for Secp256k1 {}

impl ECCurveCalculation<points::JacobianPoint> for Secp256k1 {}
impl ECCurveCalculation<points::StandardProjectivePoint> for Secp256k1 {}
