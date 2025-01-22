use crate::ecdsa::{point::SECP256K1_CURVE, Curve, Point};

use num_bigint::{BigInt, BigUint, RandBigInt, Sign};
use num_integer::Integer;
use num_traits::cast::ToPrimitive;
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

pub struct PrivKey<'a> {
    d: BigInt, // Private key
    curve: &'a Curve,
}

pub struct PubKey<'a> {
    e: Point<'a>, // Public key
}

impl<'a> PrivKey<'a> {
    pub fn new(d: BigInt, curve: &'a Curve) -> PrivKey<'a> {
        if d < BigInt::from(1) || *curve.order() <= d {
            panic!("invalid private key");
        }
        let privkey = PrivKey { d, curve };

        if !privkey.pubkey().on_curve() {
            panic!("pubkey not on curve");
        }

        privkey
    }

    pub fn new_bitcoin(privkey: &[u8]) -> PrivKey<'a> {
        let curve = &SECP256K1_CURVE;
        let d = match BigUint::parse_bytes(privkey, 16) {
            Some(d) => BigInt::from_biguint(Sign::Plus, d),
            None => panic!("invalid hex value"),
        };
        if d < BigInt::from(1) || *curve.order() <= d {
            panic!("invalid private key");
        }
        let privkey = PrivKey { d, curve };

        if !privkey.pubkey().on_curve() {
            panic!("pubkey not on curve");
        }

        privkey
    }

    pub fn rand_bitcoin() -> PrivKey<'a> {
        let curve = &SECP256K1_CURVE;
        let privkey = PrivKey {
            d: {
                let mut rng = rand::thread_rng();
                rng.gen_bigint_range(&BigInt::from(1), curve.order())
            },
            curve,
        };

        if !privkey.pubkey().on_curve() {
            panic!("pubkey not on curve");
        }

        privkey
    }

    pub fn pubkey(&self) -> PubKey<'a> {
        let g = self.curve.generator();
        PubKey { e: &g * &self.d }
    }
}

impl PubKey<'_> {
    pub fn on_curve(&self) -> bool {
        self.e.on_curve()
    }

    pub fn base58_addr(&self, mainnet: bool) -> String {
        if *self.e.curve() != *SECP256K1_CURVE {
            panic!("base58 applied on non-bitcoin pubkey");
        }

        fn compressed(e: &Point) -> [u8; 33] {
            fn abs(z: &BigInt, p: &BigInt) -> BigInt {
                if z < &BigInt::ZERO {
                    if &-z > p {
                        panic!("z + p < 0");
                    }
                    z + p
                } else {
                    z.clone()
                }
            }

            let p = e.curve().modulo();
            let x = abs(e.x(), p);
            let y = abs(e.y(), p);

            fn to_32_bytes_be(x: &BigInt) -> [u8; 32] {
                let mut bytes = x.to_signed_bytes_be();

                if bytes.len() == 33 && bytes[0] == 0 {
                    bytes.remove(0);
                }
                if bytes.len() > 32 {
                    panic!("pubkey x value too large");
                }
                if bytes.len() < 32 {
                    bytes.splice(0..0, [0].repeat(32 - bytes.len()));
                }

                bytes.try_into().unwrap()
            }

            let mut rv = vec![if y.is_odd() { 0x03 } else { 0x02 }];
            rv.extend(to_32_bytes_be(&x));
            rv.try_into().unwrap()
        }

        fn sha256(data: &[u8]) -> [u8; 32] {
            let mut h = Sha256::new();
            h.update(data);
            h.finalize().into()
        }

        fn ripemd160(data: &[u8]) -> [u8; 20] {
            let mut h = Ripemd160::new();
            h.update(data);
            h.finalize().into()
        }

        let addr: [u8; 25] = {
            let mut addr = vec![if mainnet { 0x00 } else { 0x6f }];
            addr.extend(ripemd160(&sha256(compressed(&self.e).as_slice())));

            let checksum = sha256(&sha256(addr.as_slice()));
            addr.extend(&checksum[0..4]);

            addr.try_into().unwrap()
        };

        fn base58_encode(data: &[u8]) -> String {
            const ALPHABET: &[u8; 58] =
                b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

            let mut rv = String::new();
            let mut x = BigInt::from_bytes_be(Sign::Plus, data);
            while x > BigInt::from(0) {
                let r: BigInt;
                (x, r) = x.div_rem(&BigInt::from(58));
                rv.insert(0, ALPHABET[r.to_usize().unwrap()] as char);
            }

            // Prepend the first alphabet symbol for each leading zero
            for b in data {
                if *b != 0 {
                    break;
                }
                rv.insert(0, ALPHABET[0] as char);
            }

            rv
        }

        base58_encode(&addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privkey_to_base58_addr() {
        let table = [
            // From https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses
            (
                "18e14a7b6a307f426a94f8114701e7c8e774e7f9a47e2c2035db29a206321725",
                "1PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs",
            ),
            // From https://privatekeys.pw
            (
                "74f0a07b86441a008ac179a308255343cbc1c325f9fdd0ed9fbadb40bd294b32",
                "1FZqHrYNTMLpJMUwiiPERGwSHZdewNWLPX",
            ),
            (
                "a56b8931f3e515bf6ec223ce4be2b9ab396fbf26cf1132b45decbb8c4a2babaa",
                "1QK6LMCC583QJPpvgzQ4SSKJdtfkYPzTKR",
            ),
            (
                "eb5010572cf15c436da40f624cc4c47e65178081a36d30a817d47d85eb45132e",
                "1HLgnekKKRdBQ7z2txxW5EHUGZt36cUHAR",
            ),
            (
                "a1d709fc21fe7b56ed4f14acf23586dafee8449822741d1cdf2c15c6595004e7",
                "1MJAeep33PMC5kWRdH4KUjSk2MfWBwMDxi",
            ),
            (
                "2f9cc588cba4f0dd7f92022c4795dedb0d62b8b9c0987e3d615f2c4b3762fa84",
                "1BcMWDJQz4iFiZ6GPvpVP7bqzi6qKFCMGg",
            ),
            (
                "dd5f6cd50ea9995ad25d7481e3b45b10e6de8655bbfe295ec9c24ce34419e8e3",
                "1P1GtJvtiSY25CFRbXvXxDtdjygfHUbVUd",
            ),
            (
                "2c5ef6ecc00442919671babe3e4a2963ee377b7a9a2ba22fd299a7c1ab6007b7",
                "13QRBvNYjAYF8b3YHbXJKsc3T6pgZ2MRMj",
            ),
            (
                "fdc0cd4245259d04124168c22f84ad04a5aee435a330c0e716bf47cf095319fd",
                "1CiHaVULNUpqn22B5mV4Y6pw1rYPsSzE7R",
            ),
            (
                "be6fed0077d17dd919e64047f318068757e98fbdcfa84eda01cf122fc46a6be6",
                "1FVkUAgDeosKyQeGfWHtHhhryBptaz3R7h",
            ),
            (
                "15da872c95a13dd738fbf50e427583ad61f18fd99f628c417a61cf8343c90419",
                "1Nhc1grLraxJbCiGLPryCtv2d3i7G4Y9md",
            ),
        ];

        for (privkey, addr) in table.iter() {
            let privkey = PrivKey::new_bitcoin(privkey.as_bytes());
            let pubkey = privkey.pubkey();
            assert_eq!(pubkey.base58_addr(true), addr.to_string());
        }
    }
}
