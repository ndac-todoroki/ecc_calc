/// module about finate curves ex. Secp256r1(a.k.a. P-256).
mod infinity_error;
mod ecc_value;

pub use self::ecc_value::{ECCValue, ECCValueRes};
pub mod prime;
pub mod binary;
