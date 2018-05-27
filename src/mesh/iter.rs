use std::{collections, iter, slice};
use json;

use super::{Attribute, Mesh, MorphTarget, Primitive};
use ::Document;

/// An `Iterator` that visits the morph targets of a `Primitive`.
#[derive(Clone, Debug)]
pub struct MorphTargets<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The internal JSON iterator.
    pub(crate) iter: slice::Iter<'a, json::mesh::MorphTarget>,
}

/// An `Iterator` that visits the attributes of a `Primitive`.
#[derive(Clone, Debug)]
pub struct Attributes<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The parent `Primitive` struct.
    pub(crate) prim: &'a Primitive<'a>,

    /// The internal attribute iterator.
    pub(crate) iter: collections::hash_map::Iter<
            'a,
        json::validation::Checked<json::mesh::Semantic>,
        json::Index<json::accessor::Accessor>,
        >,
}

/// An `Iterator` that visits the primitives of a `Mesh`.
#[derive(Clone, Debug)]
pub struct Primitives<'a>  {
    /// The parent `Mesh` struct.
    pub(crate) mesh: &'a Mesh<'a>,

    /// The internal JSON primitive iterator.
    pub(crate) iter: iter::Enumerate<slice::Iter<'a, json::mesh::Primitive>>,
}

impl<'a> ExactSizeIterator for Attributes<'a> {}
impl<'a> Iterator for Attributes<'a> {
    type Item = Attribute<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(key, index)| {
                let semantic = key.as_ref().unwrap().clone();
                let accessor = self.document.accessors().nth(index.value()).unwrap();
                (semantic, accessor)
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Primitives<'a> {}
impl<'a> Iterator for Primitives<'a> {
    type Item = Primitive<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, json)| Primitive::new(self.mesh, index, json))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for MorphTargets<'a> {}
impl<'a> Iterator for MorphTargets<'a> {
    type Item = MorphTarget<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|json| {
                let positions = json.positions
                    .as_ref()
                    .map(|index| self.document.accessors().nth(index.value()).unwrap());
                let normals = json.normals
                    .as_ref()
                    .map(|index| self.document.accessors().nth(index.value()).unwrap());
                let tangents = json.tangents
                    .as_ref()
                    .map(|index| self.document.accessors().nth(index.value()).unwrap());
                MorphTarget {
                    positions,
                    normals,
                    tangents,
                }
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
