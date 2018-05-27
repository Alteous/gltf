/// Casting iterator adapters for colors.
pub mod colors;

/// Casting iterator adapters for vertex indices.
pub mod indices;

/// Casting iterator adapters for joint indices.
pub mod joints;

/// Casting iterator adapters for texture co-ordinates.
pub mod tex_coords;

/// Casting iterator adapters for node weights.
pub mod weights;

use mesh;

use accessor::Iter;
use Buffer;

/// XYZ vertex positions of type `[f32; 3]`.
pub type ReadPositions<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex normals of type `[f32; 3]`.
pub type ReadNormals<'a> = Iter<'a, [f32; 3]>;

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
pub type ReadTangents<'a> = Iter<'a, [f32; 4]>;

/// XYZ vertex position displacements of type `[f32; 3]`.
pub type ReadPositionDisplacements<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex normal displacements of type `[f32; 3]`.
pub type ReadNormalDisplacements<'a> = Iter<'a, [f32; 3]>;

/// XYZ vertex tangent displacements.
pub type ReadTangentDisplacements<'a> = Iter<'a, [f32; 3]>;

/// Vertex colors.
#[derive(Clone, Debug)]
pub enum ReadColors<'a> {
    /// RGB vertex color of type `[u8; 3]>`.
    RgbU8(Iter<'a, [u8; 3]>),
    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(Iter<'a, [u16; 3]>),
    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(Iter<'a, [f32; 3]>),
    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(Iter<'a, [u8; 4]>),
    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(Iter<'a, [u16; 4]>),
    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(Iter<'a, [f32; 4]>),
}

/// Index data.
#[derive(Clone, Debug)]
pub enum ReadIndices<'a> {
    /// Index data of type U8
    U8(Iter<'a, u8>),
    /// Index data of type U16
    U16(Iter<'a, u16>),
    /// Index data of type U32
    U32(Iter<'a, u32>),
}

/// Vertex joints.
#[derive(Clone, Debug)]
pub enum ReadJoints<'a> {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(Iter<'a, [u8; 4]>),
    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(Iter<'a, [u16; 4]>),
}

/// UV texture co-ordinates.
#[derive(Clone, Debug)]
pub enum ReadTexCoords<'a> {
    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(Iter<'a, [u8; 2]>),
    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(Iter<'a, [u16; 2]>),
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(Iter<'a, [f32; 2]>),
}

/// Weights.
#[derive(Clone, Debug)]
pub enum ReadWeights<'a> {
    /// Weights of type `[u8; 4]`.
    U8(Iter<'a, [u8; 4]>),
    /// Weights of type `[u16; 4]`.
    U16(Iter<'a, [u16; 4]>),
    /// Weights of type `[f32; 4]`.
    F32(Iter<'a, [f32; 4]>),
}

/// Morph targets.
#[derive(Clone, Debug)]
pub struct ReadMorphTargets<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    pub(crate) index: usize,
    pub(crate) reader: mesh::Reader<'a, 's, F>,
}

impl<'a, 's, F> ExactSizeIterator for ReadMorphTargets<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{}

impl<'a, 's, F> Iterator for ReadMorphTargets<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    type Item = (
        Option<ReadPositionDisplacements<'s>>,
        Option<ReadNormalDisplacements<'s>>,
        Option<ReadTangentDisplacements<'s>>,
    );
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.reader.primitive
            .morph_targets()
            .nth(self.index - 1)
            .map(|morph_target| {
                let positions = if let Some(accessor) = morph_target.positions() {
                    let buffer = accessor.view().buffer();
                    if let Some(slice) = ((self.reader).get_buffer_data)(buffer) {
                        Some(Iter::new(accessor, slice))
                    } else {
                        None
                    }
                } else {
                    None
                };
                let normals = if let Some(accessor) = morph_target.normals() {
                    let buffer = accessor.view().buffer();
                    if let Some(slice) = ((self.reader).get_buffer_data)(buffer) {
                        Some(Iter::new(accessor, slice))
                    } else {
                        None
                    }
                } else {
                    None
                };
                let tangents = if let Some(accessor) = morph_target.tangents() {
                    let buffer = accessor.view().buffer();
                    if let Some(slice) = ((self.reader).get_buffer_data)(buffer) {
                        Some(Iter::new(accessor, slice))
                    } else {
                        None
                    }
                } else {
                    None
                };
                (positions, normals, tangents)
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.reader.primitive.morph_targets().size_hint()
    }
}

impl<'a> ReadColors<'a> {
    /// Reinterpret colors as RGB u8, discarding alpha, if present.  Lossy if
    /// the underlying iterator yields u16, f32 or any RGBA.
    pub fn into_rgb_u8(self) -> self::colors::CastingIter<'a, self::colors::RgbU8> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGB u16, discarding alpha, if present.  Lossy if
    /// the underlying iterator yields f32 or any RGBA.
    pub fn into_rgb_u16(self) -> self::colors::CastingIter<'a, self::colors::RgbU16> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGB f32, discarding alpha, if present.  Lossy if
    /// the underlying iterator yields u16 or any RGBA.
    pub fn into_rgb_f32(self) -> self::colors::CastingIter<'a, self::colors::RgbF32> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGBA u8, with default alpha 255.  Lossy if the
    /// underlying iterator yields u16 or f32.
    pub fn into_rgba_u8(self) -> self::colors::CastingIter<'a, self::colors::RgbaU8> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGBA u16, with default alpha 65535.  Lossy if the
    /// underlying iterator yields f32.
    pub fn into_rgba_u16(self) -> self::colors::CastingIter<'a, self::colors::RgbaU16> {
        self::colors::CastingIter::new(self)
    }

    /// Reinterpret colors as RGBA f32, with default alpha 1.0.  Lossy if the
    /// underlying iterator yields u16.
    pub fn into_rgba_f32(self) -> self::colors::CastingIter<'a, self::colors::RgbaF32> {
        self::colors::CastingIter::new(self)
    }
}

impl<'a> ReadIndices<'a> {
    /// Reinterpret indices as u32, which can fit any possible index.
    pub fn into_u32(self) -> self::indices::CastingIter<'a, self::indices::U32> {
        self::indices::CastingIter::new(self)
    }
}

impl<'a> ReadJoints<'a> {
    /// Reinterpret joints as u16, which can fit any possible joint.
    pub fn into_u16(self) -> self::joints::CastingIter<'a, self::joints::U16> {
        self::joints::CastingIter::new(self)
    }
}

impl<'a> ReadTexCoords<'a> {
    /// Reinterpret texture coordinates as u8.  Lossy if the underlying iterator
    /// yields u16 or f32.
    pub fn into_u8(self) -> self::tex_coords::CastingIter<'a, self::tex_coords::U8> {
        self::tex_coords::CastingIter::new(self)
    }

    /// Reinterpret texture coordinates as u16.  Lossy if the underlying
    /// iterator yields f32.
    pub fn into_u16(self) -> self::tex_coords::CastingIter<'a, self::tex_coords::U16> {
        self::tex_coords::CastingIter::new(self)
    }

    /// Reinterpret texture coordinates as f32.  Lossy if the underlying
    /// iterator yields u16.
    pub fn into_f32(self) -> self::tex_coords::CastingIter<'a, self::tex_coords::F32> {
        self::tex_coords::CastingIter::new(self)
    }
}

impl<'a> ReadWeights<'a> {
    /// Reinterpret weights as u8.  Lossy if the underlying iterator yields u16
    /// or f32.
    pub fn into_u8(self) -> self::weights::CastingIter<'a, self::weights::U8> {
        self::weights::CastingIter::new(self)
    }

    /// Reinterpret weights as u16.  Lossy if the underlying iterator yields
    /// f32.
    pub fn into_u16(self) -> self::weights::CastingIter<'a, self::weights::U16> {
        self::weights::CastingIter::new(self)
    }

    /// Reinterpret weights as f32.  Lossy if the underlying iterator yields
    /// u16.
    pub fn into_f32(self) -> self::weights::CastingIter<'a, self::weights::F32> {
        self::weights::CastingIter::new(self)
    }
}
