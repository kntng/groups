use crate::utils::{euler_totient, mod_inv};

use super::{Group, GroupElement, GroupOrder};

/// An element of Z/N
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZnMulElement<const N: usize> {
    pub value: usize,
}

impl<const N: usize> GroupElement for ZnMulElement<N> {
    type Group = ZnMul<N>;
}

impl<const N: usize> ZnMulElement<N> {
    pub fn new(value: usize) -> Option<Self> {
        if mod_inv(value, N).is_some() {
            Some(Self { value: value % N })
        } else {
            None
        }
    }
}

pub struct ZnMul<const N: usize>;

impl<const N: usize> Group for ZnMul<N> {
    type Element = ZnMulElement<N>;
    fn identity() -> Self::Element {
        ZnMulElement::new(1).unwrap()
    }

    fn op(a: &Self::Element, b: &Self::Element) -> Self::Element {
        ZnMulElement::new((a.value * b.value) % N).unwrap()
    }

    fn inv(a: &Self::Element) -> Self::Element {
        ZnMulElement::new(mod_inv(a.value, N).unwrap()).unwrap()
    }
}

impl<const N: usize> GroupOrder for ZnMul<N> {
    fn order() -> usize {
        euler_totient(N)
    }
}

impl<const N: usize> std::ops::Mul for ZnMulElement<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ZnMul::<N>::op(&self, &rhs)
    }
}

impl<const N: usize> std::ops::Neg for ZnMulElement<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ZnMul::<N>::inv(&self)
    }
}

/// Macro to create (Z/N) multiplicative elements
/// ```rust
/// let el = znmul!(7, 8);
/// assert_eq!(el.value, 7)
/// ```
macro_rules! znmul {
    ($x:expr, $n:literal) => {
        ZnMulElement::<$n>::new($x)
    };
}

#[cfg(test)]
mod tests {
    use crate::group::Group;

    use super::{ZnMul, ZnMulElement};

    #[test]
    fn z_4() {
        let id = ZnMul::<4>::identity();
        assert!(znmul!(2, 4).is_none());
        assert!(znmul!(1, 4).is_some());
    }
}
