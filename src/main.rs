extern crate wannabe_key_generator as module;

// test data http://point-at-infinity.org/ecc/nisttv

fn main() {
   use module::affine::{AffinePoint, NewPoint};
   use module::jacobian::JacobianPoint;
   use module::ecc::{ECCValue, ECCurve, ECCurvePoint, Secp256r1};

   let point = AffinePoint::try_new(
      "6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296",
      "4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5",
      16,
   ).unwrap();

   println!("{:x}", point);

   let curve = Secp256r1::new();
   let value = point.into();

   let jp = curve.point_is_on_curve(value);

   println!("{:?}", jp)

   // let point = Point::try_from("1234",
   // "ffffff_fffffff_fffff_fffffffffff_fffff", 16); let point2 =
   // Point::try_from("1234", "ffffffffffffffffffffffffffffffffff", 16);

   // println!("{:?}", point);
   // println!("{:?}", point2);
}
