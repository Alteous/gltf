use serde::{de, ser};
use std::fmt;
use validation::{Checked, Error, Validate};
use {accessor, extensions, scene, Extras, Index, Path, Root};

/// All valid animation interpolation algorithms.
pub const VALID_INTERPOLATIONS: &'static [&'static str] = &[
    "LINEAR",
    "STEP",
    "CATMULLROMSPLINE",
    "CUBICSPLINE",
];

/// All valid animation property names.
pub const VALID_PROPERTIES: &'static [&'static str] = &[
    "translation",
    "rotation",
    "scale",
    "weights",
];

/// Specifies an interpolation algorithm.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Interpolation {
    /// Linear interpolation.
    ///
    /// The animated values are linearly interpolated between keyframes.
    /// When targeting a rotation, spherical linear interpolation (slerp) should be
    /// used to interpolate quaternions. The number output of elements must equal
    /// the number of input elements.
    Linear = 1,

    /// Step interpolation.
    ///
    /// The animated values remain constant to the output of the first keyframe,
    /// until the next keyframe. The number of output elements must equal the number
    /// of input elements.
    Step,

    /// Uniform Catmull-Rom spline interpolation.
    ///
    /// The animation's interpolation is computed using a uniform Catmull-Rom spline.
    /// The number of output elements must equal two more than the number of input
    /// elements. The first and last output elements represent the start and end
    /// tangents of the spline. There must be at least four keyframes when using this
    /// interpolation.
    CatmullRomSpline,

    /// Cubic spline interpolation.
    ///
    /// The animation's interpolation is computed using a uniform Catmull-Rom spline.
    /// The number of output elements must equal two more than the number of input
    /// elements. The first and last output elements represent the start and end
    /// tangents of the spline. There must be at least four keyframes when using this
    /// interpolation.
    CubicSpline,
}

/// Specifies a property to animate.
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Property {
    /// XYZ translation vector.
    Translation = 1,

    /// XYZW rotation quaternion.
    Rotation,

    /// XYZ scale vector.
    Scale,

    /// Weights of morph targets.
    MorphTargetWeights,
}

/// A keyframe animation.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Animation {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::animation::Animation,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// An array of channels, each of which targets an animation's sampler at a
    /// node's property.
    ///
    /// Different channels of the same animation must not have equal targets.
    pub channels: Vec<Channel>,
    
    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,
    
    /// An array of samplers that combine input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub samplers: Vec<Sampler>,
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
    /// The index of a sampler in this animation used to compute the value for the
    /// target.
    pub sampler: Index<Sampler>,
    
    /// The index of the node and TRS property to target.
    pub target: Target,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::animation::Channel,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// The index of the node and TRS property that an animation channel targets.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Target {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::animation::Target,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of the node to target.
    pub node: Index<scene::Node>,
    
    /// The name of the node's property to modify or the 'weights' of the
    /// morph targets it instantiates.
    pub path: Checked<Property>,
}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Sampler {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::animation::Sampler,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of an accessor containing keyframe input values, e.g., time.
    pub input: Index<accessor::Accessor>,
    
    /// The interpolation algorithm.
    #[serde(default)]
    pub interpolation: Checked<Interpolation>,
    
    /// The index of an accessor containing keyframe output values.
    pub output: Index<accessor::Accessor>,
}

impl Validate for Animation {
    fn validate_minimally<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&Fn() -> Path, Error),
    {
        self.samplers.validate_minimally(root, || path().field("samplers"), report);
        for (index, channel) in self.channels.iter().enumerate() {
            if channel.sampler.value() as usize >= self.samplers.len() {
                let path = || path().field("channels").index(index).field("sampler");
                report(&path, Error::IndexOutOfBounds);
            }
        }
    }
}

impl Default for Interpolation {
    fn default() -> Self {
        Interpolation::Linear
    }
}

impl<'de> de::Deserialize<'de> for Checked<Interpolation> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Interpolation>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_INTERPOLATIONS)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::Interpolation::*;
                use validation::Checked::*;
                Ok(match value {
                    "LINEAR" => Valid(Linear),
                    "STEP" => Valid(Step),
                    "CATMULLROMSPLINE" => Valid(CatmullRomSpline),
                    "CUBICSPLINE" => Valid(CubicSpline),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl ser::Serialize for Interpolation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer
    {
        serializer.serialize_str(match *self {
            Interpolation::Linear => "LINEAR",
            Interpolation::Step => "STEP",
            Interpolation::CatmullRomSpline => "CATMULLROMSPLINE",
            Interpolation::CubicSpline => "CUBICSPLINE",
        })
    }
}

impl<'de> de::Deserialize<'de> for Checked<Property> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Property>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_PROPERTIES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::Property::*;
                use validation::Checked::*;
                Ok(match value {
                    "translation" => Valid(Translation),
                    "rotation" => Valid(Rotation),
                    "scale" => Valid(Scale),
                    "weights" => Valid(MorphTargetWeights),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl ser::Serialize for Property {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer
    {
        serializer.serialize_str(match *self {
            Property::Translation => "translation",
            Property::Rotation => "rotation",
            Property::Scale => "scale",
            Property::MorphTargetWeights => "weights",
        })
    }
}
