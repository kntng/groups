/// Runs the Euclidean algorithm to compute gcd(a,b)
pub fn euclidean(a: isize, b: isize) -> isize {
    let (mut a_, mut b_) = (a, b);
    while b_ != 0 {
        (a_, b_) = (b_, a_ % b_);
    }
    return a_;
}

/// Runs the Extended Euclidean algorithm to compute
/// $ax + by = gcd(a,b)$
/// Returns (x, y, gcd(a,b))
pub fn extended_euclidean(a: isize, b: isize) -> (isize, isize, isize) {
    let (mut a_, mut b_) = (a, b);
    let (mut s, mut s_new) = (1, 0);

    while b_ != 0 {
        let q = a_ / b_;
        (a_, b_) = (b_, a_ % b_);
        (s, s_new) = (s_new, s - q * s_new);
    }

    let t = if b != 0 { (a_ - s * a) / b } else { 0 };
    return (s, t, a_);
}

/// Computes the modular inverse of $a mod n$
pub fn mod_inv(a: usize, n: usize) -> Option<usize> {
    let n_ = n as isize;
    let (x, _, gcd) = extended_euclidean((a % n) as isize, n_);
    if gcd > 1 {
        None
    } else {
        Some(((x % n_ + n_) % n_) as usize)
    }
}

/// Computes Euler's totient/phi function
pub fn euler_totient(n: usize) -> usize {
    let mut phi = n;
    let mut n_ = n;
    let mut p = 2;

    while p * p <= n_ {
        if n_ % p == 0 {
            while n_ % p == 0 {
                n_ /= p;
            }
            phi -= phi / p;
        }
        p += 1;
    }

    if n_ > 1 {
        phi -= phi / n_;
    }
    phi
}

#[cfg(test)]
mod tests {
    use crate::utils::euler_totient;

    use super::{euclidean, extended_euclidean, mod_inv};

    #[test]
    fn gcd() {
        assert_eq!(euclidean(3, 7), 1);
        assert_eq!(euclidean(3, 6), 3);

        assert_eq!(euclidean(27, 121), 1);
        assert_eq!(euclidean(12009, 512384), 4003);

        assert_eq!(euclidean(-30, 12), -6);
    }

    #[test]
    fn bezout() {
        assert_eq!(extended_euclidean(15, 69), (-9, 2, 3));

        assert_eq!(extended_euclidean(-30, 12), (1, 2, -6));
    }

    #[test]
    fn inverse() {
        assert_eq!(mod_inv(8, 26), None);
        assert_eq!(mod_inv(7, 26), Some(15));
        assert_eq!(mod_inv(4, 9), Some(7));
        assert_eq!(mod_inv(4, 19), Some(5));
    }

    #[test]
    fn phi() {
        assert_eq!(euler_totient(7), 6);
        assert_eq!(euler_totient(19), 18);

        assert_eq!(euler_totient(4), 2);
        assert_eq!(euler_totient(24), 8);
        assert_eq!(euler_totient(10), 4);
    }
}
