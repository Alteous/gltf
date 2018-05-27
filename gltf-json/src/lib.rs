#[macro_use]
extern crate gltf_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// Contains `Accessor` and other related data structures.
pub mod accessor;

/// Contains `Animation` and other related data structures.
pub mod animation;

/// Contains `Asset` metadata.
pub mod asset;

/// Contains `Buffer`, `View`, and other related data structures.
pub mod buffer;

/// Contains `Camera` and other related data structures.
pub mod camera;

/// Contains extension specific data structures and the names of all
/// 2.0 extensions supported by the library.
pub mod extensions;

/// Contains `Extras`.
pub mod extras;

/// Contains `Image` and other related data structures.
pub mod image;

/// Contains `Material` and other related data structures.
pub mod material;

/// Contains `Mesh` and other related data structures.
pub mod mesh;

/// Contains `Path`.
pub mod path;

/// Contains `Root`.
pub mod root;

/// Contains `Scene`, `Node`, and other related data structures.
pub mod scene;

/// Contains `Skin` and other related data structures.
pub mod skin;

/// Contains `Texture`, `Sampler`, and other related data structures.
pub mod texture;

/// Contains functions that validate glTF JSON data against the specification.
pub mod validation;

#[doc(inline)]
pub use accessor::Accessor;
#[doc(inline)]
pub use animation::Animation;
#[doc(inline)]
pub use asset::Asset;
#[doc(inline)]
pub use buffer::Buffer;
#[doc(inline)]
pub use camera::Camera;
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use material::Material;
#[doc(inline)]
pub use mesh::Mesh;
#[doc(inline)]
pub use scene::Node;
#[doc(inline)]
pub use scene::Scene;
#[doc(inline)]
pub use skin::Skin;
#[doc(inline)]
pub use texture::Texture;

#[doc(inline)]
pub use self::extras::Extras;
#[doc(inline)]
pub use self::path::Path;
#[doc(inline)]
pub use self::root::Index;
#[doc(inline)]
pub use self::root::Root;

#[doc(inline)]
pub use serde_json::Error;
#[doc(inline)]
pub use serde_json::Value;

/// Re-exports of `serde_json` deserialization functions.
///
/// This module re-exports the generic serde deserialization functions
/// so that one can deserialize data structures other than `Root` without
/// being bound to a specific version of `serde_json`.
pub mod deserialize {
    pub use serde_json::{from_reader, from_slice, from_str, from_value};
}

/// Re-exports of `serde_json` serialization functions.
///
/// This module re-exports the generic serde serialization functions
/// so that one can serialize data structures other than `Root` without
/// being bound to a specific version of `serde_json`.
pub mod serialize {
    pub use serde_json::{to_string, to_string_pretty, to_value, to_vec, to_vec_pretty, to_writer, to_writer_pretty};
}
