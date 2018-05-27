use animation::util::Rotations;
use std::marker::PhantomData;
use Normalize;

/// Casting iterator for `Rotations`.
#[derive(Clone, Debug)]
pub struct CastingIter<'a, T>(Rotations<'a>, PhantomData<T>);

/// Type which describes how to cast any weight into i8.
#[derive(Clone, Debug)]
pub struct I8;

/// Type which describes how to cast any weight into u8.
#[derive(Clone, Debug)]
pub struct U8;

/// Type which describes how to cast any weight into i16.
#[derive(Clone, Debug)]
pub struct I16;

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

    /// Cast from i8.
    fn cast_i8(x: [i8; 4]) -> Self::Output;

    /// Cast from u8.
    fn cast_u8(x: [u8; 4]) -> Self::Output;

    /// Cast from i16.
    fn cast_i16(x: [i16; 4]) -> Self::Output;

    /// Cast from u16.
    fn cast_u16(x: [u16; 4]) -> Self::Output;

    /// Cast from f32.
    fn cast_f32(x: [f32; 4]) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: Rotations<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `Rotations` object.
    pub fn unwrap(self) -> Rotations<'a> {
        self.0
    }
}

impl<'a, A: Cast> ExactSizeIterator for CastingIter<'a, A> {}
impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Rotations::I8(ref mut i)  => i.next().map(A::cast_i8),
            Rotations::U8(ref mut i)  => i.next().map(A::cast_u8),
            Rotations::I16(ref mut i) => i.next().map(A::cast_i16),
            Rotations::U16(ref mut i) => i.next().map(A::cast_u16),
            Rotations::F32(ref mut i) => i.next().map(A::cast_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            Rotations::I8(ref mut i)  => i.nth(x).map(A::cast_i8),
            Rotations::U8(ref mut i)  => i.nth(x).map(A::cast_u8),
            Rotations::I16(ref mut i) => i.nth(x).map(A::cast_i16),
            Rotations::U16(ref mut i) => i.nth(x).map(A::cast_u16),
            Rotations::F32(ref mut i) => i.nth(x).map(A::cast_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Rotations::I8(i)  => i.last().map(A::cast_i8),
            Rotations::U8(i)  => i.last().map(A::cast_u8),
            Rotations::I16(i) => i.last().map(A::cast_i16),
            Rotations::U16(i) => i.last().map(A::cast_u16),
            Rotations::F32(i) => i.last().map(A::cast_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Rotations::I8(ref i)  => i.size_hint(),
            Rotations::U8(ref i)  => i.size_hint(),
            Rotations::I16(ref i) => i.size_hint(),
            Rotations::U16(ref i) => i.size_hint(),
            Rotations::F32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for I8 {
    type Output = [i8; 4];

    fn cast_i8(x: [i8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: [i16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 4]) -> Self::Output {
        x.normalize()
    }
}

impl Cast for i8 {
    type Output = [u8; 4];

    fn cast_i8(x: [i8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: [i16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 4]) -> Self::Output {
        x.normalize()
    }
}

impl Cast for I16 {
    type Output = [i16; 4];

    fn cast_i8(x: [i8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: [i16; 4]) -> Self::Output {
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

    fn cast_i8(x: [i8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: [i16; 4]) -> Self::Output {
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

    fn cast_i8(x: [i8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u8(x: [u8; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_i16(x: [i16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_u16(x: [u16; 4]) -> Self::Output {
        x.normalize()
    }

    fn cast_f32(x: [f32; 4]) -> Self::Output {
        x.normalize()
    }
}
