# ECC Calculator
This is a experimental library that do calculation on Elliptic Curves.

Do NOT take this as a production ready library.  
There are still many bugs and planned changes, and calculations are not modified to be fast.

## How to test
There some tests (but not enough). You can test them by doing `cargo test`.  
Those tests will eventially move to individual modules.

## Required Rust version
Will work with `rustc 1.25.0-nightly`. Only `nightly` versions will work.  
`rustup install nightly` if you don't have a nightly version.

## Examples

```rust
extern crate ecc_calc as module;
use module::ecc::prime::points::{AffineCoordinates, StandardProjectiveCoordinates};
use module::ecc::prime::points::affine::NewPoint;
use module::ecc::prime::ECCurvePoint;
use module::ecc::prime::curves::{ECCurve, ECCurveCalculation, Secp256k1, Secp256r1};

extern crate num;
use num::BigInt;

// Creating points from strings
let point = AffineCoordinates::try_new(
    "6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296",
    "4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5",
    16,
).unwrap();

// Printing coordinates
println!("Given point: {:x}", point);

// Instancing curves
let curve = Secp256k1::new();

// Converting points on curves and calculating with them
let point_g = curve.convert_point_to::<StandardProjectiveCoordinates>(&curve.base_point());
match point_g {
    Ok(point) => {
    let point_2g = curve.multipy_point(&point, BigInt::from(2));
    println!(
        "2G= {:064x}",
        curve
            .convert_point_to::<AffineCoordinates>(&point_2g)
            .unwrap()
    )
    },
    _ => (),  // Affine to Others will always work.
}

let point_g: AffineCoordinates = curve.base_point();
let val: ECCValue = point_g
      .unwrap()
      .into();
println!("{}", val.to_compressed());  // 03xxxxxxxxx...
```

## Contributing
Contributions are very, very welcomed. This is my first Rust program.  
There are some planned changes (maybe in the issues), but any issues/PRs are welcomed!