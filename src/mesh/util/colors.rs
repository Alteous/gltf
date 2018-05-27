use std::marker::PhantomData;

use Normalize;

use mesh::util::ReadColors;

/// Casting iterator for `Colors`.
#[derive(Clone, Debug)]
pub struct CastingIter<'a, T>(ReadColors<'a>, PhantomData<T>);

/// Type which describes how to cast any color into RGB u8.
#[derive(Clone, Debug)]
pub struct RgbU8;

/// Type which describes how to cast any color into RGB u16.
#[derive(Clone, Debug)]
pub struct RgbU16;

/// Type which describes how to cast any color into RGB f32.
#[derive(Clone, Debug)]
pub struct RgbF32;

/// Type which describes how to cast any color into RGBA u8.
#[derive(Clone, Debug)]
pub struct RgbaU8;

/// Type which describes how to cast any color into RGBA u16.
#[derive(Clone, Debug)]
pub struct RgbaU16;

/// Type which describes how to cast any color into RGBA f32.
#[derive(Clone, Debug)]
pub struct RgbaF32;

trait ColorChannel {
    fn max_color() -> Self;
}

impl ColorChannel for u8 {
    fn max_color() -> Self { u8::max_value() }
}

impl ColorChannel for u16 {
    fn max_color() -> Self { u16::max_value() }
}

impl ColorChannel for f32 {
    fn max_color() -> Self { 1.0 }
}

trait ColorArray<T> {
    fn into_rgb(self)  -> [T; 3];
    fn into_rgba(self) -> [T; 4];
}

impl<T: Copy + ColorChannel> ColorArray<T> for [T; 3] {
    fn into_rgb(self)  -> [T; 3] { self }
    fn into_rgba(self) -> [T; 4] { [self[0], self[1], self[2], T::max_color()] }
}

impl<T: Copy + ColorChannel> ColorArray<T> for [T; 4] {
    fn into_rgb(self)  -> [T; 3] { [self[0], self[1], self[2]] }
    fn into_rgba(self) -> [T; 4] { self }
}

/// Trait for types which describe casting behaviour.
pub trait Cast {
    /// Output type.
    type Output;

    /// Cast from RGB u8.
    fn cast_rgb_u8(x: [u8; 3]) -> Self::Output;

    /// Cast from RGB u16.
    fn cast_rgb_u16(x: [u16; 3]) -> Self::Output;

    /// Cast from RGB f32.
    fn cast_rgb_f32(x: [f32; 3]) -> Self::Output;

    /// Cast from RGBA u8.
    fn cast_rgba_u8(x: [u8; 4]) -> Self::Output;

    /// Cast from RGBA u16.
    fn cast_rgba_u16(x: [u16; 4]) -> Self::Output;

    /// Cast from RGBA f32.
    fn cast_rgba_f32(x: [f32; 4]) -> Self::Output;
}

impl<'a, A> CastingIter<'a, A> {
    pub(crate) fn new(iter: ReadColors<'a>) -> Self {
        CastingIter(iter, PhantomData)
    }

    /// Unwrap underlying `ReadColors` object.
    pub fn unwrap(self) -> ReadColors<'a> {
        self.0
    }
}

impl<'a, A: Cast> ExactSizeIterator for CastingIter<'a, A> {}
impl<'a, A: Cast> Iterator for CastingIter<'a, A> {
    type Item = A::Output;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            ReadColors::RgbU8(ref mut i)   => i.next().map(A::cast_rgb_u8),
            ReadColors::RgbU16(ref mut i)  => i.next().map(A::cast_rgb_u16),
            ReadColors::RgbF32(ref mut i)  => i.next().map(A::cast_rgb_f32),
            ReadColors::RgbaU8(ref mut i)  => i.next().map(A::cast_rgba_u8),
            ReadColors::RgbaU16(ref mut i) => i.next().map(A::cast_rgba_u16),
            ReadColors::RgbaF32(ref mut i) => i.next().map(A::cast_rgba_f32),
        }
    }

    #[inline]
    fn nth(&mut self, x: usize) -> Option<Self::Item> {
        match self.0 {
            ReadColors::RgbU8(ref mut i)   => i.nth(x).map(A::cast_rgb_u8),
            ReadColors::RgbU16(ref mut i)  => i.nth(x).map(A::cast_rgb_u16),
            ReadColors::RgbF32(ref mut i)  => i.nth(x).map(A::cast_rgb_f32),
            ReadColors::RgbaU8(ref mut i)  => i.nth(x).map(A::cast_rgba_u8),
            ReadColors::RgbaU16(ref mut i) => i.nth(x).map(A::cast_rgba_u16),
            ReadColors::RgbaF32(ref mut i) => i.nth(x).map(A::cast_rgba_f32),
        }
    }

    fn last(self) -> Option<Self::Item> {
        match self.0 {
            ReadColors::RgbU8(i)   => i.last().map(A::cast_rgb_u8),
            ReadColors::RgbU16(i)  => i.last().map(A::cast_rgb_u16),
            ReadColors::RgbF32(i)  => i.last().map(A::cast_rgb_f32),
            ReadColors::RgbaU8(i)  => i.last().map(A::cast_rgba_u8),
            ReadColors::RgbaU16(i) => i.last().map(A::cast_rgba_u16),
            ReadColors::RgbaF32(i) => i.last().map(A::cast_rgba_f32),
        }
    }

    fn count(self) -> usize {
        self.size_hint().0
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            ReadColors::RgbU8(ref i)   => i.size_hint(),
            ReadColors::RgbU16(ref i)  => i.size_hint(),
            ReadColors::RgbF32(ref i)  => i.size_hint(),
            ReadColors::RgbaU8(ref i)  => i.size_hint(),
            ReadColors::RgbaU16(ref i) => i.size_hint(),
            ReadColors::RgbaF32(ref i) => i.size_hint(),
        }
    }
}

impl Cast for RgbU8 {
    type Output = [u8; 3];

    fn cast_rgb_u8(x: [u8; 3]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgb_u16(x: [u16; 3]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgb_f32(x: [f32; 3]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgba_u8(x: [u8; 4]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgba_u16(x: [u16; 4]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgba_f32(x: [f32; 4]) -> Self::Output {
        x.into_rgb().normalize()
    }
}

impl Cast for RgbU16 {
    type Output = [u16; 3];

    fn cast_rgb_u8(x: [u8; 3]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgb_u16(x: [u16; 3]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgb_f32(x: [f32; 3]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgba_u8(x: [u8; 4]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgba_u16(x: [u16; 4]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgba_f32(x: [f32; 4]) -> Self::Output {
        x.into_rgb().normalize()
    }
}

impl Cast for RgbF32 {
    type Output = [f32; 3];

    fn cast_rgb_u8(x: [u8; 3]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgb_u16(x: [u16; 3]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgb_f32(x: [f32; 3]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgba_u8(x: [u8; 4]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgba_u16(x: [u16; 4]) -> Self::Output {
        x.into_rgb().normalize()
    }

    fn cast_rgba_f32(x: [f32; 4]) -> Self::Output {
        x.into_rgb().normalize()
    }
}

impl Cast for RgbaU8 {
    type Output = [u8; 4];

    fn cast_rgb_u8(x: [u8; 3]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgb_u16(x: [u16; 3]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgb_f32(x: [f32; 3]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgba_u8(x: [u8; 4]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgba_u16(x: [u16; 4]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgba_f32(x: [f32; 4]) -> Self::Output {
        x.normalize().into_rgba()
    }
}

impl Cast for RgbaU16 {
    type Output = [u16; 4];

    fn cast_rgb_u8(x: [u8; 3]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgb_u16(x: [u16; 3]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgb_f32(x: [f32; 3]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgba_u8(x: [u8; 4]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgba_u16(x: [u16; 4]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgba_f32(x: [f32; 4]) -> Self::Output {
        x.normalize().into_rgba()
    }
}

impl Cast for RgbaF32 {
    type Output = [f32; 4];

    fn cast_rgb_u8(x: [u8; 3]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgb_u16(x: [u16; 3]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgb_f32(x: [f32; 3]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgba_u8(x: [u8; 4]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgba_u16(x: [u16; 4]) -> Self::Output {
        x.normalize().into_rgba()
    }

    fn cast_rgba_f32(x: [f32; 4]) -> Self::Output {
        x.normalize().into_rgba()
    }
}
