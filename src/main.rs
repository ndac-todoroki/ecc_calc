#![allow(non_snake_case)]

extern crate ecc_calc as module;
extern crate num;
extern crate simple_logger;

#[cfg(test)]
#[allow(unused_qualifications)]
mod overall {
   #![allow(non_snake_case)]

   extern crate ecc_calc as module;
   extern crate num;

   use self::num::BigInt;

   extern crate simple_logger;

   use module::ecc::prime::points::affine::{AffineCoordinates, NewPoint as NewAffine};
   use module::ecc::prime::points::standard_projective::StandardProjectiveCoordinates;
   use module::ecc::prime::ECCurvePoint;
   use module::ecc::prime::curves::{ECCurve, ECCurveCalculation, Secp256k1, Secp256r1};
   use std::error::Error;

   #[test]
   fn create_affine_point() {
      let point = AffineCoordinates::try_new(
         "6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296",
         "4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5",
         16,
      ).unwrap();

      println!("Given point: {:x}", point);
   }

   #[test]
   fn check_if_point_is_on_curve() {
      let secp256r1 = Secp256r1::new();
      let secp256k1 = Secp256k1::new();

      let point = secp256r1.base_point(); // AffineCoordinates

      assert!(secp256r1.point_is_on_curve(&point) == true);
      assert!(secp256k1.point_is_on_curve(&point) == false);
   }

   #[test]
   fn point_convertion_to_StandardProjectiveCoordinates() {
      let secp256r1 = Secp256r1::new();
      let point = secp256r1.base_point();

      secp256r1
         .convert_point_to::<StandardProjectiveCoordinates>(&point)
         .unwrap();
   }

   #[test]
   fn baseG_multipy2_on_Secp256k1() {
      let curve = Secp256k1::new();
      println!("\nG, 2G test 2 on {}", curve.name());
      let point_G = curve.convert_point_to::<StandardProjectiveCoordinates>(&curve.base_point());
      println!("G={:x}", &curve.base_point());
      match point_G {
         Ok(point) => {
            println!("G= {:x}", point);
            let point_2G = curve.multipy_point(&point, BigInt::from(2));
            println!("2G= {:064x}", point_2G);
            println!(
               "2G= {:064x}",
               curve
                  .convert_point_to::<AffineCoordinates>(&point_2G)
                  .unwrap()
            )
         },
         _ => (),
      }
   }

   #[test]
   fn baseG_multipy2_on_Secp256r1() {
      let curve = Secp256r1::new();
      println!("\nG, 2G test 2 on {}", curve.name());
      let point_G = curve.convert_point_to::<StandardProjectiveCoordinates>(&curve.base_point());
      println!("G={:x}", &curve.base_point());
      match point_G {
         Ok(point) => {
            println!("G= {:x}", point);
            let point_2G = curve.multipy_point(&point, BigInt::from(2));
            println!("2G= {:064x}", point_2G);
            println!(
               "2G= {:064x}",
               curve
                  .convert_point_to::<AffineCoordinates>(&point_2G)
                  .unwrap()
            )
         },
         _ => (),
      }
   }

   #[test]
   #[should_panic]
   fn baseG_multipyn_on_Secp256k1() {
      let curve = Secp256k1::new();
      println!("\nn * G should be inf test on {}", curve.name());
      let point_G = curve.convert_point_to::<StandardProjectiveCoordinates>(&curve.base_point());
      let n = curve.n();
      match point_G {
         Ok(point) => {
            let point_R = curve.multipy_point(&point, n);

            println!("R= {:x}", point_R);
            println!(
               "R= {:x}",
               curve
                  .convert_point_to::<AffineCoordinates>(&point_R)
                  .unwrap()
            );
         },
         _ => (),
      }
   }

   #[test]
   #[should_panic]
   fn baseG_multipyn_on_Secp256r1() {
      let curve = Secp256r1::new();
      println!("\nn * G should be inf test on {}", curve.name());
      let point_G = curve.convert_point_to::<StandardProjectiveCoordinates>(&curve.base_point());
      let n = curve.n();
      match point_G {
         Ok(point) => {
            let point_R = curve.multipy_point(&point, n);

            println!("R= {:x}", point_R);
            println!(
               "R= {:x}",
               curve
                  .convert_point_to::<AffineCoordinates>(&point_R)
                  .unwrap()
            );
         },
         _ => (),
      }
   }
}
