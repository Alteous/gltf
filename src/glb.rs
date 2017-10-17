
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use byteorder::{LE, ReadBytesExt};

use std::{fmt, io};

/// Represents a Glb loader error.
#[derive(Debug)]
pub enum Error {
    /// Io error occured.
    Io(io::Error),
    /// Unsupported version.
    Version(u32),
    /// Magic says that file is not glTF.
    Magic([u8; 4]),
    /// Length specified in GLB header exceeeds that of slice.
    Length {
        /// length specified in GLB header.
        length: u32,
        /// Actual length of data read.
        length_read: usize,
    },
    /// Stream ended before we could read the chunk.
    ChunkLength {
        /// chunkType error happened at.
        ty: ChunkType,
        /// chunkLength.
        length: u32,
        /// Actual length of data read.
        length_read: usize,
    },
    /// Chunk of this chunkType was not expected.
    ChunkType(ChunkType),
    /// Unknown chunk type.
    UnknownChunkType([u8; 4]),
}

/// The contents of a .glb file.
#[derive(Clone, Debug)]
pub struct Glb<'a> {
    /// The header section of the `.glb` file.
    pub header: Header,
    /// The JSON section of the `.glb` file.
    pub json: &'a [u8],
    /// The optional BIN section of the `.glb` file.
    pub bin: Option<&'a [u8]>,
}

/// The header section of a .glb file.
#[derive(Copy, Clone, Debug)]
pub struct Header {
    /// Must be `b"glTF"`.
    pub magic: [u8; 4],
    /// Must be `2`.
    pub version: u32,
    /// Must match the length of the parent .glb file.
    pub length: u32,
}

/// GLB chunk type.
#[derive(Copy, Clone, Debug)]
pub enum ChunkType {
    /// JSON chunk.
    Json,
    /// BIN\0 chunk.
    Bin,
}

/// Chunk header with no data read yet.
#[derive(Copy, Clone, Debug)]
struct ChunkHeader {
    /// The length of the chunk data in byte excluding the header.
    length: u32,
    /// Chunk type.
    ty: ChunkType,
}

impl Header {
    fn from_reader<R: io::Read>(mut reader: R) -> Result<Self, Error> {
        use self::Error::Io;
        let mut magic = [0; 4];
        reader.read_exact(&mut magic).map_err(Io)?;
        // We only validate magic as we don't care for version and length of
        // contents, the caller does.  Let them decide what to do next with
        // regard to version and length.
        if &magic == b"glTF" {
            Ok(Self {
                magic,
                version: reader.read_u32::<LE>().map_err(Io)?,
                length: reader.read_u32::<LE>().map_err(Io)?,
            })
        } else {
            Err(Error::Magic(magic))
        }
    }

    fn size_of() -> usize { 12 }
}

impl ChunkHeader {
    fn from_reader<R: io::Read>(mut reader: R) -> Result<Self, Error> {
        use self::Error::Io;
        let length = reader.read_u32::<LE>().map_err(Io)?;
        let mut ty = [0; 4];
        reader.read_exact(&mut ty).map_err(Io)?;
        let ty = match &ty {
            b"JSON" => Ok(ChunkType::Json),
            b"BIN\0" => Ok(ChunkType::Bin),
            _ => Err(Error::UnknownChunkType(ty)),
        }?;
        Ok(Self { length, ty })
    }
}

impl<'a> Glb<'a> {
    /// Splits loaded GLB into its three chunks.
    ///
    /// * Mandatory GLB header.
    /// * Mandatory JSON chunk.
    /// * Optional BIN chunk.
    pub fn from_slice(mut data: &'a [u8]) -> Result<Self, Error> {
        let header = Header::from_reader(&mut data)
            .and_then(|header| {
                let contents_length = header.length as usize - Header::size_of();
                if contents_length <= data.len() {
                    Ok(header)
                } else {
                    Err(Error::Length {
                        length: contents_length as u32,
                        length_read: data.len(),
                    })
                }
            })?;
        match header.version {
            2 => Self::from_v2(data).map(|(json, bin)| Glb { header, json, bin }),
            x => Err(Error::Version(x)),
        }
    }

    /// Does the loading job for you.  Provided buf will be cleared before new
    /// data will be written.  When error happens, if only header was read, buf
    /// will not be mutated, otherwise, buf will be empty.
    pub fn from_reader<R: io::Read>(
        mut reader: R,
        buf: &'a mut Vec<u8>,
    ) -> Result<Self, Error> {
        let header = Header::from_reader(&mut reader)?;
        match header.version {
            2 => {
                let glb_len = header.length - Header::size_of() as u32;
                buf.clear();
                buf.reserve(glb_len as usize);
                // SAFETY: We are doing unsafe operation on a user-supplied
                // container!  Make sure not to expose user to uninitialized
                // data if an error happens during reading.
                //
                // It is guaranteed by reserve's implementation that the reserve
                // call will make buf's capacity _at least_ buf_len.
                //
                // We do not read contents of the Vec unless it is fully
                // initialized.
                unsafe { buf.set_len(glb_len as usize) };
                if let Err(e) = reader.read_exact(buf).map_err(Error::Io) {
                    // SAFETY: It is safe to not run destructors because u8 has
                    // none.
                    unsafe { buf.set_len(0) };
                    Err(e)
                } else {
                    Self::from_v2(buf).map(|(json, bin)| Glb { header, json, bin })
                }
            }
            x => Err(Error::Version(x))
        }
    }

    fn from_v2(mut data: &'a [u8]) -> Result<(&'a [u8], Option<&'a [u8]>), Error> {
        let (json, mut data) = ChunkHeader::from_reader(&mut data)
            .and_then(|json_h| if let ChunkType::Json = json_h.ty {
                Ok(json_h)
            } else {
                Err(Error::ChunkType(json_h.ty))
            })
            .and_then(|json_h| if json_h.length as usize <= data.len() {
                Ok(json_h)
            } else {
                Err(Error::ChunkLength {
                    ty: json_h.ty,
                    length: json_h.length,
                    length_read: data.len(),
                })
            })
            // We have verified that json_h.length is no greater than that of
            // data.len().
            .map(|json_h| data.split_at(json_h.length as usize))?;

        let bin = if data.len() > 0 {
            ChunkHeader::from_reader(&mut data)
                .and_then(|bin_h| if let ChunkType::Bin = bin_h.ty {
                    Ok(bin_h)
                } else {
                    Err(Error::ChunkType(bin_h.ty))
                })
                .and_then(|bin_h| if bin_h.length as usize <= data.len() {
                    Ok(bin_h)
                } else {
                    Err(Error::ChunkLength {
                        ty: bin_h.ty,
                        length: bin_h.length,
                        length_read: data.len(),
                    })
                })
                // We have verified that bin_h.length is no greater than that
                // of data.len().
                .map(|bin_h| data.split_at(bin_h.length as usize))
                .map(|(x, _)| Some(x))?
        } else {
            None
        };
        Ok((json, bin))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
         match *self {
             Error::Io(ref e) => e.description(),
             Error::Version(_) => "unsupported version",
             Error::Magic(_) => "not glTF magic",
             Error::Length { .. } => "could not completely read the object",
             Error::ChunkLength { ty, .. } => match ty {
                 ChunkType::Json => "JSON chunk length exceeds that of slice",
                 ChunkType::Bin => "BIN\\0 chunk length exceeds that of slice",
             },
             Error::ChunkType(ty) => match ty {
                 ChunkType::Json => "was not expecting JSON chunk",
                 ChunkType::Bin => "was not expecting BIN\\0 chunk",
             },
             Error::UnknownChunkType(_) => "unknown chunk type",
        }
    }
}
