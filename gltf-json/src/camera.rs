use serde::{de, ser};
use std::fmt;
use validation::{Checked, Error, Validate};
use {extensions, Extras, Root, Path};

/// All valid camera types.
pub const VALID_CAMERA_TYPES: &'static [&'static str] = &[
    "perspective",
    "orthographic",
];

/// Specifies the camera type.
#[derive(Clone, Copy, Debug)]
pub enum Type {
    /// A perspective projection.
    Perspective = 1,

    /// An orthographic projection.
    Orthographic,
}

/// A camera's projection.
///
/// A node can reference a camera to apply a transform to place the camera in the
/// scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Camera {
    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,

    /// An orthographic camera containing properties to create an orthographic
    /// projection matrix.
    pub orthographic: Option<Orthographic>,

    /// A perspective camera containing properties to create a perspective
    /// projection matrix.
    pub perspective: Option<Perspective>,

    /// Specifies if the camera uses a perspective or orthographic projection.
    #[serde(rename = "type")]
    pub type_: Checked<Type>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::camera::Camera,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Values for an orthographic camera.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Orthographic {
    /// The horizontal magnification of the view.
    pub xmag: f32,

    /// The vertical magnification of the view.
    pub ymag: f32,

    /// The distance to the far clipping plane.
    pub zfar: f32,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::camera::Orthographic,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Values for a perspective camera.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Perspective {
    /// Aspect ratio of the field of view.
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: Option<f32>,

    /// The vertical field of view in radians.
    pub yfov: f32,

    /// The distance to the far clipping plane.
    pub zfar: Option<f32>,

    /// The distance to the near clipping plane.
    pub znear: f32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::camera::Perspective,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

impl Validate for Camera {
    fn validate_minimally<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        if self.orthographic.is_none() && self.perspective.is_none() {
            report(&path, Error::Missing);
        }

        self.orthographic
            .validate_minimally(root, || path().field("orthographic"), report);
        self.perspective
            .validate_minimally(root, || path().field("perspective"), report);
        self.type_
            .validate_minimally(root, || path().field("type"), report);
        self.extensions
            .validate_minimally(root, || path().field("extensions"), report);
        self.extras
            .validate_minimally(root, || path().field("extras"), report);
    }
}

impl Validate for Orthographic {
    fn validate_completely<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        if self.znear < 0.0 {
            report(&path, Error::Invalid);
        }
 
        if self.zfar < 0.0  || self.zfar < self.znear {
            report(&path, Error::Invalid);
        }

        self.extensions
            .validate_completely(root, || path().field("extensions"), report);
        self.extras
            .validate_completely(root, || path().field("extras"), report);
    }
}

impl Validate for Perspective {
    fn validate_completely<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        self.aspect_ratio.map(|aspect_ratio| {
            if aspect_ratio < 0.0 {
                report(&path, Error::Invalid);
            }
        });

        if self.yfov < 0.0 {
            report(&path, Error::Invalid);
        }

        if self.znear < 0.0 {
            report(&path, Error::Invalid);
        }

        self.zfar.map(|zfar| {
            if zfar < 0.0 || zfar < self.znear {
                report(&path, Error::Invalid);
            }
        });

        self.extensions
            .validate_completely(root, || path().field("extensions"), report);
        self.extras
            .validate_completely(root, || path().field("extras"), report);
    }
}

impl<'de> de::Deserialize<'de> for Checked<Type> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Type>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_CAMERA_TYPES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::Type::*;
                use validation::Checked::*;
                Ok(match value {
                    "perspective" => Valid(Perspective),
                    "orthographic" => Valid(Orthographic),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl ser::Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        match *self {
            Type::Perspective => serializer.serialize_str("perspective"),
            Type::Orthographic => serializer.serialize_str("orthographic"),
        }
    }
}
