extern crate ecc_calculator as module;
extern crate num;

use self::num::BigInt;

#[macro_use]
extern crate log;
extern crate simple_logger;

// test data http://point-at-infinity.org/ecc/nisttv

fn main() {
   use module::ecc::prime::points::affine::{AffinePoint, NewPoint as NewAffine};
   use module::ecc::prime::points::jacobian::{JacobianPoint, NewPoint as NewJacobian};
   use module::ecc::prime::ECCurvePoint;
   use module::ecc::prime::curves::{ECCurve, ECCurveCalculation, Secp256r1};
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

   // Jacobian addition test
   /*
   # G + G == 2*G
   a = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c, 8571ff1825885d85d2e88688dd21f3258b4ab8e4ba19e45cddf25357ce95560a, 00000000fffffffeffffffffffffffffffffffff000000000000000000000001
   b = 18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c, 8571ff1825885d85d2e88688dd21f3258b4ab8e4ba19e45cddf25357ce95560a, 00000000fffffffeffffffffffffffffffffffff000000000000000000000001
   r = f6bb32e43dcf3a3b732205038d1490d9aa6ae3c1a433827d850046d410ddd64d, 78c577510a5b8a3b19a8fb0e92042dbe152cd7cbeb236ff82f3648d361bee1a5
   */

   println!("\n{}", "G + G = 2G test");
   let point2 = JacobianPoint::try_new(
      "18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c",
      "8571ff1825885d85d2e88688dd21f3258b4ab8e4ba19e45cddf25357ce95560a",
      "00000000fffffffeffffffffffffffffffffffff000000000000000000000001",
      16,
   ).unwrap();

   /* println!("{}", "Check if point G is on curve");
   match curve.point_is_on_curve(&point2) {
      true => {
         println!(
            "{:X}",
            curve.convert_point_to::<AffinePoint>(&point2).unwrap()
         );

         let point2p2 = curve.add_points(&point2, &point2);
         println!("{:x}", point2p2);
         println!(
            "2G= {:x}",
            curve.convert_point_to::<AffinePoint>(&point2p2).unwrap()
         );

         let point_R = AffinePoint::try_new(
            "f6bb32e43dcf3a3b732205038d1490d9aa6ae3c1a433827d850046d410ddd64d",
            "78c577510a5b8a3b19a8fb0e92042dbe152cd7cbeb236ff82f3648d361bee1a5",
            16,
         ).unwrap();
         println!("\nPoint r = {:x}", point_R);
      },
      false => println!("Could not parse {:?}", point2),
   } */

   println!("\n{}", "G, 2G test");
   let point_G = curve.convert_point_to::<JacobianPoint>(&curve.base_point());
   println!("G= {:x}", &curve.base_point());
   match point_G {
      Ok(point) => {
         println!("G= {:x}", point);
         let point_2G = curve.double_point(&point);
         println!(
            "2G= {:x}",
            curve.convert_point_to::<AffinePoint>(&point_2G).unwrap()
         )
      },
      _ => (),
   }

   println!("\n{}", "G, 3G test 2");
   let point_G = curve.convert_point_to::<JacobianPoint>(&curve.base_point());
   println!("G= {:x}", &curve.base_point());
   match point_G {
      Ok(point) => {
         println!("G= {:x}", point);
         let point_2G = curve.multipy_point(&point, BigInt::from(3));
         println!("2G= {:x}", point_2G);
         println!(
            "2G= {:x}",
            curve.convert_point_to::<AffinePoint>(&point_2G).unwrap()
         )
      },
      _ => (),
   }
}
