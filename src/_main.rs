extern crate ring;
extern crate untrusted;

use ring::ec::suite_b::{ecdsa, ops};
use ring::test;

static CHAIN_CODE: &str = "";

fn main() {
   let priv_ops = &ops::p256::PRIVATE_KEY_OPS;

   let privkey = "E9873D79C6D87DC0FB6A5778633389F4453213303DA61F20BD67FC233AA33262";

   // priv_ops.pub2pub("", "", 1);
   let pubkey = priv_ops.priv2pub(privkey);
   println!("{}", pubkey);

   let point = ops::xy_to_jacobian_point(
      priv_ops,
      "18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c",
      "18905f76a53755c679fb732b7762251075ba95fc5fedb60179e730d418a9143c",
   );
   println!("{}", point)
}

trait KeyDerivation {
   fn pub2pub(&self, &str, &str, usize) -> &str;
   fn priv2priv(&self, &str, &str, usize) -> &str;
   fn priv2pub(&self, &str) -> String;
}

impl KeyDerivation for ops::PrivateKeyOps {
   fn pub2pub(&self, kp: &str, uuid: &str, i: usize) -> &str { "sample" }

   /// Private parent key → public child key
   /// @see https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#private-parent-key--public-child-key
   fn priv2priv(&self, kp: &str, uuid: &str, i: usize) -> &str { "sample" }

   fn priv2pub(&self, kp: &str) -> String {
      let bytes = consume_bytes(kp);
      let bytes = untrusted::Input::from(&bytes);
      let a =
         ops::scalar_parse_big_endian_variable(self.common, ops::AllowZero::Yes, bytes).unwrap();
      self.point_mul_base(&a).to_string()
   }
}

/// Returns the value of an attribute that is encoded as a sequence of an
/// even number of hex digits, or as a double-quoted UTF-8 string. The
/// empty (zero-length) value is represented as "".
pub fn consume_bytes(s: &str) -> Vec<u8> {
   if s.starts_with('\"') {
      // The value is a quoted UTF-8 string.

      let mut bytes = Vec::with_capacity(s.as_bytes().len() - 2);
      let mut s = s.as_bytes().iter().skip(1);
      loop {
         let b = match s.next() {
            Some(&b'\\') => {
               match s.next() {
                  // We don't allow all octal escape sequences, only "\0" for null.
                  Some(&b'0') => 0u8,
                  Some(&b't') => b'\t',
                  Some(&b'n') => b'\n',
                  // "\xHH"
                  Some(&b'x') => {
                     let hi = s.next().expect("Invalid hex escape sequence in string.");
                     let lo = s.next().expect("Invalid hex escape sequence in string.");
                     if let (Ok(hi), Ok(lo)) = (from_hex_digit(*hi), from_hex_digit(*lo)) {
                        (hi << 4) | lo
                     } else {
                        panic!("Invalid hex escape sequence in string.");
                     }
                  },
                  _ => {
                     panic!("Invalid hex escape sequence in string.");
                  },
               }
            },
            Some(&b'"') => {
               if s.next().is_some() {
                  panic!("characters after the closing quote of a quoted string.");
               }
               break;
            },
            Some(b) => *b,
            None => panic!("Missing terminating '\"' in string literal."),
         };
         bytes.push(b);
      }
      bytes
   } else {
      // The value is hex encoded.
      match test::from_hex(&s) {
         Ok(s) => s,
         Err(ref err_str) => {
            panic!("{} in {}", err_str, s);
         },
      }
   }
}

fn from_hex_digit(d: u8) -> Result<u8, String> {
   if d >= b'0' && d <= b'9' {
      Ok(d - b'0')
   } else if d >= b'a' && d <= b'f' {
      Ok(d - b'a' + 10u8)
   } else if d >= b'A' && d <= b'F' {
      Ok(d - b'A' + 10u8)
   } else {
      Err(format!("Invalid hex digit '{}'", d as char))
   }
}
