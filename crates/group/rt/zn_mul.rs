use crate::utils::{euler_totient, mod_inv};

use crate::rt::{Finite, Group, GroupElement};

/// An element of Z/N
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ZnMulElement<'a> {
    pub value: usize,
    group: &'a ZnMul,
}

impl<'a> GroupElement<'a> for ZnMulElement<'a> {
    type Group = ZnMul;

    fn order(&self) -> usize
    where
        Self::Group: Finite,
        Self: Clone + PartialEq,
    {
        let mut pow = self.clone();
        let identity = Self::Group::identity(&self.group);
        for k in 1..=Self::Group::order(&self.group) {
            if pow == identity {
                return k;
            }
            pow = Self::Group::op(&self.group, &pow, self)
        }
        unreachable!()
    }

    fn subgroup(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        self.group.subgroup(self)
    }
}

impl<'a> ZnMulElement<'a> {
    pub fn new(value: usize, group: &'a ZnMul) -> Option<Self> {
        if mod_inv(value, group.n).is_some() {
            Some(Self {
                value: value % group.n,
                group,
            })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ZnMul {
    pub n: usize,
}

impl ZnMul {
    pub fn new(n: usize) -> Self {
        Self { n }
    }

    pub fn iter(&self) -> ZnMulIterator<'_> {
        ZnMulIterator {
            candidate: 1,
            group: self,
            yielded: 0,
            total: self.order(),
        }
    }
}

impl Group for ZnMul {
    type Element<'a>
        = ZnMulElement<'a>
    where
        Self: 'a;

    fn identity<'a>(&'a self) -> Self::Element<'a> {
        ZnMulElement::new(1, self).unwrap()
    }

    fn op<'a>(&'a self, a: &Self::Element<'a>, b: &Self::Element<'a>) -> Self::Element<'a> {
        ZnMulElement::new((a.value * b.value) % self.n, &self).unwrap()
    }

    fn inv<'a>(&'a self, a: &Self::Element<'a>) -> Self::Element<'a> {
        ZnMulElement::new(mod_inv(a.value, self.n).unwrap(), &self).unwrap()
    }

    fn element<'a>(&'a self, a: usize) -> Option<Self::Element<'a>> {
        Self::Element::new(a, self)
    }

    fn subgroup<'a>(&'a self, a: &Self::Element<'a>) -> Vec<Self::Element<'a>> {
        let mut subgroup = Vec::with_capacity(a.order());
        let mut pow = a.clone();
        for _ in 1..=a.order() {
            subgroup.push(pow.clone());
            pow = self.op(&pow, a);
        }
        subgroup
    }
}

impl Finite for ZnMul {
    fn order(&self) -> usize {
        euler_totient(self.n)
    }
}

/// An iterator over the elements of a Z/N multiplicative group
pub struct ZnMulIterator<'a> {
    candidate: usize,
    group: &'a ZnMul,
    // State for ExactSizeIterator
    yielded: usize,
    total: usize,
}

impl<'a> std::iter::Iterator for ZnMulIterator<'a>
where
    Self: 'a,
{
    type Item = ZnMulElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.candidate < self.group.n {
            if let Some(el) = self.group.element(self.candidate) {
                self.candidate += 1;
                self.yielded += 1;
                return Some(el);
            }
            self.candidate += 1;
        }
        None
    }
}

impl<'a> ExactSizeIterator for ZnMulIterator<'a> {
    fn len(&self) -> usize {
        self.total - self.yielded
    }
}

impl<'a> std::ops::Mul for ZnMulElement<'a> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ZnMul::op(&self.group, &self, &rhs)
    }
}

impl<'a> std::ops::Neg for ZnMulElement<'a> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ZnMul::inv(&self.group, &self)
    }
}

/// Macro to create (Z/N) multiplicative elements
/// ```rust
/// use group::{znmul, rt::{ZnMul, Group}};
/// assert_eq!(znmul!(7, 8).unwrap().value, 7)
/// ```
#[macro_export]
macro_rules! znmul {
    ($x:expr, $n:expr) => {
        ZnMul::new($n).element($x)
    };
}

#[cfg(test)]
mod tests {
    use crate::rt::{Finite, Group, GroupElement};

    use super::{ZnMul, ZnMulElement};

    #[test]
    fn z_4() {
        let g = ZnMul::new(4);
        let id = g.identity();
        assert!(znmul!(2, 4).is_none());
        assert!(znmul!(1, 4).is_some());
        assert!(znmul!(3, 4).is_some());

        assert_eq!(g.order(), 2);
        assert_eq!(znmul!(1, 4).unwrap().order(), 1);
        assert_eq!(znmul!(3, 4).unwrap().order(), 2);

        let elements: Vec<usize> = g.iter().map(|e| e.value).collect();
        assert_eq!(elements, vec![1, 3]);
    }
}
