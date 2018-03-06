extern crate ecc_calculator as module;
extern crate num;

use self::num::BigInt;

#[macro_use]
extern crate log;
extern crate simple_logger;

// test data http://point-at-infinity.org/ecc/nisttv

use module::ecc::prime::points;
use module::ecc::prime::points::affine::{AffinePoint, NewPoint as NewAffine};
use module::ecc::prime::points::jacobian::{JacobianPoint, NewPoint as NewJacobian};
use module::ecc::prime::ECCurvePoint;
use module::ecc::prime::curves::{ECCurve, ECCurveCalculation, Secp256k1, Secp256r1};
use module::ecc::ECCValue;
use std::error::Error;
use std::marker::Sized;

fn main() {
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
   };

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

   let secp256k1 = Curves::Secp256k1;
   let secp256r1 = Curves::Secp256r1;

   baseG_multipy_2_test(&secp256r1);
   baseG_multipy_n_test(&secp256r1);

   baseG_multipy_2_test(&secp256k1);
   baseG_multipy_n_test(&secp256k1);
}

enum Curves {
   Secp256k1,
   Secp256r1,
}

fn baseG_multipy_2_test(curve_enum: &Curves) {
   let curve = match curve_enum {
      &Curves::Secp256k1 => {
         let curve = Secp256k1::new();
         println!("\nG, 2G test 2 on {}", curve.name());
         let point_G = curve.convert_point_to::<JacobianPoint>(&curve.base_point());
         println!("G={:x}", &curve.base_point());
         match point_G {
            Ok(point) => {
               println!("G= {:x}", point);
               let point_2G = curve.multipy_point(&point, BigInt::from(2));
               println!("2G= {:x}", point_2G);
               println!(
                  "2G= {:x}",
                  curve.convert_point_to::<AffinePoint>(&point_2G).unwrap()
               )
            },
            _ => (),
         }
      },
      &Curves::Secp256r1 => {
         let curve = Secp256r1::new();
         println!("\nG, 2G test 2 on {}", curve.name());
         let point_G = curve.convert_point_to::<JacobianPoint>(&curve.base_point());
         println!("G={:x}", &curve.base_point());
         match point_G {
            Ok(point) => {
               println!("G= {:x}", point);
               let point_2G = curve.multipy_point(&point, BigInt::from(2));
               println!("2G= {:x}", point_2G);
               println!(
                  "2G= {:x}",
                  curve.convert_point_to::<AffinePoint>(&point_2G).unwrap()
               )
            },
            _ => (),
         }
      },
   };
}

fn baseG_multipy_n_test(curve_enum: &Curves) {
   match curve_enum {
      &Curves::Secp256k1 => {
         let curve = Secp256k1::new();
         println!("\nn * G should be inf test on {}", curve.name());
         let point_G = curve.convert_point_to::<JacobianPoint>(&curve.base_point());
         let n = curve.n();
         match point_G {
            Ok(point) => {
               let point_R = curve.multipy_point(&point, n);

               println!("R= {:x}", point_R);
               println!(
                  "R= {:x}",
                  curve.convert_point_to::<AffinePoint>(&point_R).unwrap()
               );
            },
            _ => (),
         }
      },
      &Curves::Secp256r1 => {
         let curve = Secp256r1::new();
         println!("\nn * G should be inf test on {}", curve.name());
         let point_G = curve.convert_point_to::<JacobianPoint>(&curve.base_point());
         let n = curve.n();
         match point_G {
            Ok(point) => {
               let point_R = curve.multipy_point(&point, n);

               println!("R= {:x}", point_R);
               println!(
                  "R= {:x}",
                  curve.convert_point_to::<AffinePoint>(&point_R).unwrap()
               );
            },
            _ => (),
         }
      },
   };
}
