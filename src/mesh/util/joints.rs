use std::marker::PhantomData;

use mesh::util::ReadJoints;

/// Casting iterator for `Joints`.
#[derive(Clone, Debug)]
pub struct CastingIter<'a, T>(ReadJoints<'a>, PhantomData<T>);

/// Type which describes how to cast any joint into u16.
#[derive(Clone, Debug)]
pub struct U16;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Output;

    /// Cast from u8.
    fn cast_u8(x: [u8; 4]) -> Self::Output;

    /// Cast from u16.
    fn cast_u16(x: [u16; 4]) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: ReadJoints<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Joints` object.
    pub fn unwrap(self) -> ReadJoints<'a> {
        self.0
    }
}

impl<'a, A: Cast> ExactSizeIterator for CastingIter<'a, A> {}
impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            ReadJoints::U8(ref mut i)  => i.next().map(A::cast_u8),
            ReadJoints::U16(ref mut i) => i.next().map(A::cast_u16),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            ReadJoints::U8(ref mut i)  => i.nth(x).map(A::cast_u8),
            ReadJoints::U16(ref mut i) => i.nth(x).map(A::cast_u16),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            ReadJoints::U8(i)  => i.last().map(A::cast_u8),
            ReadJoints::U16(i) => i.last().map(A::cast_u16),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            ReadJoints::U8(ref i)  => i.size_hint(),
            ReadJoints::U16(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U16 {
    type Output = [u16; 4];

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        [x[0] as u16, x[1] as u16, x[2] as u16, x[3] as u16]
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x
    }
}
