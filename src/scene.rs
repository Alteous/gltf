use cgmath;
use cgmath::prelude::*;
use json;
use std::slice;

use {Camera, Document, Mesh, Skin};

type Matrix3 = cgmath::Matrix3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;
type Quaternion = cgmath::Quaternion<f32>;

/// The transform for a `Node`.
#[derive(Clone, Debug)]
pub enum Transform {
    /// 4x4 transformation matrix in column-major order.
    Matrix {
        /// 4x4 matrix.
        matrix: [[f32; 4]; 4],
    },

    /// Decomposed TRS properties.
    Decomposed {
        /// `[x, y, z]` vector.
        translation: [f32; 3],

        /// `[x, y, z, w]` quaternion, where `w` is the scalar.
        rotation: [f32; 4],

        /// `[x, y, z]` vector.
        scale: [f32; 3],
    },
}

impl Transform {
    /// Returns the matrix representation of this transform.
    ///
    /// If the transform is `Decomposed`, then the matrix is generated with the
    /// equation `matrix = translation * rotation * scale`.
    pub fn matrix(self) -> [[f32; 4]; 4] {
        match self {
            Transform::Matrix { matrix } => matrix,
            Transform::Decomposed { translation: t, rotation: r, scale: s } => {
                let t = Matrix4::from_translation(t.into());
                let r = Matrix4::from(Quaternion::new(r[3], r[0], r[1], r[2]));
                let s = Matrix4::from_nonuniform_scale(s[0], s[1], s[2]);
                (t * r * s).into()
            },
        }
    }

    /// Returns a decomposed representation of this transform.
    ///
    /// If the transform is `Matrix`, then the decomposition is extracted from the
    /// matrix.
    pub fn decomposed(self) -> ([f32; 3], [f32; 4], [f32; 3]) {
        match self {
            Transform::Matrix { matrix: m } => {
                let translation = [m[3][0], m[3][1], m[3][2]];
                let mut i = Matrix3::new(
                    m[0][0], m[0][1], m[0][2],
                    m[1][0], m[1][1], m[1][2],
                    m[2][0], m[2][1], m[2][2],
                );
                let sx = i.x.magnitude();
                let sy = i.y.magnitude();
                let sz = i.determinant().signum() * i.z.magnitude();
                let scale = [sx, sy, sz];
                i.x /= sx;
                i.y /= sy;
                i.z /= sz;
                let r = Quaternion::from(i);
                let rotation = [r.v.x, r.v.y, r.v.z, r.s];
                (translation, rotation, scale)
            },
            Transform::Decomposed { translation, rotation, scale } => {
                (translation, rotation, scale)
            },
        }
    }
}

/// A node in the node hierarchy.
///
/// When a node contains a skin, all its meshes contain `JOINTS_0` and `WEIGHTS_0`
/// attributes.
#[derive(Clone, Debug)]
pub struct Node<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::scene::Node,
}

/// The root nodes of a scene.
#[derive(Clone, Debug)]
pub struct Scene<'a> {
    /// The parent `Document` struct.
    #[allow(dead_code)]
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::scene::Scene,
}

/// An `Iterator` that visits the nodes in a scene.
#[derive(Clone, Debug)]
pub struct Nodes<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The internal node index iterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

/// An `Iterator` that visits the children of a node.
#[derive(Clone, Debug)]
pub struct Children<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The internal node index iterator.
    iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

impl<'a> Node<'a> {
    /// Constructs a `Node`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::scene::Node,
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

    /// Returns the camera referenced by this node.
    pub fn camera(&self) -> Option<Camera> {
        self.json.camera.as_ref().map(|index| {
            self.document.cameras().nth(index.value()).unwrap()
        })
    }

    /// Returns an `Iterator` that visits the node's children.
    pub fn children(&self) -> Children {
        Children {
            document: self.document,
            iter: self.json.children.as_ref().map_or([].iter(), |x| x.iter()),
        }
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Returns the mesh referenced by this node.
    pub fn mesh(&self) -> Option<Mesh> {
        self.json.mesh.as_ref().map(|index| {
            self.document.meshes().nth(index.value()).unwrap()
        })
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns the node's transform.
    pub fn transform(&self) -> Transform {
        if let Some(m) = self.json.matrix {
            Transform::Matrix {
                matrix: [
                    [m[0], m[1], m[2], m[3]],
                    [m[4], m[5], m[6], m[7]],
                    [m[8], m[9], m[10], m[11]],
                    [m[12], m[13], m[14], m[15]],
                ],
            }
        } else {
            Transform::Decomposed {
                translation: self.json.translation,
                rotation: self.json.rotation.0,
                scale: self.json.scale,
            }
        }
    }

    /// Returns the skin referenced by this node.
    pub fn skin(&self) -> Option<Skin> {
        self.json.skin.as_ref().map(|index| {
            self.document.skins().nth(index.value()).unwrap()
        })
    }

    /// Returns the weights of the instantiated morph target.
    pub fn weights(&self) -> Option<&[f32]> {
        self.json.weights.as_ref().map(Vec::as_slice)
    }
}

impl<'a> Scene<'a> {
    /// Constructs a `Scene`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::scene::Scene,
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
    pub fn extras(&self) -> &json::Extras{
        &self.json.extras
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Returns an `Iterator` that visits each root node of the scene.
    pub fn nodes(&self) -> Nodes<'a> {
        Nodes {
            document: self.document,
            iter: self.json.nodes.iter(),
        }
    }
}

impl<'a> ExactSizeIterator for Nodes<'a> {}
impl<'a> Iterator for Nodes<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.document.nodes().nth(index.value()).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Children<'a> {}
impl<'a> Iterator for Children<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.document.nodes().nth(index.value()).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use ::cgmath::{vec3, InnerSpace, Matrix4, Quaternion, Rad, Rotation3};
    use ::scene::Transform;
    use ::std::f32::consts::PI;

    fn rotate(x: f32, y: f32, z: f32, r: f32) -> [f32; 4] {
        let r = Quaternion::from_axis_angle(vec3(x, y, z).normalize(), Rad(r));
        [r[1], r[2], r[3], r[0]]
    }

    fn test_decompose(translation: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) {
        let matrix = Transform::Decomposed { translation, rotation, scale }.matrix();
        let (translation, rotation, scale) = Transform::Matrix { matrix }.decomposed();
        let check = Transform::Decomposed { translation, rotation, scale }.matrix();
        assert_relative_eq!(
            Matrix4::from(check),
            Matrix4::from(matrix),
            epsilon = 0.05
        );
    }

    fn test_decompose_rotation(rotation: [f32; 4]) {
        let translation = [1.0, -2.0, 3.0];
        let scale = [1.0, 1.0, 1.0];
        test_decompose(translation, rotation, scale);
    }

    fn test_decompose_scale(scale: [f32; 3]) {
        let translation = [1.0, 2.0, 3.0];
        let rotation = rotate(1.0, 0.0, 0.0, PI / 2.0);
        test_decompose(translation, rotation, scale);
    }

    fn test_decompose_translation(translation: [f32; 3]) {
        let rotation = [0.0, 0.0, 0.0, 1.0];
        let scale = [1.0, 1.0, 1.0];
        test_decompose(translation, rotation, scale);
    }

    #[test]
    fn decompose_identity() {
        let translation = [0.0, 0.0, 0.0];
        let rotation = [0.0, 0.0, 0.0, 1.0];
        let scale = [1.0, 1.0, 1.0];
        test_decompose(translation, rotation, scale);
    }

    #[test]
    fn decompose_translation_unit_x() {
        let translation = [1.0, 0.0, 0.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_unit_y() {
        let translation = [0.0, 1.0, 0.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_unit_z() {
        let translation = [0.0, 0.0, 1.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_random0() {
        let translation = [1.0, -1.0, 1.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_random1() {
        let translation = [-1.0, -1.0, -1.0];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_translation_random2() {
        let translation = [-10.0, 100000.0, -0.0001];
        test_decompose_translation(translation);
    }

    #[test]
    fn decompose_rotation_xaxis() {
        let rotation = rotate(1.0, 0.0, 0.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_yaxis() {
        let rotation = rotate(0.0, 1.0, 0.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_zaxis() {
        let rotation = rotate(0.0, 0.0, 1.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_negative_xaxis() {
        let rotation = rotate(-1.0, 0.0, 0.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_negative_yaxis() {
        let rotation = rotate(0.0, -1.0, 0.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_negative_zaxis() {
        let rotation = rotate(0.0, 0.0, -1.0, PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_eighth_turn() {
        let rotation = rotate(1.0, 0.0, 0.0, PI / 4.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_negative_quarter_turn() {
        let rotation = rotate(0.0, 1.0, 0.0, -PI / 2.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_half_turn() {
        let rotation = rotate(0.0, 0.0, 1.0, PI);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_zero_turn_xaxis() {
        let rotation = rotate(1.0, 0.0, 0.0, 0.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_zero_turn_yaxis() {
        let rotation = rotate(0.0, 1.0, 0.0, 0.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_zero_turn_zaxis() {
        let rotation = rotate(0.0, 0.0, 1.0, 0.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_full_turn() {
        let rotation = rotate(1.0, 0.0, 0.0, 2.0 * PI);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_random0() {
        let rotation = rotate(1.0, 1.0, 1.0, PI / 3.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_rotation_random1() {
        let rotation = rotate(1.0, -1.0, 1.0, -PI / 6.0);
        test_decompose_rotation(rotation);
    }

    #[test]
    fn decompose_uniform_scale_up() {
        let scale = [100.0, 100.0, 100.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_uniform_scale_down() {
        let scale = [0.01, 0.01, 0.01];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_xscale_up() {
        let scale = [100.0, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_xscale_down() {
        let scale = [0.001, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_yscale_up() {
        let scale = [1.0, 100.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_yscale_down() {
        let scale = [1.0, 0.001, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_zscale_up() {
        let scale = [1.0, 1.0, 100.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_zscale_down() {
        let scale = [1.0, 1.0, 0.001];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_xscale_unit() {
        let scale = [-1.0, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_xscale_up() {
        let scale = [-10.0, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_xscale_down() {
        let scale = [-0.1, 1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_yscale_unit() {
        let scale = [1.0, -1.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_yscale_up() {
        let scale = [1.0, -10.0, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_yscale_down() {
        let scale = [1.0, -0.1, 1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_zscale_unit() {
        let scale = [1.0, 1.0, -1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_zscale_up() {
        let scale = [1.0, 1.0, -10.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_negative_zscale_down() {
        let scale = [1.0, 1.0, -0.1];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_up_sml() {
        let scale = [10.0, 100.0, 1000.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_up_mls() {
        let scale = [100.0, 1000.0, 10.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_up_lsm() {
        let scale = [1000.0, 10.0, 100.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_down_sml() {
        let scale = [0.01, 0.001, 0.0001];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_down_mls() {
        let scale = [0.001, 0.0001, 0.01];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_down_lsm() {
        let scale = [0.0001, 0.01, 0.01];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_unit_ls() {
        let scale = [1.0, 100000.0, 0.000001];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_ms_negative_unit() {
        let scale = [10.0, 0.1, -1.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_ms_negative_up() {
        let scale = [10.0, 0.1, -10.0];
        test_decompose_scale(scale);
    }

    #[test]
    fn decompose_nonuniform_scale_ms_negative_down() {
        let scale = [10.0, 0.1, -0.1];
        test_decompose_scale(scale);
    }
}
