extern crate wannabe_key_generator as module;

#[macro_use]
extern crate log;
extern crate simple_logger;

// test data http://point-at-infinity.org/ecc/nisttv

fn main() {
   use module::affine::{AffinePoint, NewPoint};
   use module::jacobian::JacobianPoint;
   use module::ecc::{ECCValue, ECCurve, ECCurvePoint, Secp256r1};
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
   let value = ECCValue::from(point);

   println!("\n{}", "Check if point is on curve");
   match curve.point_is_on_curve(value.clone()) {
      Ok(value) => {
         match value {
            ECCValue::Point(p) => {
               println!("{:X}", p);
            },
            ECCValue::Infinity(i) => println!("inf: {:?}", i),
         }
      },
      Err(err) => println!("Could not parse {:?}", err.cause()),
   }

   println!("\n{}", "Affine -> Jacobian");
   let jacob = curve.convert_point_to::<JacobianPoint>(value.clone());
   let aff = curve.convert_point_to::<AffinePoint>(jacob.clone().unwrap());
   match jacob {
      Ok(value) => {
         match value {
            ECCValue::Point(p) => {
               println!("{:X}", p);
            },
            ECCValue::Infinity(i) => println!("inf: {:?}", i),
         }
      },
      Err(err) => println!("Could not parse {:?}", err.cause()),
   }

   println!("\n{}", "Affine -> Jacobian -> Affine");
   match aff {
      Ok(value) => {
         match value {
            ECCValue::Point(p) => {
               println!("{:X}", p);
            },
            ECCValue::Infinity(i) => println!("inf: {:?}", i),
         }
      },
      Err(err) => println!("Could not parse {:?}", err.cause()),
   }

   // let point = Point::try_from("1234",
   // "ffffff_fffffff_fffff_fffffffffff_fffff", 16); let point2 =
   // Point::try_from("1234", "ffffffffffffffffffffffffffffffffff", 16);

   // println!("{:?}", point);
   // println!("{:?}", point2);
}
