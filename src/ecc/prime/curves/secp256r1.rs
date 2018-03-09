extern crate num;

use self::num::{BigInt, Num};

use super::super::ECCurvePoint;
use super::super::points;
use self::points::Point;
use self::points::affine::{AffineCoordinates, NewPoint};
use super::super::curves::{ECCurve, ECCurveCalculation};

/// filed `p` where `E: y2 = x3 + ax + b over Fp`
const P: &str = "FFFFFFFF_00000001_00000000_00000000_00000000_FFFFFFFF_FFFFFFFF_FFFFFFFF";
/// const `a` where `E: y2 = x3 + ax + b over Fp`
const A: &str = "FFFFFFFF_00000001_00000000_00000000_00000000_FFFFFFFF_FFFFFFFF_FFFFFFFC";
/// const `b` where `E: y2 = x3 + ax + b over Fp`
const B: &str = "5AC635D8_AA3A93E7_B3EBBD55_769886BC_651D06B0_CC53B0F6_3BCE3C3E_27D2604B";
/// order `n`
const N: &str = "FFFFFFFF_00000000_FFFFFFFF_FFFFFFFF_BCE6FAAD_A7179E84_F3B9CAC2_FC632551";
///Base point `G` in uncompressed form.
#[allow(dead_code)]
const G: &str = "046B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C2964FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5";
#[allow(non_upper_case_globals)]
const Gx: &str = "6B17D1F2_E12C4247_F8BCE6E5_63A440F2_77037D81_2DEB33A0_F4A13945_D898C296";
#[allow(non_upper_case_globals)]
const Gy: &str = "4FE342E2_FE1A7F9B_8EE7EB4A_7C0F9E16_2BCE3357_6B315ECE_CBB64068_37BF51F5";

/// ## Secp256r1
/// @see http://www.secg.org/sec2-v2.pdf 2.4.2
pub struct Secp256r1 {}

impl Secp256r1 {
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
   fn base_point() -> AffineCoordinates {
      // We know this will succeed.
      AffineCoordinates::try_new(Gx, Gy, 16).unwrap()
   }
}

impl ECCurve for Secp256r1 {
   #[inline]
   fn new() -> Secp256r1 { return Secp256r1 {}; }

   #[inline]
   fn name(&self) -> &str { "Secp256r1" }

   #[inline]
   fn p(&self) -> BigInt { return Self::p(); }

   #[inline]
   fn a(&self) -> BigInt { return Self::a(); }

   #[inline]
   fn b(&self) -> BigInt { return Self::b(); }

   #[inline]
   fn n(&self) -> BigInt { return Self::n(); }

   #[inline]
   fn base_point(&self) -> AffineCoordinates { return Self::base_point(); }
}

impl<P: Point> ECCurvePoint<P> for Secp256r1 {}
// impl ECCurvePoint<point::affine::AffineCoordinates> for Secp256r1 {}
// impl ECCurvePoint<point::jacobian::JacobianCoordinates> for Secp256r1 {}

impl ECCurveCalculation<points::JacobianCoordinates> for Secp256r1 {}
impl ECCurveCalculation<points::StandardProjectiveCoordinates> for Secp256r1 {}
