/// Extended public key
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ExtendedPubKey {
   /// The network this key is to be used on
   pub network: Network,
   /// How many derivations this key is from the master (which is 0)
   pub depth: u8,
   /// Fingerprint of the parent key
   pub parent_fingerprint: Fingerprint,
   /// Child number of the key used to derive from parent (0 for master)
   pub child_number: ChildNumber,
   /// Public key
   pub public_key: PublicKey,
   /// Chain code
   pub chain_code: ChainCode,
}

/// A child number for a derived key
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ChildNumber {
   /// Hardened key index, within [0, 2^31 - 1]
   Hardened(u32),
   /// Non-hardened key, within [0, 2^31 - 1]
   Normal(u32),
}

impl fmt::Display for ChildNumber {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         ChildNumber::Hardened(n) => write!(f, "{}h", n),
         ChildNumber::Normal(n) => write!(f, "{}", n),
      }
   }
}

impl Serialize for ChildNumber {
   fn serialize<S>(&self, s: &mut S) -> Result<(), S::Error>
   where
      S: Serializer,
   {
      match *self {
         ChildNumber::Hardened(n) => (n + (1 << 31)).serialize(s),
         ChildNumber::Normal(n) => n.serialize(s),
      }
   }
}

impl Deserialize for ChildNumber {
   fn deserialize<D>(d: &mut D) -> Result<ChildNumber, D::Error>
   where
      D: Deserializer,
   {
      let n: u32 = try!(Deserialize::deserialize(d));
      if n < (1 << 31) {
         Ok(ChildNumber::Normal(n))
      } else {
         Ok(ChildNumber::Hardened(n - (1 << 31)))
      }
   }
}

/// A BIP32 error
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Error {
   /// A pk->pk derivation was attempted on a hardened key
   CannotDeriveFromHardenedKey,
   /// A secp256k1 error occured
   Ecdsa(secp256k1::Error),
   /// A child number was provided that was out of range
   InvalidChildNumber(ChildNumber),
   /// Error creating a master seed --- for application use
   RngError(String),
}

impl fmt::Display for Error {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         Error::CannotDeriveFromHardenedKey => {
            f.write_str("cannot derive hardened key from public key")
         },
         Error::Ecdsa(ref e) => fmt::Display::fmt(e, f),
         Error::InvalidChildNumber(ref n) => write!(f, "child number {} is invalid", n),
         Error::RngError(ref s) => write!(f, "rng error {}", s),
      }
   }
}

impl error::Error for Error {
   fn cause(&self) -> Option<&error::Error> {
      if let Error::Ecdsa(ref e) = *self {
         Some(e)
      } else {
         None
      }
   }

   fn description(&self) -> &str {
      match *self {
         Error::CannotDeriveFromHardenedKey => "cannot derive hardened key from public key",
         Error::Ecdsa(ref e) => error::Error::description(e),
         Error::InvalidChildNumber(_) => "child number is invalid",
         Error::RngError(_) => "rng error",
      }
   }
}

impl From<secp256k1::Error> for Error {
   fn from(e: secp256k1::Error) -> Error { Error::Ecdsa(e) }
}

impl ExtendedPubKey {
   /// Derives a public key from a private key
   pub fn from_private(secp: &Secp256k1, sk: &ExtendedPrivKey) -> ExtendedPubKey {
      ExtendedPubKey {
         network:            sk.network,
         depth:              sk.depth,
         parent_fingerprint: sk.parent_fingerprint,
         child_number:       sk.child_number,
         public_key:         PublicKey::from_secret_key(secp, &sk.secret_key).unwrap(),
         chain_code:         sk.chain_code,
      }
   }

   /// Compute the scalar tweak added to this key to get a child key
   pub fn ckd_pub_tweak(
      &self,
      secp: &Secp256k1,
      i: ChildNumber,
   ) -> Result<(SecretKey, ChainCode), Error> {
      match i {
         ChildNumber::Hardened(n) => {
            if n >= (1 << 31) {
               Err(Error::InvalidChildNumber(i))
            } else {
               Err(Error::CannotDeriveFromHardenedKey)
            }
         },
         ChildNumber::Normal(n) => {
            let mut hmac = Hmac::new(Sha512::new(), &self.chain_code[..]);
            hmac.input(&self.public_key.serialize()[..]);
            let mut be_n = [0; 4];
            BigEndian::write_u32(&mut be_n, n);
            hmac.input(&be_n);

            let mut result = [0; 64];
            hmac.raw_result(&mut result);

            let secret_key = try!(SecretKey::from_slice(secp, &result[..32]));
            let chain_code = ChainCode::from(&result[32..]);
            Ok((secret_key, chain_code))
         },
      }
   }

   /// Public->Public child key derivation
   pub fn ckd_pub(&self, secp: &Secp256k1, i: ChildNumber) -> Result<ExtendedPubKey, Error> {
      let (sk, chain_code) = try!(self.ckd_pub_tweak(secp, i));
      let mut pk = self.public_key.clone();
      try!(pk.add_exp_assign(secp, &sk).map_err(Error::Ecdsa));

      Ok(ExtendedPubKey {
         network: self.network,
         depth: self.depth + 1,
         parent_fingerprint: self.fingerprint(),
         child_number: i,
         public_key: pk,
         chain_code,
      })
   }

   /// Returns the HASH160 of the chaincode
   pub fn identifier(&self) -> [u8; 20] {
      let mut sha2_res = [0; 32];
      let mut ripemd_res = [0; 20];
      // Do SHA256 of just the ECDSA pubkey
      let mut sha2 = Sha256::new();
      sha2.input(&self.public_key.serialize()[..]);
      sha2.result(&mut sha2_res);
      // do RIPEMD160
      let mut ripemd = Ripemd160::new();
      ripemd.input(&sha2_res);
      ripemd.result(&mut ripemd_res);
      // Return
      ripemd_res
   }

   /// Returns the first four bytes of the identifier
   pub fn fingerprint(&self) -> Fingerprint { Fingerprint::from(&self.identifier()[0..4]) }
}

impl ToBase58 for ExtendedPubKey {
   fn base58_layout(&self) -> Vec<u8> {
      let mut ret = Vec::with_capacity(78);
      ret.extend(
         match self.network {
            Network::Bitcoin => [0x04u8, 0x88, 0xB2, 0x1E],
            Network::Testnet => [0x04u8, 0x35, 0x87, 0xCF],
         }.iter()
            .cloned(),
      );
      ret.push(self.depth as u8);
      ret.extend(self.parent_fingerprint[..].iter().cloned());
      let mut be_n = [0; 4];
      match self.child_number {
         ChildNumber::Hardened(n) => {
            BigEndian::write_u32(&mut be_n, n + (1 << 31));
         },
         ChildNumber::Normal(n) => {
            BigEndian::write_u32(&mut be_n, n);
         },
      }
      ret.extend(be_n.iter().cloned());
      ret.extend(self.chain_code[..].iter().cloned());
      ret.extend(self.public_key.serialize()[..].iter().cloned());
      ret
   }
}

impl FromBase58 for ExtendedPubKey {
   fn from_base58_layout(data: Vec<u8>) -> Result<ExtendedPubKey, base58::Error> {
      let s = Secp256k1::with_caps(secp256k1::ContextFlag::None);

      if data.len() != 78 {
         return Err(base58::Error::InvalidLength(data.len()));
      }

      let cn_int = Cursor::new(&data[9..13]).read_u32::<BigEndian>().unwrap();
      let child_number = if cn_int < (1 << 31) {
         ChildNumber::Normal(cn_int)
      } else {
         ChildNumber::Hardened(cn_int - (1 << 31))
      };

      Ok(ExtendedPubKey {
         network: if &data[0..4] == [0x04u8, 0x88, 0xB2, 0x1E] {
            Network::Bitcoin
         } else if &data[0..4] == [0x04u8, 0x35, 0x87, 0xCF] {
            Network::Testnet
         } else {
            return Err(base58::Error::InvalidVersion((&data[0..4]).to_vec()));
         },
         depth: data[4],
         parent_fingerprint: Fingerprint::from(&data[5..9]),
         child_number,
         chain_code: ChainCode::from(&data[13..45]),
         public_key: try!(
            PublicKey::from_slice(&s, &data[45..78])
               .map_err(|e| base58::Error::Other(e.to_string()))
         ),
      })
   }
}
