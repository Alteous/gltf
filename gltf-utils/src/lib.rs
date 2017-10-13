
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate byteorder;
extern crate gltf;

use std::{fmt, io, marker, mem};

use gltf::accessor::{DataType, Dimensions};

/// Marker trait for data types able to be read from an accessor.
pub trait AccessorData {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt, Self: Sized;
}

impl AccessorData for i8 {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        reader.read_i8()
    }
}

impl AccessorData for u8 {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        reader.read_u8()
    }
} 

impl AccessorData for i16 {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        reader.read_i16::<B>()
    }
}

impl AccessorData for u16 {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        reader.read_u16::<B>()
    }
}

impl AccessorData for f32 {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        reader.read_f32::<B>()
    }
}

impl AccessorData for i32 {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        reader.read_i32::<B>()
    }
}

impl AccessorData for u32 {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        reader.read_u32::<B>()
    }
}

impl<T: AccessorData> AccessorData for [T; 1] {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        Ok([T::read::<B, R>(reader)?])
    }
}

impl<T: AccessorData> AccessorData for [T; 2] {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        Ok([T::read::<B, R>(reader)?, T::read::<B, R>(reader)?])
    }

}

impl<T: AccessorData> AccessorData for [T; 3] {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        Ok([T::read::<B, R>(reader)?, T::read::<B, R>(reader)?, T::read::<B, R>(reader)?])
    }

}

impl<T: AccessorData> AccessorData for [T; 4] {
    fn read<B, R>(reader: &mut R) -> io::Result<Self>
        where B: byteorder::ByteOrder, R: byteorder::ReadBytesExt
    {
        Ok([T::read::<B, R>(reader)?, T::read::<B, R>(reader)?, T::read::<B, R>(reader)?, T::read::<B, R>(reader)?])
    }

}

/// Helper trait for denormalizing integer types.
///
/// # Examples
///
/// Denormalize a single `u16`.
///
/// ```rust
/// use gltf_utils::Denormalize;
/// let x: u16 = 65535;
/// assert_eq!(1.0, x.denormalize());
/// ```
///
/// Denormalize an array of integers.
///
/// ```rust
/// use gltf_utils::Denormalize;
/// let rgb: [u8; 3] = [0, 120, 255];
/// assert_eq!([0.0, 120.0 / 255.0, 1.0], rgb.denormalize());
/// ```
pub trait Denormalize {
    /// The denormalized version of this type.
    type Denormalized;

    /// Returns the denormalized equivalent of the value.
    fn denormalize(&self) -> Self::Denormalized;
}

/// Represents sources of buffer data.
///
/// See the `Buffers` type in the `gltf-importer` crate for the reference
/// implementation.
pub trait Source: fmt::Debug {
    /// Return the buffer data referenced by the given `Buffer`.
    ///
    /// This method must not fail.
    fn source_buffer(&self, buffer: &gltf::Buffer) -> &[u8];
}

/// Extra methods for working with `gltf::Primitive`.
pub trait PrimitiveIterators<'a> {
    /// Visits the vertex positions of a primitive.
    fn positions<S>(&'a self, source: &'a S) -> Option<Positions<'a>>
        where S: Source;

    /// Visits the vertex normals of a primitive.
    fn normals<S>(&'a self, source: &'a S) -> Option<Normals<'a>>
        where S: Source;
    
    /// Visits the vertex tangents of a primitive.
    fn tangents<S>(&'a self, source: &'a S) -> Option<Tangents<'a>>
        where S: Source;
    
    /// Visits the vertex texture co-ordinates of a primitive.
    fn tex_coords_f32<S>(
        &'a self,
        set: u32,
        source: &'a S,
    ) -> Option<TexCoordsF32<'a>>
        where S: Source;
    
    /// Visits the vertex colors of a primitive.
    fn colors_rgba_f32<S>(
        &'a self,
        set: u32,
        default_alpha: f32,
        source: &'a S,
    ) -> Option<ColorsRgbaF32<'a>>
        where S: Source;

    /// Visits the vertex draw sequence of a primitive.
    fn indices_u32<S>(&'a self, source: &'a S) -> Option<IndicesU32<'a>>
        where S: Source;

    /// Visits the joint indices of the primitive.
    fn joints_u16<S: Source>(&'a self, set: u32, source: &'a S) -> Option<JointsU16<'a>>
        where S: Source;

    /// Visits the joint weights of the primitive.
    fn weights_f32<S: Source>(&'a self, set: u32, source: &'a S) -> Option<WeightsF32<'a>>
        where S: Source;
}

impl<'a> PrimitiveIterators<'a> for gltf::Primitive<'a> {
    fn positions<S>(&'a self, source: &'a S) -> Option<Positions<'a>>
        where S: Source
    {
        self.get(&gltf::Semantic::Positions)
            .map(|accessor| Positions(AccessorIter::new(accessor, source)))
    }

    fn normals<S>(&'a self, source: &'a S) -> Option<Normals<'a>>
        where S: Source
    {
        self.get(&gltf::Semantic::Normals)
            .map(|accessor| Normals(AccessorIter::new(accessor, source)))
    }

    fn tangents<S>(&'a self, source: &'a S) -> Option<Tangents<'a>>
        where S: Source
    {
        self.get(&gltf::Semantic::Tangents)
            .map(|accessor| Tangents(AccessorIter::new(accessor, source)))
    }

    fn tex_coords_f32<S>(&'a self, set: u32, source: &'a S) -> Option<TexCoordsF32<'a>>
        where S: Source
    {
        self.get(&gltf::Semantic::TexCoords(set))
            .map(|accessor| TexCoordsF32(TexCoords::new(accessor, source)))
    }

    fn colors_rgba_f32<S>(
        &'a self,
        set: u32,
        default_alpha: f32,
        source: &'a S,
    ) -> Option<ColorsRgbaF32<'a>>
        where S: Source
    {
        self.get(&gltf::Semantic::Colors(set))
            .map(|accessor| {
                ColorsRgbaF32 {
                    iter: Colors::new(accessor, source),
                    default_alpha,
                }
            })
    }

    fn indices_u32<S>(&'a self, source: &'a S) -> Option<IndicesU32<'a>>
        where S: Source
    {
        self.indices().map(|accessor| IndicesU32(Indices::new(accessor, source)))
    }
    
    fn joints_u16<S>(&'a self, set: u32, source: &'a S) -> Option<JointsU16<'a>>
        where S: Source
    {
        self.get(&gltf::Semantic::Colors(set))
            .map(|accessor| JointsU16(Joints::new(accessor, source)))
    }

    fn weights_f32<S>(&'a self, set: u32, source: &'a S) -> Option<WeightsF32<'a>>
        where S: Source
    {
        self.get(&gltf::Semantic::Weights(set))
            .map(|accessor| WeightsF32(Weights::new(accessor, source)))
    }
}

/// Visits the items in an `Accessor`.
#[derive(Clone, Debug)]
pub struct AccessorIter<'a, T: AccessorData> {
    /// The total number of iterations left.
    count: usize,

    /// The index of the next iteration.
    index: usize,

    /// The number of bytes to advance between each iteration.
    advance: i64,

    /// The data we're iterating over.
    cursor: io::Cursor<&'a [u8]>,

    /// The accessor we're iterating over.
    accessor: gltf::Accessor<'a>,

    /// Consumes the data type we're returning at each iteration.
    _marker: marker::PhantomData<T>,
}

impl<'a, T: AccessorData> AccessorIter<'a, T> {
    pub fn new<S>(accessor: gltf::Accessor<'a>, source: &'a S) -> AccessorIter<'a, T>
        where S: Source,
    {
        assert_eq!(mem::size_of::<T>(), accessor.size());
        assert!(mem::align_of::<T>() <= 4);
        let view = accessor.view();
        let buffer = view.buffer();
        let buffer_data = source.source_buffer(&buffer);
        let view_data = &buffer_data[view.offset()..(view.offset() + view.length())];
        let mut cursor = io::Cursor::new(view_data);
        cursor.set_position(accessor.offset() as u64);
        let advance = view.stride().unwrap_or(mem::size_of::<T>()) as i64 - accessor.size() as i64;
        let count = accessor.count();
        AccessorIter {
            accessor,
            advance,
            count,
            cursor,
            index: 0,
            _marker: marker::PhantomData,
        }
    }
}

/// XYZ vertex normals of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct Normals<'a>(AccessorIter<'a, [f32; 3]>);

/// XYZ vertex positions of type `[f32; 3]`.
#[derive(Clone, Debug)]
pub struct Positions<'a>(AccessorIter<'a, [f32; 3]>);

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
#[derive(Clone, Debug)]
pub struct Tangents<'a>(AccessorIter<'a, [f32; 4]>);

/// Vertex colors.
#[derive(Clone, Debug)]
enum Colors<'a> {
    /// RGB vertex color of type `[u8; 3]>`.
    RgbU8(AccessorIter<'a, [u8; 3]>),

    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(AccessorIter<'a, [u8; 4]>),

    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(AccessorIter<'a, [u16; 3]>),

    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(AccessorIter<'a, [u16; 4]>),

    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(AccessorIter<'a, [f32; 3]>),

    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(AccessorIter<'a, [f32; 4]>),
}

/// Index data.
#[derive(Clone, Debug)]
enum Indices<'a> {
    /// Index data of type U8
    U8(AccessorIter<'a, u8>),
    /// Index data of type U16
    U16(AccessorIter<'a, u16>),
    /// Index data of type U32
    U32(AccessorIter<'a, u32>),
}

/// Vertex joints.
#[derive(Clone, Debug)]
enum Joints<'a> {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(AccessorIter<'a, [u8; 4]>),
    
    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(AccessorIter<'a, [u16; 4]>),
}

/// UV texture co-ordinates.
#[derive(Clone, Debug)]
enum TexCoords<'a> {
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(AccessorIter<'a, [f32; 2]>),

    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(AccessorIter<'a, [u8; 2]>),

    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(AccessorIter<'a, [u16; 2]>),
}

/// Weights,
#[derive(Clone, Debug)]
enum Weights<'a> {
    /// Weights of type `[f32; 4]`.
    F32(AccessorIter<'a, [f32; 4]>),

    /// Weights of type `[u8; 4]`.
    U8(AccessorIter<'a, [u8; 4]>),

    /// Weights of type `[u16; 4]`.
    U16(AccessorIter<'a, [u16; 4]>),
}

/// Index data coerced into `u32` values.
#[derive(Clone, Debug)]
pub struct IndicesU32<'a>(Indices<'a>);

/// Texture co-ordinates coerced into `[f32; 2]` values.
#[derive(Clone, Debug)]
pub struct TexCoordsF32<'a>(TexCoords<'a>);

/// Joint indices co-coerced into `[u16; 4]` values.
#[derive(Clone, Debug)]
pub struct JointsU16<'a>(Joints<'a>);

/// Joint weights co-coerced into `[f32; 4]` values.
#[derive(Clone, Debug)]
pub struct WeightsF32<'a>(Weights<'a>);

/// Vertex colors coerced into `[f32; 4]` (RGBA) values.
#[derive(Clone, Debug)]
pub struct ColorsRgbaF32<'a> {
    /// Internal iterator type.
    iter: Colors<'a>,

    /// Default alpha value.
    default_alpha: f32,
}

impl<'a, T: AccessorData> ExactSizeIterator for AccessorIter<'a, T> {}
impl<'a, T: AccessorData> Iterator for AccessorIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        use io::Seek;
        if self.index < self.count {
            let value = T::read::<byteorder::LE, _>(&mut self.cursor)
                .expect("accessor read error");
            let _ = self.cursor
                .seek(io::SeekFrom::Current(self.advance))
                .expect("accessor seek error");
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.count - self.index;
        (hint, Some(hint))
    }
}

impl<'a> Colors<'a> {
    fn new<S: Source>(accessor: gltf::Accessor<'a>, source: &'a S) -> Colors<'a> {
        match (accessor.dimensions(), accessor.data_type()) {
            (Dimensions::Vec3, DataType::U8) => {
                Colors::RgbU8(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec4, DataType::U8) => {
                Colors::RgbaU8(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec3, DataType::U16) => {
                Colors::RgbU16(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec4, DataType::U16) => {
                Colors::RgbaU16(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec3, DataType::F32) => {
                Colors::RgbF32(AccessorIter::new(accessor, source))
            },
            (Dimensions::Vec4, DataType::F32) => {
                Colors::RgbaF32(AccessorIter::new(accessor, source))
            },
            _ => unreachable!(),
        }
    }
}
impl<'a> TexCoords<'a> {
    fn new<S: Source>(accessor: gltf::Accessor<'a>, source: &'a S) -> TexCoords<'a> {
        match accessor.data_type() {
            DataType::U8 => TexCoords::U8(AccessorIter::new(accessor, source)),
            DataType::U16 => TexCoords::U16(AccessorIter::new(accessor, source)),
            DataType::F32 => TexCoords::F32(AccessorIter::new(accessor, source)),
            _ => unreachable!(),
        }
    }
}

impl<'a> Indices<'a> {
    fn new<S: Source>(accessor: gltf::Accessor<'a>, source: &'a S) -> Indices<'a> {
        match accessor.data_type() {
            DataType::U8 => Indices::U8(AccessorIter::new(accessor, source)),
            DataType::U16 => Indices::U16(AccessorIter::new(accessor, source)),
            DataType::U32 => Indices::U32(AccessorIter::new(accessor, source)),
            _ => unreachable!(),
        }
    }
}

impl<'a> Joints<'a> {
    fn new<S: Source>(accessor: gltf::Accessor<'a>, source: &'a S) -> Joints<'a> {
        match accessor.data_type() {
            DataType::U8 => Joints::U8(AccessorIter::new(accessor, source)),
            DataType::U16 => Joints::U16(AccessorIter::new(accessor, source)),
            _ => unreachable!(),
        }
    }
}

impl<'a> Weights<'a> {
    fn new<S: Source>(accessor: gltf::Accessor<'a>, source: &'a S) -> Weights<'a> {
        match accessor.data_type() {
            DataType::U8 => Weights::U8(AccessorIter::new(accessor, source)),
            DataType::U16 => Weights::U16(AccessorIter::new(accessor, source)),
            DataType::F32 => Weights::F32(AccessorIter::new(accessor, source)),
            _ => unreachable!(),
        }
    }
}

impl<'a> ExactSizeIterator for IndicesU32<'a> {}
impl<'a> Iterator for IndicesU32<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Indices::U8(ref mut i) => i.next().map(|x| x as u32),
            Indices::U16(ref mut i) => i.next().map(|x| x as u32),
            Indices::U32(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Indices::U8(ref i) => i.size_hint(),
            Indices::U16(ref i) => i.size_hint(),
            Indices::U32(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for JointsU16<'a> {}
impl<'a> Iterator for JointsU16<'a> {
    type Item = [u16; 4];
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Joints::U8(ref mut i) => {
                i.next()
                    .map(|x| [x[0] as u16, x[1] as u16, x[2] as u16, x[3] as u16])
            },
            Joints::U16(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Joints::U8(ref i) => i.size_hint(),
            Joints::U16(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for ColorsRgbaF32<'a> {}
impl<'a> Iterator for ColorsRgbaF32<'a> {
    type Item = [f32; 4];
    fn next(&mut self) -> Option<Self::Item> {
        let default_alpha = self.default_alpha;
        match self.iter {
            Colors::RgbU8(ref mut i) => {
                i.next().map(|x| {
                    let rgb = x.denormalize();
                    [rgb[0], rgb[1], rgb[2], default_alpha]
                })
            },
            Colors::RgbU16(ref mut i) => {
                i.next().map(|x| {
                    let rgb = x.denormalize();
                    [rgb[0], rgb[1], rgb[2], default_alpha]
                })
            },
            Colors::RgbF32(ref mut i) => {
                i.next().map(|rgb| [rgb[0], rgb[1], rgb[2], default_alpha])
            },
            Colors::RgbaU8(ref mut i) => i.next().map(|x| x.denormalize()),
            Colors::RgbaU16(ref mut i) => i.next().map(|x| x.denormalize()),
            Colors::RgbaF32(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.iter {
            Colors::RgbU8(ref i) => i.size_hint(),
            Colors::RgbU16(ref i) => i.size_hint(),
            Colors::RgbF32(ref i) => i.size_hint(),
            Colors::RgbaU8(ref i) => i.size_hint(),
            Colors::RgbaU16(ref i) => i.size_hint(),
            Colors::RgbaF32(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for TexCoordsF32<'a> {}
impl<'a> Iterator for TexCoordsF32<'a> {
    type Item = [f32; 2];
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            TexCoords::U8(ref mut i) => i.next().map(|x| x.denormalize()),
            TexCoords::U16(ref mut i) => i.next().map(|x| x.denormalize()),
            TexCoords::F32(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            TexCoords::U8(ref i) => i.size_hint(),
            TexCoords::U16(ref i) => i.size_hint(),
            TexCoords::F32(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for WeightsF32<'a> {}
impl<'a> Iterator for WeightsF32<'a> {
    type Item = [f32; 4];
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Weights::U8(ref mut i) => i.next().map(|x| x.denormalize()),
            Weights::U16(ref mut i) => i.next().map(|x| x.denormalize()),
            Weights::F32(ref mut i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Weights::U8(ref i) => i.size_hint(),
            Weights::U16(ref i) => i.size_hint(),
            Weights::F32(ref i) => i.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for Positions<'a> {}
impl<'a> Iterator for Positions<'a> {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Normals<'a> {}
impl<'a> Iterator for Normals<'a> {
    type Item = [f32; 3];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
 
impl<'a> ExactSizeIterator for Tangents<'a> {}
impl<'a> Iterator for Tangents<'a> {
    type Item = [f32; 4];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl Denormalize for u8 {
    type Denormalized = f32;
    fn denormalize(&self) -> Self::Denormalized {
        *self as f32 / 255.0
    }
}

impl Denormalize for u16 {
    type Denormalized = f32;
    fn denormalize(&self) -> Self::Denormalized {
        *self as f32 / 65535.0
    }
}

impl<T: Copy + Denormalize> Denormalize for [T; 2] {
    type Denormalized = [T::Denormalized; 2];
    fn denormalize(&self) -> Self::Denormalized {
        [
            self[0].denormalize(),
            self[1].denormalize(),
        ]
    }
}

impl<T: Copy + Denormalize> Denormalize for [T; 3] {
    type Denormalized = [T::Denormalized; 3];
    fn denormalize(&self) -> Self::Denormalized {
        [
            self[0].denormalize(),
            self[1].denormalize(),
            self[2].denormalize(),
        ]
    }
}

impl<T: Copy + Denormalize> Denormalize for [T; 4] {
    type Denormalized = [T::Denormalized; 4];
    fn denormalize(&self) -> Self::Denormalized {
        [
            self[0].denormalize(),
            self[1].denormalize(),
            self[2].denormalize(),
            self[3].denormalize(),
        ]
    }
}
