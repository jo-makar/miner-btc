//! Elliptic Curve Digital Signature Algorithm
//!
//! Elliptic curves are defined as $y^2 \equiv x^3 + ax +b \pmod p$.  
//! For which $(4a^3 + 27b^2) % p \neq 0$ to exclude singular curves.
//!
//! These curves are symmetric about the x-axis.  
//! A straight line can intersect the curve at a maximum of three points.
//!
//! Domain parameters
//! - $a$ and $b$ are the equation constants above
//! - $G$ is the generator point, a point on the curve above
//! - $p$ is the (prime) congruence modulo above ie $lhs % p = rhs % p$
//! - $n$ is the number of possible points on the curve, note $n < p$
//!
//! Note that $n * G = O$ (point at infinity).  
//! This implies that $n * pubkey = O$ because $n * (privkey * G) = O$
//!
//! # Arithmetic
//!
//! References
//! - <https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication>
//! - <https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm>
//!
//! ## Point at infinity
//! The point at infinity is a special point that does not lie on the curve,  
//! resulting from adding two points whose sum is not on the curve.  
//! In addition it acts as an identity element, adding it to any point results in itself.
//!
//! ## Point negation
//! Adding a point and its negation results in the point at infinity.  
//! Negated points have the same x coordinate and negated y coordinate.
//!
//! ## Point addition
//!
//! Adding (the x, y components of) one point $P$ to another point $Q$ results in a point $S$.  
//! If a line is drawn from $P$ to $Q$ it will result in a point $R$ such that $R = -S$.
//!
//! $P + Q = R$  
//! $(x_p, y_p) + (x_q, y_q) = (x_r, y_r)$
//!
//! $\lambda = ((y_q - y_p) / (x_q - x_p)) % p$  
//! "Division" is via modular inverse.  
//! Modular inverse: Find $b$ such that $(a * b) % m = 1$  
//! $\lambda = ((y_q - y_p) * modinv(x_q - x_p, p)) % p$
//!
//! $x_r = (\lambda^2 - x_p - x_q) % p$  
//! $y_r = (\lambda * (x_p - x_r) - y_p) % p$
//!
//! ## Point doubling
//! Same as point addition but with  
//! $\lambda = ((3 * x_p^2 + a) / (2 * y_p)) % p$  
//! $\lambda = ((3 * x_p^2 + a) * modinv(2 * y_p, p)) % p$  
//!
//! ## Point multiplication
//!
//! $nP = P + P + P + ... + P$
//!
//! If $n$ is negative:  
//! $(-n)P = (-P) + (-P) + (-P) + ... + (-P)$
//!
//! If $n$ is zero then $nP$ is the point at infinity.
//!
//! ## Trap door function
//! Given $R = kP$ where $R$ and $P$ are known, $k$ cannot be determined.  
//! This is the basis for ECDSA use in public-key cryptography, ie $pubkey = privkey * G$

mod point;

pub use point::{Curve, Point};
