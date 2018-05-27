use serde::{de, ser};
use serde_json::from_value;
use std::collections::HashMap;
use std::fmt;
use validation::{Checked, Error, Validate};
use {accessor, extensions, material, Extras, Index};

/// Corresponds to `GL_POINTS`.
pub const POINTS: u32 = 0;

/// Corresponds to `GL_LINES`.
pub const LINES: u32 = 1;

/// Corresponds to `GL_LINE_LOOP`.
pub const LINE_LOOP: u32 = 2;

/// Corresponds to `GL_LINE_STRIP`.
pub const LINE_STRIP: u32 = 3;

/// Corresponds to `GL_TRIANGLES`.
pub const TRIANGLES: u32 = 4;

/// Corresponds to `GL_TRIANGLE_STRIP`.
pub const TRIANGLE_STRIP: u32 = 5;

/// Corresponds to `GL_TRIANGLE_FAN`.
pub const TRIANGLE_FAN: u32 = 6;

/// All valid primitive rendering modes.
pub const VALID_MODES: &'static [u32] = &[
    POINTS,
    LINES,
    LINE_LOOP,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN,
];

/// All valid semantic names for Morph targets.
pub const VALID_MORPH_TARGETS: &'static [&'static str] = &[
    "POSITION",
    "NORMAL",
    "TANGENT",
];

/// The type of primitives to render.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum Mode {
    /// Corresponds to `GL_POINTS`.
    Points = 1,

    /// Corresponds to `GL_LINES`.
    Lines,

    /// Corresponds to `GL_LINE_LOOP`.
    LineLoop,

    /// Corresponds to `GL_LINE_STRIP`.
    LineStrip,

    /// Corresponds to `GL_TRIANGLES`.
    Triangles,

    /// Corresponds to `GL_TRIANGLE_STRIP`.
    TriangleStrip,

    /// Corresponds to `GL_TRIANGLE_FAN`.
    TriangleFan,
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Mesh {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::mesh::Mesh,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub name: Option<String>,

    /// Defines the geometry to be renderered with a material.
    pub primitives: Vec<Primitive>,

    /// Defines the weights to be applied to the morph targets.
    pub weights: Option<Vec<f32>>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Primitive {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub attributes: HashMap<Checked<Semantic>, Index<accessor::Accessor>>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: extensions::mesh::Primitive,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,

    /// The index of the accessor that contains the indices.
    pub indices: Option<Index<accessor::Accessor>>,

    /// The index of the material to apply to this primitive when rendering
    pub material: Option<Index<material::Material>>,

    /// The type of primitives to render.
    #[serde(default)]
    pub mode: Checked<Mode>,

    /// An array of Morph Targets, each  Morph Target is a dictionary mapping
    /// attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their
    /// deviations in the Morph Target.
    pub targets: Option<Vec<MorphTarget>>,
}

    impl Validate for Primitive {
        fn validate_minimally<P, R>(&self, root: &::Root, path: P, report: &mut R)
        where
            P: Fn() -> ::Path,
            R: FnMut(&Fn() -> ::Path, ::validation::Error),
        {
            // Generated part
            self.attributes
                .validate_minimally(root, || path().field("attributes"), report);
            self.extensions
                .validate_minimally(root, || path().field("extensions"), report);
            self.extras
                .validate_minimally(root, || path().field("extras"), report);
            self.indices
                .validate_minimally(root, || path().field("indices"), report);
            self.material
                .validate_minimally(root, || path().field("material"), report);
            self.mode
                .validate_minimally(root, || path().field("mode"), report);
            self.targets
                .validate_minimally(root, || path().field("targets"), report);

            // Custom part
            let position_path = &|| path().field("attributes").key("POSITION");
            if let Some(pos_accessor_index) = self.attributes.get(&Checked::Valid(Semantic::Positions)) {
                // spec: POSITION accessor **must** have `min` and `max` properties defined.
                let pos_accessor = &root.accessors[pos_accessor_index.value()];

                let min_path = &|| position_path().field("min");
                if let Some(ref min) = pos_accessor.min {
                    if from_value::<[f32; 3]>(min.clone()).is_err() {
                        report(min_path, Error::Invalid);
                    }
                } else {
                    report(min_path, Error::Missing);
                }

                let max_path = &|| position_path().field("max");
                if let Some(ref max) = pos_accessor.max {
                    if from_value::<[f32; 3]>(max.clone()).is_err() {
                        report(max_path, Error::Invalid);
                    }
                } else {
                    report(max_path, Error::Missing);
                }
            } else {
                report(position_path, Error::Missing);
            }
        }
    }

/// A dictionary mapping attributes to their deviations in the Morph Target.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct MorphTarget {
    /// XYZ vertex position displacements of type `[f32; 3]`.
    #[serde(rename = "POSITION")]
    pub positions: Option<Index<accessor::Accessor>>,

    /// XYZ vertex normal displacements of type `[f32; 3]`.
    #[serde(rename = "NORMAL")]
    pub normals: Option<Index<accessor::Accessor>>,

    /// XYZ vertex tangent displacements of type `[f32; 3]`.
    #[serde(rename = "TANGENT")]
    pub tangents: Option<Index<accessor::Accessor>>,
}

/// Vertex attribute semantic name.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Semantic {
    /// Extra attribute name.
    #[cfg(feature = "extras")]
    Extras(String),

    /// XYZ vertex positions.
    Positions,

    /// XYZ vertex normals.
    Normals,

    /// XYZW vertex tangents where the `w` component is a sign value indicating the
    /// handedness of the tangent basis.
    Tangents,

    /// RGB or RGBA vertex color.
    Colors(u32),

    /// UV texture co-ordinates.
    TexCoords(u32),

    /// Joint indices.
    Joints(u32),

    /// Joint weights.
    Weights(u32),
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Triangles
    }
}

impl Mode {
    /// Returns the equivalent `GLenum`.
    pub fn as_gl_enum(self) -> u32 {
        match self {
            Mode::Points => POINTS,
            Mode::Lines => LINES,
            Mode::LineLoop => LINE_LOOP,
            Mode::LineStrip => LINE_STRIP,
            Mode::Triangles => TRIANGLES,
            Mode::TriangleStrip => TRIANGLE_STRIP,
            Mode::TriangleFan => TRIANGLE_FAN,
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<Mode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Mode>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_MODES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where E: de::Error
            {
                use self::Mode::*;
                use validation::Checked::*;
                Ok(match value as u32 {
                    POINTS => Valid(Points),
                    LINES => Valid(Lines),
                    LINE_LOOP => Valid(LineLoop),
                    LINE_STRIP => Valid(LineStrip),
                    TRIANGLES => Valid(Triangles),
                    TRIANGLE_STRIP => Valid(TriangleStrip),
                    TRIANGLE_FAN => Valid(TriangleFan),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u64(Visitor)
    }
}

impl ser::Serialize for Mode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_u32(self.as_gl_enum())
    }
}

impl Semantic {
    fn checked(s: &str) -> Checked<Self> {
        use self::Semantic::*;
        use validation::Checked::*;
        match s {
            "NORMAL" => Valid(Normals),
            "POSITION" => Valid(Positions),
            "TANGENT" => Valid(Tangents),
            #[cfg(feature = "extras")]
            _ if s.starts_with("_") => Valid(Extras(s[1..].to_string())),
            _ if s.starts_with("COLOR_") => {
                match s["COLOR_".len()..].parse() {
                    Ok(set) => Valid(Colors(set)),
                    Err(_) => Invalid,
                }
            },
            _ if s.starts_with("TEXCOORD_") => {
                match s["TEXCOORD_".len()..].parse() {
                    Ok(set) => Valid(TexCoords(set)),
                    Err(_) => Invalid,
                }
            },
            _ if s.starts_with("JOINTS_") => {
                match s["JOINTS_".len()..].parse() {
                    Ok(set) => Valid(Joints(set)),
                    Err(_) => Invalid,
                }
            },
            _ if s.starts_with("WEIGHTS_") => {
                match s["WEIGHTS_".len()..].parse() {
                    Ok(set) => Valid(Weights(set)),
                    Err(_) => Invalid,
                }
            },
            _ => Invalid,
        }
    }
}

impl ser::Serialize for Semantic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToString for Semantic {
    fn to_string(&self) -> String {
        use self::Semantic::*;
        match *self {
            Positions => "POSITION".into(),
            Normals => "NORMAL".into(),
            Tangents => "TANGENT".into(),
            Colors(set) => format!("COLOR_{}", set),
            TexCoords(set) => format!("TEXCOORD_{}", set),
            Joints(set) => format!("JOINTS_{}", set),
            Weights(set) => format!("WEIGHTS_{}", set),
            #[cfg(feature = "extras")]
            Extras(ref name) => format!("_{}", name),
        }
    }
}

impl ToString for Checked<Semantic> {
    fn to_string(&self) -> String {
        match *self {
            Checked::Valid(ref semantic) => semantic.to_string(),
            Checked::Invalid => "<invalid semantic name>".into(),
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<Semantic> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Semantic>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "semantic name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Semantic::checked(value))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
