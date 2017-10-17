
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The reference loader implementation for the `gltf` crate.
//!
//! # Examples
//!
//! ### Importing some `glTF` 2.0
//!
//! ```rust
//! use gltf_importer::import;
//! # #[allow(unused_variables)]
//! let path = "path/to/asset.gltf";
//! # let path = "../examples/Box.gltf";
//! match import(path) {
//!     Ok((gltf, _)) => println!("{:#?}", gltf.as_json()),
//!     Err(err) => println!("error: {:?}", err),
//! }
//! ```

extern crate base64;
extern crate gltf;
extern crate gltf_utils;

use gltf::json::{self, validation};
use std::{fmt, fs, io};

use gltf::Gltf;
use gltf_utils::Source;
use std::error::Error as StdError;
use std::path::Path;

/// Contains parameters for import configuration.
pub mod config;

pub use self::config::Config;
pub use self::config::ValidationStrategy;

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum Error {
    /// A loaded glTF buffer is not of the required length.
    BufferLength(json::Path),

    /// Base 64 decoding error.
    Base64Decoding(base64::DecodeError),

    /// A glTF extension required by the asset has not been enabled by the user.
    ExtensionDisabled(String),

    /// A glTF extension required by the asset is not supported by the library.
    ExtensionUnsupported(String),

    /// The glTF version of the asset is incompatible with the importer.
    IncompatibleVersion(String),

    /// Standard I/O error.
    Io(std::io::Error),

    /// `gltf` crate error.
    Gltf(gltf::Error),

    /// Failure when deserializing .gltf or .glb JSON.
    MalformedJson(json::Error),

    /// The .gltf data is invalid.
    Validation(Vec<(json::Path, validation::Error)>),
}

/// Buffer data returned from `import`.
#[derive(Clone, Debug)]
pub struct Buffers(Vec<Vec<u8>>);

impl Source for Buffers {
    fn source_buffer(&self, buffer: &gltf::Buffer) -> &[u8] {
        &self.0[buffer.index()]
    }
}

impl Buffers {
    /// Obtain the contents of a loaded buffer.
    pub fn buffer(&self, buffer: &gltf::Buffer) -> Option<&[u8]> {
        self.0.get(buffer.index()).map(Vec::as_slice)
    }

    /// Obtain the contents of a loaded buffer view.
    pub fn view(&self, view: &gltf::buffer::View) -> Option<&[u8]> {
        self.buffer(&view.buffer())
            .map(|data| {
                let begin = view.offset();
                let end = begin + view.length();
                &data[begin..end]
            })
    }

    /// Take the loaded buffer data.
    pub fn take(self) -> Vec<Vec<u8>> {
        self.0
    }
}

fn import_impl(path: &Path, config: &Config) -> Result<(Gltf, Buffers), Error> {
    let data = read_to_end(path)?;
    if data.starts_with(b"glTF") {
        import_binary(&data, config, path)
    } else {
        import_standard(&data, config, path)
    }
}

/// Imports glTF 2.0 with custom configuration.
pub fn import_with_config<P>(path: P, config: &Config) -> Result<(Gltf, Buffers), Error>
    where P: AsRef<Path>
{
    import_impl(path.as_ref(), config)
}

/// Imports glTF 2.0 with default configuration.
pub fn import<P>(path: P) -> Result<(Gltf, Buffers), Error>
    where P: AsRef<Path>
{
    import_impl(path.as_ref(), &Default::default())
}

fn read_to_end_impl(path: &Path) -> Result<Vec<u8>, Error> {
    use io::Read;
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    let mut buffer = vec![];
    let _ = reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn read_to_end<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    read_to_end_impl(path.as_ref())
}

fn parse_data_uri(uri: &str) -> Result<Vec<u8>, Error> {
    let encoded = uri.split(',').nth(1).unwrap();
    let decoded = base64::decode(&encoded)?;
    Ok(decoded)
}

fn load_external_buffers(
    base_path: &Path,
    gltf: &Gltf,
    mut bin: Option<Vec<u8>>,
) -> Result<Vec<Vec<u8>>, Error> {
    let mut buffers = vec![];
    for (index, buffer) in gltf.buffers().enumerate() {
        let uri = buffer.uri();
        let data = if uri == "#bin" {
            Ok(bin.take().unwrap())
        } else if uri.starts_with("data:") {
            Ok(parse_data_uri(uri)?)
        } else {
            let path = base_path.parent().unwrap_or_else(|| Path::new("./")).join(uri);
            read_to_end(&path)
        }?;
        if data.len() < buffer.length() {
            let path = json::Path::new().field("buffers").index(index);
            return Err(Error::BufferLength(path));
        }
        buffers.push(data);
    }
    Ok(buffers)
}

fn validate_standard(
    unvalidated: gltf::Unvalidated,
    config: &Config,
) -> Result<Gltf, Error> {
    use config::ValidationStrategy;
    match config.validation_strategy {
        ValidationStrategy::Skip => Ok(unvalidated.skip_validation()),
        ValidationStrategy::Minimal => unvalidated.validate_minimally(),
        ValidationStrategy::Complete => unvalidated.validate_completely(),
    }.map_err(Error::Gltf)
}

fn validate_binary(
    unvalidated: gltf::Unvalidated,
    config: &Config,
    has_bin: bool,
) -> Result<Gltf, Error> {
    use config::ValidationStrategy;
    use json::validation::Error as Reason;

    if config.validation_strategy == ValidationStrategy::Skip {
        return Ok(unvalidated.skip_validation());
    }

    let mut errs = vec![];
    {
        let json = unvalidated.as_json();
        for (index, buffer) in json.buffers.iter().enumerate() {
            let path = || {
                json::Path::new()
                    .field("buffers")
                    .index(index)
                    .field("uri")
            };
            match index {
                0 if has_bin => if buffer.uri.is_some() {
                    errs.push((path(), Reason::Missing));
                },
                _ if buffer.uri.is_none() => {
                    errs.push((path(), Reason::Missing));
                },
                _ => {},
            }
        }
    }

    if errs.is_empty() {
        Ok(match config.validation_strategy {
            ValidationStrategy::Minimal => unvalidated.validate_minimally()?,
            ValidationStrategy::Complete => unvalidated.validate_completely()?,
            ValidationStrategy::Skip => unreachable!(),
        })
    } else {
        Err(Error::Validation(errs))
    }
}

fn import_standard<'a>(
    data: &'a [u8],
    config: &Config,
    base_path: &Path,
) -> Result<(Gltf, Buffers), Error> {
    let unvalidated = Gltf::from_slice(data)?;
    let gltf = validate_standard(unvalidated, config)?;
    let bin = None;
    let mut buffers = Buffers(vec![]);
    for buffer in load_external_buffers(base_path, &gltf, bin)? {
        buffers.0.push(buffer);
    }
    Ok((gltf, buffers))
}

fn import_binary<'a>(
    data: &'a [u8],
    config: &Config,
    base_path: &Path,
) -> Result<(Gltf, Buffers), Error> {
    let gltf::Glb { json, bin, .. } = gltf::Glb::from_slice(data)?;
    let unvalidated = Gltf::from_slice(json)?;
    let bin = bin.map(|x| x.to_vec());
    let gltf = validate_binary(unvalidated, config, bin.is_some())?;
    let mut buffers = Buffers(vec![]);
    for buffer in load_external_buffers(base_path, &gltf, bin)? {
        buffers.0.push(buffer);
    }
    Ok((gltf, buffers))
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
        Error::MalformedJson(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<Vec<(json::Path, validation::Error)>> for Error {
    fn from(errs: Vec<(json::Path, validation::Error)>) -> Error {
        Error::Validation(errs)
    }
}

impl From<gltf::Error> for Error {
    fn from(err: gltf::Error) -> Error {
        Error::Gltf(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Error {
        Error::Base64Decoding(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            Base64Decoding(_) => "Base 64 decoding failed",
            BufferLength(_) => "Loaded buffer does not match required length",
            ExtensionDisabled(_) => "Asset requires a disabled extension",
            ExtensionUnsupported(_) => "Assets requires an unsupported extension",
            IncompatibleVersion(_) => "Asset is not glTF version 2.0",
            Io(_) => "I/O error",
            Gltf(_) => "Error from gltf crate",
            MalformedJson(_) => "Malformed .gltf / .glb JSON",
            Validation(_) => "Asset failed validation tests",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        use self::Error::*;
        match *self {
            MalformedJson(ref err) => Some(err),
            Io(ref err) => Some(err),
            _ => None,
        }
    }
}
