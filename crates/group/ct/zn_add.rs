use crate::ct::{Group, GroupElement};

/// An element of Z/N
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZnElement<const N: usize> {
    pub value: usize,
}

impl<const N: usize> GroupElement for ZnElement<N> {
    type Group = Zn<N>;
}

impl<const N: usize> ZnElement<N> {
    pub fn new(value: usize) -> Self {
        Self { value: value % N }
    }
}

pub struct Zn<const N: usize>;

impl<const N: usize> Group for Zn<N> {
    type Element = ZnElement<N>;
    fn identity() -> Self::Element {
        ZnElement::new(0)
    }

    fn op(a: &Self::Element, b: &Self::Element) -> Self::Element {
        ZnElement::new((a.value + b.value) % N)
    }

    fn inv(a: &Self::Element) -> Self::Element {
        if a.value == 0 {
            Self::identity()
        } else {
            ZnElement::new(N - a.value)
        }
    }
}

impl<const N: usize> std::ops::Add for ZnElement<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Zn::<N>::op(&self, &rhs)
    }
}

impl<const N: usize> std::ops::Neg for ZnElement<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Zn::<N>::inv(&self)
    }
}

/// Macro to create Z/N elements
/// ```rust
/// let el = zn!(12, 7);
/// assert_eq!(el.value, 5)
/// ```
macro_rules! zn {
    ($x:expr, $n:literal) => {
        ZnElement::<$n>::new($x)
    };
}
