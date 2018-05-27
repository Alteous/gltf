use std::marker::PhantomData;

use Normalize;

use mesh::util::ReadTexCoords;

/// Casting iterator for `TexCoords`.
#[derive(Clone, Debug)]
pub struct CastingIter<'a, T>(ReadTexCoords<'a>, PhantomData<T>);

/// Type which describes how to cast any texture coordinate into pair of u8.
#[derive(Clone, Debug)]
pub struct U8;

/// Type which describes how to cast any texture coordinate into pair of u16.
#[derive(Clone, Debug)]
pub struct U16;

/// Type which describes how to cast any texture coordinate into pair of f32.
#[derive(Clone, Debug)]
pub struct F32;

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Output;

    /// Cast from u8 pair.
    fn cast_u8(x: [u8; 2]) -> Self::Output;

    /// Cast from u16 pair.
    fn cast_u16(x: [u16; 2]) -> Self::Output;

    /// Cast from f32 pair.
    fn cast_f32(x: [f32; 2]) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: ReadTexCoords<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `TexCoords` object.
    pub fn unwrap(self) -> ReadTexCoords<'a> {
        self.0
    }
}

impl<'a, A: Cast> ExactSizeIterator for CastingIter<'a, A> {}
impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            ReadTexCoords::U8(ref mut i)  => i.next().map(A::cast_u8),
            ReadTexCoords::U16(ref mut i) => i.next().map(A::cast_u16),
            ReadTexCoords::F32(ref mut i) => i.next().map(A::cast_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            ReadTexCoords::U8(ref mut i)  => i.nth(x).map(A::cast_u8),
            ReadTexCoords::U16(ref mut i) => i.nth(x).map(A::cast_u16),
            ReadTexCoords::F32(ref mut i) => i.nth(x).map(A::cast_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            ReadTexCoords::U8(i)  => i.last().map(A::cast_u8),
            ReadTexCoords::U16(i) => i.last().map(A::cast_u16),
            ReadTexCoords::F32(i) => i.last().map(A::cast_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            ReadTexCoords::U8(ref i)  => i.size_hint(),
            ReadTexCoords::U16(ref i) => i.size_hint(),
            ReadTexCoords::F32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for U8 {
    type Output = [u8; 2];

    fn cast_u8(x: [u8; 2]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 2]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 2]) -> Self::Output {
        x.normalize()
    }
}

impl Cast for U16 {
    type Output = [u16; 2];

    fn cast_u8(x: [u8; 2]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 2]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 2]) -> Self::Output {
        x.normalize()
    }
}

impl Cast for F32 {
    type Output = [f32; 2];

    fn cast_u8(x: [u8; 2]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 2]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 2]) -> Self::Output {
        x.normalize()
    }
}
