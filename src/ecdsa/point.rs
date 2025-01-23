use lazy_static::lazy_static;

use num_bigint::{BigInt, Sign};

use std::ops::{Add, AddAssign, Mul, Neg};

#[derive(Clone, Eq, PartialEq)]
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
    n: BigInt, // Number of possible points on the curve (also called its order)
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

// The BigInt.mod* methods round like mod_floor, ie given a modulus>0 the range is [0, modulus),
// which is unlike % whose range is (-modulus, modulus) depending on the dividend sign.

impl<'a> Point<'a> {
    pub fn new(x: BigInt, y: BigInt, curve: &'a Curve) -> Point<'a> {
        Point {
            x,
            y,
            at_pof: false,
            curve,
        }
    }

    pub fn x(&self) -> &BigInt {
        &self.x
    }

    pub fn y(&self) -> &BigInt {
        &self.y
    }

    pub fn curve(&self) -> &Curve {
        self.curve
    }

    pub fn at_pof(&self) -> bool {
        self.at_pof
    }

    pub fn on_curve(&self) -> bool {
        if self.at_pof {
            return false;
        }

        let lhs = self.y.modpow(&BigInt::from(2), &self.curve.p);

        let mut rhs = (self.x.modpow(&BigInt::from(3), &self.curve.p)
            + (&self.curve.a * &self.x)
            + &self.curve.b)
            % &self.curve.p;
        if rhs < BigInt::ZERO {
            rhs += &self.curve.p;
        }

        lhs == rhs
    }

    fn is_negation(&self, q: &Point) -> bool {
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

    fn double(&self, check_on_curve: bool) -> Point<'a> {
        if self.at_pof {
            return Point {
                x: BigInt::from(0),
                y: BigInt::from(0),
                at_pof: true,
                curve: self.curve,
            };
        }

        let lambda = ((self.x.modpow(&BigInt::from(2), &self.curve.p) * BigInt::from(3)
            + &self.curve.a)
            * (&BigInt::from(2) * &self.y).modinv(&self.curve.p).unwrap())
            % &self.curve.p;
        let x =
            (lambda.modpow(&BigInt::from(2), &self.curve.p) - &self.x - &self.x) % &self.curve.p;
        let y = (&lambda * (&self.x - &x) - &self.y) % &self.curve.p;

        let p = Point {
            x,
            y,
            at_pof: false,
            curve: self.curve,
        };

        if check_on_curve && !p.on_curve() {
            panic!("doubled point not on curve");
        }

        p
    }
}

impl<'a> Neg for &Point<'a> {
    type Output = Point<'a>;

    fn neg(self) -> Self::Output {
        if self.at_pof {
            panic!("negate point at infinity");
        }

        let p = Point {
            x: self.x.clone(),
            y: -&self.y,
            at_pof: false,
            curve: self.curve,
        };

        if !p.on_curve() {
            panic!("negated point not on curve");
        }

        p
    }
}

impl<'a> Add<&Point<'_>> for &Point<'a> {
    type Output = Point<'a>;

    fn add(self, other: &Point) -> Self::Output {
        if self.curve != other.curve {
            panic!("points not on same curve");
        }

        if self.at_pof && other.at_pof {
            return Point {
                x: BigInt::from(0),
                y: BigInt::from(0),
                at_pof: true,
                curve: self.curve,
            };
        } else if self.at_pof && !other.at_pof {
            return Point {
                x: other.x.clone(),
                y: other.y.clone(),
                at_pof: false,
                curve: self.curve,
            };
        } else if !self.at_pof && other.at_pof {
            return Point {
                x: self.x.clone(),
                y: self.y.clone(),
                at_pof: false,
                curve: self.curve,
            };
        }

        if self == other {
            return self.double(true);
        }

        if self.is_negation(other) {
            return Point {
                x: BigInt::from(0),
                y: BigInt::from(0),
                at_pof: true,
                curve: self.curve,
            };
        }

        if self.x == other.x {
            panic!("points with same x but not negations");
        }

        let lambda = ((&other.x - &self.x).modinv(&self.curve.p).unwrap() * (&other.y - &self.y))
            % &self.curve.p;
        let x =
            (lambda.modpow(&BigInt::from(2), &self.curve.p) - &self.x - &other.x) % &self.curve.p;
        let y = ((&lambda * (&other.x - &x)) - &other.y) % &self.curve.p;

        let p = Point {
            x,
            y,
            at_pof: false,
            curve: self.curve,
        };

        if !p.on_curve() {
            panic!("added point not on curve");
        }

        p
    }
}

impl<'a> AddAssign<&Point<'a>> for Point<'a> {
    fn add_assign(&mut self, other: &Point<'a>) {
        // Dereference then rereference to convert from a mutable to an immutable reference
        let sum = &*self + other;
        self.x = sum.x;
        self.y = sum.y;
        self.at_pof = sum.at_pof;
    }
}

impl<'a> Mul<&BigInt> for &Point<'a> {
    type Output = Point<'a>;

    fn mul(self, other: &BigInt) -> Self::Output {
        if self.at_pof || *other == BigInt::ZERO {
            return Point {
                x: BigInt::from(0),
                y: BigInt::from(0),
                at_pof: true,
                curve: self.curve,
            };
        }

        // To compute $sP$ decompose $s$ into its binary representation:
        //   $s = s_0 + 2s_1 + 2^2s_2 + ... + 2^(n-1)s_(n-1)$
        //   where $s_0, ..., s_(n-1) \in {0,1}, n = \lceil log_2 s \rceil$

        let mut rv = Point {
            x: BigInt::from(0),
            y: BigInt::from(0),
            at_pof: true,
            curve: self.curve,
        };
        let mut p = self.clone();

        let (_, bytes) = other.to_bytes_le();
        for byte in bytes {
            for bit in 0..8 {
                if byte & (1 << bit) != 0 {
                    rv += &p;
                }
                p = p.double(false);
            }
        }

        if !rv.on_curve() {
            panic!("multiplied point not on curve");
        }

        rv
    }
}

impl Curve {
    pub fn order(&self) -> &BigInt {
        &self.n
    }

    pub fn modulo(&self) -> &BigInt {
        &self.p
    }

    pub fn generator(&self) -> Point {
        Point {
            x: self.gx.clone(),
            y: self.gy.clone(),
            at_pof: false,
            curve: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::panic::catch_unwind;

    #[test]
    fn curve_validate() {
        for curve in vec![&SECP256K1_CURVE] {
            assert!(curve.n < curve.p);

            // Verify $(4a^3 + 27b^2) % p \neq 0$
            assert_ne!(
                ((BigInt::from(4) * curve.a.modpow(&BigInt::from(3), &curve.p))
                    + (BigInt::from(27) * curve.b.modpow(&BigInt::from(2), &curve.p)))
                    % &curve.p,
                BigInt::ZERO
            );

            let g = Point {
                x: curve.gx.clone(),
                y: curve.gy.clone(),
                at_pof: false,
                curve,
            };
            assert!(g.on_curve());

            let panic = catch_unwind(|| &g * &curve.n);
            assert_eq!(
                panic.err().unwrap().downcast_ref::<&str>().unwrap(),
                &"multiplied point not on curve"
            );
        }
    }

    lazy_static! {
        static ref CURVE: Curve = Curve {
            p: BigInt::from(7),
            a: BigInt::from(3),
            b: BigInt::from(4),
            // Unused fields
            gx: BigInt::ZERO,
            gy: BigInt::ZERO,
            n: BigInt::ZERO
        };
    }

    #[test]
    fn point_add() {
        let p = Point {
            x: BigInt::from(1),
            y: BigInt::from(1),
            at_pof: false,
            curve: &CURVE,
        };
        assert!(p.on_curve());

        let q = Point {
            x: BigInt::from(2),
            y: BigInt::from(5),
            at_pof: false,
            curve: &CURVE,
        };
        assert!(q.on_curve());

        let r = &p + &q;
        assert!(r.on_curve());
        assert!(r.x == BigInt::from(-1) || r.x == BigInt::from(6));
        assert_eq!(r.y, BigInt::from(0));
    }

    #[test]
    fn point_add_negation() {
        let p = Point {
            x: BigInt::from(1),
            y: BigInt::from(1),
            at_pof: false,
            curve: &CURVE,
        };
        assert!(p.on_curve());

        let q = -&p;
        assert!(q.on_curve());

        assert!((&p + &q).at_pof);
    }

    #[test]
    fn point_double() {
        let p = Point {
            x: BigInt::from(2),
            y: BigInt::from(2),
            at_pof: false,
            curve: &CURVE,
        };
        assert!(p.on_curve());

        let q = p.double(false);
        assert!(q.on_curve());
        assert_eq!(q.x, BigInt::ZERO);
        assert_eq!(q.y, BigInt::from(2));

        let q = &p + &p;
        assert!(q.on_curve());
        assert_eq!(q.x, BigInt::ZERO);
        assert_eq!(q.y, BigInt::from(2));
    }

    #[test]
    fn point_mul() {
        let curve = &Curve {
            p: BigInt::from(17),
            a: BigInt::from(0),
            b: BigInt::from(7),
            // Unused fields
            gx: BigInt::ZERO,
            gy: BigInt::ZERO,
            n: BigInt::ZERO,
        };

        let p = Point {
            x: BigInt::from(15),
            y: BigInt::from(13),
            at_pof: false,
            curve,
        };
        assert!(p.on_curve());

        let q = &p * &BigInt::from(6);
        assert!(q.on_curve());
        assert_eq!(q.x, BigInt::from(5));
        assert!(q.y == BigInt::from(8) || q.y == BigInt::from(-9));
    }
}
