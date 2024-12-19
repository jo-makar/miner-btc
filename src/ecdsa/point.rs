use lazy_static::lazy_static;

use num_bigint::{BigInt, Sign};
use num_integer::Integer;

#[derive(Eq, PartialEq)]
pub struct Point<'a> {
    x: BigInt,
    y: BigInt,
    at_pof: bool, // At point of infinity?
    curve: &'a Curve,
}

#[derive(Eq, PartialEq)]
pub struct Curve {
    p: BigInt, // Elliptic curve parameters: $y^2 \equiv x^3 + ax + b \pmod p$
    a: BigInt,
    b: BigInt,
    gx: BigInt, // Generator point on the curve
    gy: BigInt,
    n: BigInt, // Number of possible points on the curve
}

lazy_static! {
    pub static ref SECP256K1_CURVE: Curve = Curve {
        p: BigInt::parse_bytes(
            b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
            16
        )
        .unwrap(),
        a: BigInt::from(0),
        b: BigInt::from(7),
        gx: BigInt::parse_bytes(
            b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16
        )
        .unwrap(),
        gy: BigInt::parse_bytes(
            b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16
        )
        .unwrap(),
        n: BigInt::parse_bytes(
            b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16
        )
        .unwrap(),
    };
}

impl<'a> Point<'a> {
    pub fn new(x: BigInt, y: BigInt, curve: &'a Curve) -> Point<'a> {
        Point {
            x,
            y,
            at_pof: false,
            curve,
        }
    }

    pub fn at_pof(&self) -> bool {
        self.at_pof
    }

    // FIXME Temp name
    fn _on_curve(&self) -> bool {
        if self.at_pof {
            return false;
        }

        let lhs = _modpow(&self.y, &BigInt::from(2), &self.curve.p);

        let mut rhs = _modpow(&self.x, &BigInt::from(3), &self.curve.p);
        rhs += &self.curve.a * &self.x;
        rhs += &self.curve.b;

        lhs == rhs
    }

    // FIXME Temp name
    fn _is_negation(&self, q: &Point) -> bool {
        if self.curve != q.curve {
            panic!("points not on same curve")
        }

        if self.x != q.x {
            return false;
        }

        match (self.y.sign(), q.y.sign()) {
            (Sign::Minus, Sign::Plus) => -&self.y == q.y,
            (Sign::Plus, Sign::Minus) => self.y == -&q.y,
            _ => false,
        }
    }
}

// FIXME STOPPED Add point operations (negate, add, double, multiply)

// The num_bigint::BigInt *pow methods use something like mod_floor not like %,
// which has different behavior if the base or modulus is a negative value.

// FIXME Temp name
fn _modpow(base: &BigInt, power: &BigInt, modulus: &BigInt) -> BigInt {
    let x = base.clone();
    x.modpow(power, modulus);

    if base.sign() == Sign::Minus && power.is_odd() {
        x - modulus
    } else {
        x
    }
}
