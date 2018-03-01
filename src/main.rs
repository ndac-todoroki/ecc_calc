extern crate wannabe_key_generator as module;

#[macro_use]
extern crate log;
extern crate simple_logger;

// test data http://point-at-infinity.org/ecc/nisttv

fn main() {
   use module::ecc::prime::points::affine::{AffinePoint, NewPoint as NewAffine};
   use module::ecc::prime::points::jacobian::{JacobianPoint, NewPoint as NewJacobian};
   use module::ecc::prime::ECCurvePoint;
   use module::ecc::prime::curves::{ECCurve, Secp256r1};
   use module::ecc::ECCValue;
   use std::error::Error;

   simple_logger::init().unwrap();

   let point = AffinePoint::try_new(
      "6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296",
      "4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5",
      16,
   ).unwrap();

   debug!("Given point:");
   println!("{:x}", point);

   let curve = Secp256r1::new();
   // let value = ECCValue::from(point);

   println!("\n{}", "Check if point is on curve");
   match curve.point_is_on_curve(&point) {
      true => {
         println!("{:X}", point);
      },
      false => println!("Could not parse {:?}", point),
   }

   println!("\n{}", "Affine -> Jacobian");
   let jacob = curve.convert_point_to::<JacobianPoint>(&point);
   match jacob.clone() {
      Ok(value) => {
         println!("{:X}", value);
      },
      Err(err) => println!("Could not parse {:?}", err.cause()),
   }

   println!("\n{}", "Affine -> Jacobian -> Affine");
   let aff = curve.convert_point_to::<AffinePoint>(&jacob.unwrap());
   match aff {
      Ok(value) => {
         println!("{:X}", value);
      },
      Err(err) => println!("Could not parse {:?}", err.cause()),
   }

   // Jacobian

   let point2 = JacobianPoint::try_new(
      "2b11cb945c8cf152ffa4c9c2b1c965b019b35d0b7626919ef0ae6cb9d232f8af",
      "6d333da42e30f7011245b6281015ded14e0f100968e758a1b6c3c083afc14ea0",
      "0000000000000000000000000000000000000000000000000000000000000000",
      16,
   ).unwrap();
   let point3 = point2.clone();
}
