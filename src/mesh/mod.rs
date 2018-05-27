/// Iterators.
pub mod iter;

/// Utility functions.
#[cfg(feature = "utils")]
pub mod util;

use json;
use {Accessor, Buffer, Document, Material};

#[cfg(feature = "utils")]
use accessor;

pub use json::mesh::{Mode, Semantic};
use json::validation::Checked;

/// Vertex attribute data.
pub type Attribute<'a> = (Semantic, Accessor<'a>);

/// Vertex position bounding box.
pub type BoundingBox = Bounds<[f32; 3]>;

/// The minimum and maximum values for a generic accessor.
#[derive(Clone, Debug, PartialEq)]
pub struct Bounds<T> {
    /// Minimum value.
    pub min: T,

    /// Maximum value.
    pub max: T,
}

/// A set of primitives to be rendered.
#[derive(Clone, Debug)]
pub struct Mesh<'a>  {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Mesh,
}

/// A single morph target for a mesh primitive.
#[derive(Clone, Debug)]
pub struct MorphTarget<'a> {
    /// XYZ vertex position displacements.
    positions: Option<Accessor<'a>>,

    /// XYZ vertex normal displacements.
    normals: Option<Accessor<'a>>,

    /// XYZ vertex tangent displacements.
    tangents: Option<Accessor<'a>>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug)]
pub struct Primitive<'a>  {
    /// The parent `Mesh` struct.
    mesh: &'a Mesh<'a>,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::mesh::Primitive,
}

/// Mesh primitive reader.
#[derive(Clone, Debug)]
pub struct Reader<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    pub(crate) primitive: &'a Primitive<'a>,
    pub(crate) get_buffer_data: F,
}

impl<'a> Mesh<'a>  {
    /// Constructs a `Mesh`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::mesh::Mesh,
    ) -> Self {
        Self {
            document: document,
            index: index,
            json: json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Defines the geometry to be renderered with a material.
    pub fn primitives(&'a self) -> iter::Primitives<'a> {
        iter::Primitives {
            mesh: self,
            iter: self.json.primitives.iter().enumerate(),
        }
    }

    /// Defines the weights to be applied to the morph targets.
    pub fn weights(&self) -> Option<&[f32]> {
        self.json.weights.as_ref().map(Vec::as_slice)
    }
}

impl<'a> Primitive<'a> {
    /// Constructs a `Primitive`.
    pub(crate) fn new(
        mesh: &'a Mesh<'a>,
        index: usize,
        json: &'a json::mesh::Primitive,
    ) -> Self {
        Self {
            mesh: mesh,
            index: index,
            json: json,
        }
    }

    /// Returns the bounds of the `POSITION` vertex attribute.
    pub fn bounding_box(&self) -> BoundingBox {
        // NOTE: cannot panic if validated "minimally"
        let pos_accessor_index = self.json.attributes.get(&Checked::Valid(Semantic::Positions)).unwrap();
        let pos_accessor = self.mesh.document.accessors().nth(pos_accessor_index.value()).unwrap();
        let min: [f32; 3] = json::deserialize::from_value(pos_accessor.min().unwrap()).unwrap();
        let max: [f32; 3] = json::deserialize::from_value(pos_accessor.max().unwrap()).unwrap();
        Bounds { min, max }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Return the accessor with the given semantic.
    pub fn get(&self, semantic: &Semantic) -> Option<Accessor> {
        self.json.attributes
            .get(&json::validation::Checked::Valid(semantic.clone()))
            .map(|index| self.mesh.document.accessors().nth(index.value()).unwrap())
    }

    /// Returns the accessor containing the primitive indices, if provided.
    pub fn indices(&self) -> Option<Accessor> {
        self.json.indices
            .as_ref()
            .map(|index| self.mesh.document.accessors().nth(index.value()).unwrap())
    }

    /// Returns an `Iterator` that visits the vertex attributes.
    pub fn attributes(&self) -> iter::Attributes {
        iter::Attributes {
            document: self.mesh.document,
            prim: self,
            iter: self.json.attributes.iter(),
        }
    }

    /// Returns the material to apply to this primitive when rendering
    pub fn material(&self) -> Material {
        self.json.material
            .as_ref()
            .map(|index| self.mesh.document.materials().nth(index.value()).unwrap())
            .unwrap_or_else(|| Material::default(self.mesh.document))
    }

    /// The type of primitives to render.
    pub fn mode(&self) -> Mode {
        self.json.mode.unwrap()
    }

    /// Returns an `Iterator` that visits the morph targets of the primitive.
    pub fn morph_targets(&self) -> iter::MorphTargets {
        if let Some(slice) = self.json.targets.as_ref() {
            iter::MorphTargets {
                document: self.mesh.document,
                iter: slice.iter(),
            }
        } else {
            iter::MorphTargets {
                document: self.mesh.document,
                iter: (&[]).iter(),
            }
        }
    }

    /// Constructs the primitive reader.
    #[cfg(feature = "utils")]
    pub fn reader<'s, F>(
        &'a self,
        get_buffer_data: F,
    ) -> Reader<'a, 's, F>
    where
        F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
    {
        Reader { primitive: self, get_buffer_data }
    }
}

#[cfg(feature = "utils")]
impl<'a, 's, F> Reader<'a, 's, F>
    where F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    /// Visits the vertex positions of a primitive.
    pub fn read_positions(&self) -> Option<util::ReadPositions<'s>> {
        if let Some(accessor) = self.primitive.get(&Semantic::Positions) {
            if let Some(slice) = (self.get_buffer_data)(accessor.clone().view().buffer()) {
                return Some(accessor::Iter::new(accessor, slice))
            }
        }
        None
    }

    /// Visits the vertex normals of a primitive.
    pub fn read_normals(&self) -> Option<util::ReadNormals<'s>> {
        if let Some(accessor) = self.primitive.get(&Semantic::Normals) {
            if let Some(slice) = (self.get_buffer_data)(accessor.clone().view().buffer()) {
                return Some(accessor::Iter::new(accessor, slice))
            }
        }
        None
    }

    /// Visits the vertex tangents of a primitive.
    pub fn read_tangents(&self) -> Option<util::ReadTangents<'s>> {
        if let Some(accessor) = self.primitive.get(&Semantic::Tangents) {
            if let Some(slice) = (self.get_buffer_data)(accessor.clone().view().buffer()) {
                return Some(accessor::Iter::new(accessor, slice))
            }
        }
        None
    }

    /// Visits the vertex colors of a primitive.
    pub fn read_colors(&self, set: u32) -> Option<util::ReadColors<'s>> {
        use accessor::DataType::{U8, U16, F32};
        use accessor::Dimensions::{Vec3, Vec4};
        use self::util::ReadColors;

        if let Some(accessor) = self.primitive.get(&Semantic::Colors(set)) {
            if let Some(slice) = (self.get_buffer_data)(accessor.clone().view().buffer()) {
                return Some(
                    match (accessor.data_type(), accessor.dimensions()) {
                        (U8, Vec3)  => ReadColors::RgbU8(accessor::Iter::new(accessor, slice)),
                        (U16, Vec3) => ReadColors::RgbU16(accessor::Iter::new(accessor, slice)),
                        (F32, Vec3) => ReadColors::RgbF32(accessor::Iter::new(accessor, slice)),
                        (U8, Vec4)  => ReadColors::RgbaU8(accessor::Iter::new(accessor, slice)),
                        (U16, Vec4) => ReadColors::RgbaU16(accessor::Iter::new(accessor, slice)),
                        (F32, Vec4) => ReadColors::RgbaF32(accessor::Iter::new(accessor, slice)),
                        _ => unreachable!(),
                    }
                )
            }
        }

        None
    }

    /// Visits the vertex draw sequence of a primitive.
    pub fn read_indices(&self) -> Option<util::ReadIndices<'s>> {
        use accessor::DataType;
        use self::util::ReadIndices;

        if let Some(accessor) = self.primitive.indices() {
            if let Some(slice) = (self.get_buffer_data)(accessor.clone().view().buffer()) {
                return Some(
                    match accessor.data_type() {
                        DataType::U8  => ReadIndices::U8(accessor::Iter::new(accessor, slice)),
                        DataType::U16 => ReadIndices::U16(accessor::Iter::new(accessor, slice)),
                        DataType::U32 => ReadIndices::U32(accessor::Iter::new(accessor, slice)),
                        _ => unreachable!(),
                    }
                )
            }
        }

        None
    }

    /// Visits the joint indices of the primitive.
    pub fn read_joints(&self, set: u32) -> Option<util::ReadJoints<'s>> {
        use accessor::DataType;
        use self::util::ReadJoints;

        if let Some(accessor) = self.primitive.get(&Semantic::Joints(set)) {
            if let Some(slice) = (self.get_buffer_data)(accessor.view().buffer()) {
                return Some(
                    match accessor.data_type() {
                        DataType::U8  => ReadJoints::U8(accessor::Iter::new(accessor, slice)),
                        DataType::U16 => ReadJoints::U16(accessor::Iter::new(accessor, slice)),
                        _ => unreachable!(),
                    }
                )
            }
        }

        None
    }

    /// Visits the vertex texture co-ordinates of a primitive.
    pub fn read_tex_coords(&self, set: u32) -> Option<util::ReadTexCoords<'s>> {
        use accessor::DataType;
        use self::util::ReadTexCoords;

        if let Some(accessor) = self.primitive.get(&Semantic::TexCoords(set)) {
            if let Some(slice) = (self.get_buffer_data)(accessor.view().buffer()) {
                return Some(
                    match accessor.data_type() {
                        DataType::U8  => ReadTexCoords::U8(accessor::Iter::new(accessor, slice)),
                        DataType::U16 => ReadTexCoords::U16(accessor::Iter::new(accessor, slice)),
                        DataType::F32 => ReadTexCoords::F32(accessor::Iter::new(accessor, slice)),
                        _ => unreachable!(),
                    }
                )
            }
        }

        None
    }

    /// Visits the joint weights of the primitive.
    pub fn read_weights(&self, set: u32) -> Option<util::ReadWeights<'s>>  {
        use self::accessor::DataType;
        use self::util::ReadWeights;

        if let Some(accessor) = self.primitive.get(&Semantic::Weights(set)) {
            if let Some(slice) = (self.get_buffer_data)(accessor.view().buffer()) {
                return Some(
                    match accessor.data_type() {
                        DataType::U8  => ReadWeights::U8(accessor::Iter::new(accessor, slice)),
                        DataType::U16 => ReadWeights::U16(accessor::Iter::new(accessor, slice)),
                        DataType::F32 => ReadWeights::F32(accessor::Iter::new(accessor, slice)),
                        _ => unreachable!(),
                    }
                )
            }
        }

        None
    }

    /// Visits the morph targets of the primitive.
    pub fn read_morph_targets(&self) -> util::ReadMorphTargets<'a, 's, F> {
        util::ReadMorphTargets {
            index: 0,
            reader: self.clone(),
        }
    }
}

impl<'a> MorphTarget<'a> {
    /// Returns the XYZ vertex position displacements.
    pub fn positions(&self) -> Option<Accessor<'a>> {
        self.positions.clone()
    }

    /// Returns the XYZ vertex normal displacements.
    pub fn normals(&self) -> Option<Accessor<'a>> {
        self.normals.clone()
    }

    /// Returns the XYZ vertex tangent displacements.
    pub fn tangents(&self) -> Option<Accessor<'a>> {
        self.tangents.clone()
    }
}
