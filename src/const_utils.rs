pub const fn const_gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub const fn const_totient(n: usize) -> usize {
    let mut count = 0;
    let mut i = 1;
    while i < n {
        if const_gcd(i, n) == 1 {
            count += 1;
        }
        i += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{const_gcd, const_totient};

    #[test]
    fn gcd() {
        assert_eq!(const_gcd(3, 7), 1);
        assert_eq!(const_gcd(3, 6), 3);

        assert_eq!(const_gcd(27, 121), 1);
        assert_eq!(const_gcd(12009, 512384), 4003);
    }

    #[test]
    fn phi() {
        assert_eq!(const_totient(7), 6);
        assert_eq!(const_totient(19), 18);

        assert_eq!(const_totient(4), 2);
        assert_eq!(const_totient(24), 8);
        assert_eq!(const_totient(10), 4);
    }
}
