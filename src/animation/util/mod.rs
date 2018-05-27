/// Casting iterator adapters for rotations.
pub mod rotations;

/// Casting iterator adapters for morph target weights.
pub mod morph_target_weights;

use accessor;

/// Animation input sampler values of type `f32`.
pub type ReadInputs<'a> = accessor::Iter<'a, f32>;

/// Animation output sampler values of type `[f32; 3]`.
pub type Translations<'a> = accessor::Iter<'a, [f32; 3]>;

/// Animation output sampler values of type `[f32; 3]`.
pub type Scales<'a> = accessor::Iter<'a, [f32; 3]>;

/// Rotation animations
#[derive(Clone, Debug)]
pub enum Rotations<'a> {
    /// Rotations of type `[i8; 4]`.
    I8(accessor::Iter<'a, [i8; 4]>),
    /// Rotations of type `[u8; 4]`.
    U8(accessor::Iter<'a, [u8; 4]>),
    /// Rotations of type `[i16; 4]`.
    I16(accessor::Iter<'a, [i16; 4]>),
    /// Rotations of type `[u16; 4]`.
    U16(accessor::Iter<'a, [u16; 4]>),
    /// Rotations of type `[f32; 4]`.
    F32(accessor::Iter<'a, [f32; 4]>),
}

/// Morph-target weight animations.
#[derive(Clone, Debug)]
pub enum MorphTargetWeights<'a> {
    /// Weights of type `i8`.
    I8(accessor::Iter<'a, i8>),
    /// Weights of type `u8`.
    U8(accessor::Iter<'a, u8>),
    /// Weights of type `i16`.
    I16(accessor::Iter<'a, i16>),
    /// Weights of type `u16`.
    U16(accessor::Iter<'a, u16>),
    /// Weights of type `f32`.
    F32(accessor::Iter<'a, f32>),
}

/// Animation output sampler values.
pub enum ReadOutputs<'a> {
    /// XYZ translations of type `[f32; 3]`.
    Translations(Translations<'a>),

    /// Rotation animations.
    Rotations(Rotations<'a>),

    /// XYZ scales of type `[f32; 3]`.
    Scales(Scales<'a>),

    /// Morph target animations.
    MorphTargetWeights(MorphTargetWeights<'a>),
}

impl<'a> Rotations<'a> {
    /// Reinterpret rotations as u16.  Lossy if underlying iterator yields u8,
    /// i16, u16 or f32.
    pub fn into_i8(self) -> rotations::CastingIter<'a, rotations::I8> {
        rotations::CastingIter::new(self)
    }

    /// Reinterpret rotations as u16.  Lossy if underlying iterator yields i16,
    /// u16 or f32.
    pub fn into_u8(self) -> rotations::CastingIter<'a, rotations::U8> {
        rotations::CastingIter::new(self)
    }

    /// Reinterpret rotations as u16.  Lossy if underlying iterator yields u16
    /// or f32.
    pub fn into_i16(self) -> rotations::CastingIter<'a, rotations::I16> {
        rotations::CastingIter::new(self)
    }

    /// Reinterpret rotations as u16.  Lossy if underlying iterator yields f32.
    pub fn into_u16(self) -> rotations::CastingIter<'a, rotations::U16> {
        rotations::CastingIter::new(self)
    }

    /// Reinterpret rotations as f32.  Lossy if underlying iterator yields i16
    /// or u16.
    pub fn into_f32(self) -> rotations::CastingIter<'a, rotations::F32> {
        rotations::CastingIter::new(self)
    }
}

impl<'a> MorphTargetWeights<'a> {
    /// Reinterpret morph weights as u16.  Lossy if underlying iterator yields
    /// u8, i16, u16 or f32.
    pub fn into_i8(self) -> morph_target_weights::CastingIter<'a, morph_target_weights::I8> {
        morph_target_weights::CastingIter::new(self)
    }

    /// Reinterpret morph weights as u16.  Lossy if underlying iterator yields
    /// i16, u16 or f32.
    pub fn into_u8(self) -> morph_target_weights::CastingIter<'a, morph_target_weights::U8> {
        morph_target_weights::CastingIter::new(self)
    }

    /// Reinterpret morph weights as u16.  Lossy if underlying iterator yields
    /// u16 or f32.
    pub fn into_i16(self) -> morph_target_weights::CastingIter<'a, morph_target_weights::I16> {
        morph_target_weights::CastingIter::new(self)
    }

    /// Reinterpret morph weights as u16.  Lossy if underlying iterator yields
    /// f32.
    pub fn into_u16(self) -> morph_target_weights::CastingIter<'a, morph_target_weights::U16> {
        morph_target_weights::CastingIter::new(self)
    }

    /// Reinterpret morph weights as f32.  Lossy if underlying iterator yields
    /// i16 or u16.
    pub fn into_f32(self) -> morph_target_weights::CastingIter<'a, morph_target_weights::F32> {
        morph_target_weights::CastingIter::new(self)
    }
}
