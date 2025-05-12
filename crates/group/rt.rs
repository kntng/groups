mod zn_mul;
pub use zn_mul::*;

/// An algebraic `Group` and its basic axioms
pub trait Group {
    type Element<'a>: GroupElement<'a, Group = Self>
    where
        Self: 'a;
    /// Yield the group identity
    fn identity<'a>(&'a self) -> Self::Element<'a>;
    /// Group operation on two element
    fn op<'a>(&'a self, a: &Self::Element<'a>, b: &Self::Element<'a>) -> Self::Element<'a>;
    /// Inverse of an element
    fn inv<'a>(&'a self, a: &Self::Element<'a>) -> Self::Element<'a>;

    fn element<'a>(&'a self, a: usize) -> Option<Self::Element<'a>>;
}

/// Element of a `Group`
pub trait GroupElement<'a> {
    type Group: Group<Element<'a> = Self>
    where
        Self: 'a;
    fn order(&self) -> usize;
}

/// Order of a `Group`
pub trait Finite: Group {
    fn order(&self) -> usize;
}
