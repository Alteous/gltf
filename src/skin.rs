use json;
use std::slice;

use {Accessor, Buffer, Document, Node};

#[cfg(feature = "utils")]
use accessor;

/// Inverse Bind Matrices of type `[[f32; 4]; 4]`.
#[cfg(feature = "utils")]
pub type ReadInverseBindMatrices<'a> = accessor::Iter<'a, [[f32; 4]; 4]>;

/// Joints and matrices defining a skin.
#[derive(Clone, Debug)]
pub struct Skin<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::skin::Skin,
}

/// An `Iterator` that visits the joints of a `Skin`.
#[derive(Clone, Debug)]
pub struct Joints<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The internal node index iterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

/// Skin reader.
#[cfg(feature = "utils")]
#[derive(Clone, Debug)]
pub struct Reader<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    skin: Skin<'a>,
    get_buffer_data: F,
}

impl<'a> Skin<'a> {
    /// Constructs a `Skin`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::skin::Skin,
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

    /// Returns the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`, each matrix is assumed to be the 4x4 identity matrix which
    /// implies that the inverse-bind matrices were pre-applied.
    pub fn inverse_bind_matrices(&self) -> Option<Accessor<'a>> {
        self.json.inverse_bind_matrices
            .as_ref()
            .map(|index| {
                self.document
                    .accessors()
                    .nth(index.value())
                    .unwrap()
            })
    }

    /// Constructs a skin reader.
    #[cfg(feature = "utils")]
    pub fn reader<'s, F>(
        &'a self,
        get_buffer_data: F,
    ) -> Reader<'a, 's, F>
    where
        F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
    {
        Reader {
            skin: self.clone(),
            get_buffer_data,
        }
    }

    /// Returns an `Iterator` that visits the skeleton nodes used as joints in
    /// this skin.
    pub fn joints(&self) -> Joints<'a> {
        Joints {
            document: self.document,
            iter: self.json.joints.iter(),
        }
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns the node used as the skeleton root. When `None`, joints
    /// transforms resolve to scene root.
    pub fn skeleton(&self) -> Option<Node<'a>> {
        self.json.skeleton.as_ref().map(|index| {
            self.document.nodes().nth(index.value()).unwrap()
        })
    }
}

#[cfg(feature = "utils")]
impl<'a, 's, F> Reader<'a, 's, F>
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    /// Returns an `Iterator` that reads the inverse bind matrices of
    /// the skin.
    pub fn read_inverse_bind_matrices(&self) -> Option<ReadInverseBindMatrices<'s>> {
        if let Some(accessor) = self.skin.inverse_bind_matrices() {
            if let Some(slice) = (self.get_buffer_data)(accessor.view().buffer()) {
                return Some(accessor::Iter::new(accessor, slice))
            }
        }

        None
    }
}

impl<'a> Iterator for Joints<'a>  {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.document.nodes().nth(index.value()).unwrap())
    }
}
