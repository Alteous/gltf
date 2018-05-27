use std::marker::PhantomData;

use Normalize;

use mesh::util::ReadWeights;

/// Casting iterator for `Weights`.
#[derive(Clone, Debug)]
pub struct CastingIter<'a, T>(ReadWeights<'a>, PhantomData<T>);

/// Type which describes how to cast any weight into u8.
#[derive(Clone, Debug)]
pub struct U8;

/// Type which describes how to cast any weight into u16.
#[derive(Clone, Debug)]
pub struct U16;

/// Type which describes how to cast any weight into f32.
#[derive(Clone, Debug)]
pub struct F32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Output;

    /// Cast from u8.
    fn cast_u8(x: [u8; 4]) -> Self::Output;

    /// Cast from u16.
    fn cast_u16(x: [u16; 4]) -> Self::Output;

    /// Cast from f32.
    fn cast_f32(x: [f32; 4]) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: ReadWeights<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Weights` object.
    pub fn unwrap(self) -> ReadWeights<'a> {
        self.0
    }
}

impl<'a, A: Cast> ExactSizeIterator for CastingIter<'a, A> {}
impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            ReadWeights::U8(ref mut i)  => i.next().map(A::cast_u8),
            ReadWeights::U16(ref mut i) => i.next().map(A::cast_u16),
            ReadWeights::F32(ref mut i) => i.next().map(A::cast_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            ReadWeights::U8(ref mut i)  => i.nth(x).map(A::cast_u8),
            ReadWeights::U16(ref mut i) => i.nth(x).map(A::cast_u16),
            ReadWeights::F32(ref mut i) => i.nth(x).map(A::cast_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            ReadWeights::U8(i)  => i.last().map(A::cast_u8),
            ReadWeights::U16(i) => i.last().map(A::cast_u16),
            ReadWeights::F32(i) => i.last().map(A::cast_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            ReadWeights::U8(ref i)  => i.size_hint(),
            ReadWeights::U16(ref i) => i.size_hint(),
            ReadWeights::F32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U8 {
    type Output = [u8; 4];

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 4]) -> Self::Output {
        x.normalize()
    }
}

impl Cast for U16 {
    type Output = [u16; 4];

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 4]) -> Self::Output {
        x.normalize()
    }
}

impl Cast for F32 {
    type Output = [f32; 4];

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 4]) -> Self::Output {
        x.normalize()
    }
}
