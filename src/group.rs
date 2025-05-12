mod zn_add;
pub use zn_add::*;

mod zn_mul;
pub use zn_mul::*;

/// An algebraic `Group` and its basic axioms
pub trait Group {
    type Element: GroupElement<Group = Self>;
    /// Yield the group identity
    fn identity() -> Self::Element;
    /// Group operation on two elements
    fn op(a: &Self::Element, b: &Self::Element) -> Self::Element;
    /// Inverse of an element
    fn inv(a: &Self::Element) -> Self::Element;
}

/// Element of a `Group`
pub trait GroupElement {
    type Group: Group<Element = Self>;

    fn order(&self) -> usize
    where
        Self::Group: GroupOrder,
        Self: Clone + PartialEq,
    {
        let mut pow = self.clone();
        let identity = Self::Group::identity();
        for k in 1..=Self::Group::order() {
            if pow == identity {
                return k;
            }
            pow = Self::Group::op(&pow, self)
        }
        unreachable!()
    }
}

/// Order of a `Group`
pub trait GroupOrder: Group {
    fn order() -> usize;
}
