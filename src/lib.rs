
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]
#![allow(unknown_lints)]

//! glTF 2.0 loader
//!
//! This crate is intended to load [glTF 2.0], a file format designed for the
//! efficient runtime transmission of 3D scenes. The crate aims to provide
//! rustic utilities that make working with glTF simple and intuitive.
//!
//! [glTF 2.0]: https://www.khronos.org/gltf
//!
//! ## Installation
//!
//! Add `gltf` version 0.9 to your `Cargo.toml`.
//!
//! ```toml
//! [dependencies.gltf]
//! version = "0.9"
//! ```
//!
//! ## Examples
//!
//! ### Walking the node hierarchy
//!
//! Below demonstates visiting the root [`Node`]s of every [`Scene`], printing the
//! number of children each node has.
//!
//! [`Node`]: scene/struct.Node.html
//! [`Scene`]: scene/struct.Scene.html
//! ```
//! # fn run() -> Result<(), Box<std::error::Error>> {
//! # use std::{fs, io};
//! # let path = "examples/Box.gltf";
//! use gltf::Gltf;
//! let file = fs::File::open(path)?;
//! let gltf = Gltf::from_reader(io::BufReader::new(file))?.validate_minimally()?;
//! for scene in gltf.scenes() {
//!     for node in scene.nodes() {
//!         // Do something with this node.
//!         println!(
//!             "Node {} has {} children",
//!             node.index(),
//!             node.children().count(),
//!         );
//!     }
//! }
//! # Ok(())
//! # }
//! # fn main() {
//! #    let _ = run().expect("No runtime errors");
//! # }
//! ```

#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate byteorder;
extern crate cgmath;
#[macro_use]
extern crate lazy_static;

/// Contains (de)serializable data structures that match the glTF JSON text.
#[deprecated(since = "0.9.1", note = "Will be removed in 1.0.0")]
#[doc(hidden)]
pub extern crate gltf_json as json;

/// Accessors for reading vertex attributes from buffer views.
pub mod accessor;

/// Animations, their channels, targets, and samplers.
pub mod animation;

/// Buffers and buffer views.
pub mod buffer;

/// Cameras and their projections.
pub mod camera;

/// Primitives for working with binary glTF.
pub mod glb;

/// **The main module - start here.**
pub mod gltf;

/// Images that may be used by textures.
pub mod image;

/// Material properties of primitives.
pub mod material;

/// Meshes and their primitives.
pub mod mesh;

/// The glTF node heirarchy.
pub mod scene;

/// Mesh skinning primitives.
pub mod skin;

/// Textures and their samplers.
pub mod texture;

/// Validation data.
pub mod validation;

pub use self::animation::Animation;
pub use self::accessor::Accessor;
pub use self::buffer::Buffer;
pub use self::camera::Camera;
pub use self::glb::Glb;
pub use self::gltf::Gltf;
pub use self::image::Image;
pub use self::material::Material;
pub use self::mesh::{Attribute, Mesh, Primitive, Semantic};
pub use self::scene::{Node, Scene};
pub use self::skin::Skin;
pub use self::texture::Texture;
pub use self::validation::Unvalidated;

use std::{error, fmt};

/// Generic runtime error provided for convenience.
///
/// Users may opt for module-level errors instead.
#[derive(Debug)]
pub enum Error {
    /// JSON deserialization error.
    Deserialize(json::Error),
    /// GLB parsing error.
    Glb(glb::Error),
    /// `glTF` validation error.
    Validation(validation::Error),
    /// Intended to prevent use in `match` expressions.
    #[doc(hidden)]
    _Nonexhaustive,
}

/// Returns `true` if the slice begins with the `b"glTF"` magic string, indicating
/// a binary `glTF` asset.
///
/// # Examples
///
/// ```rust
/// assert_eq!(true, gltf::is_binary(b"glTF..."));
/// assert_eq!(false, gltf::is_binary(b"{...}"));
/// ```
pub fn is_binary(slice: &[u8]) -> bool {
    slice.starts_with(b"glTF")
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
         match *self {
             Error::Deserialize(_) => "deserialization error",
             Error::Glb(ref e) => e.description(),
             Error::Validation(_) => "invalid glTF JSON",
             _ => unreachable!(),
        }
    }
}

impl From<glb::Error> for Error {
    fn from(err: glb::Error) -> Self {
        Error::Glb(err)
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Self {
        Error::Deserialize(err)
    }
}

impl From<validation::Error> for Error {
    fn from(err: validation::Error) -> Self {
        Error::Validation(err)
    }
}
