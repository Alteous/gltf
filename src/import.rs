use base64;
use buffer;
use image;
use image_crate;
use std::{fs, io};

use image_crate::ImageFormat::{JPEG as Jpeg, PNG as Png};
use std::path::Path;
use {Document, Error, Gltf, Result};

/// Return type of `import`.
type Import = (Document, Vec<buffer::Data>, Vec<image::Data>);

/// Represents the set of URI schemes the importer supports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Scheme<'a> {
    /// `data:[<media type>];base64,<data>`.
    Data(Option<&'a str>, &'a str),

    /// `file:[//]<absolute file path>`.
    ///
    /// Note: The file scheme does not implement authority.
    File(&'a str),

    /// `../foo`, etc.
    Relative,

    /// Placeholder for an unsupported URI scheme identifier.
    Unsupported,
}

impl<'a> Scheme<'a> {
    fn parse<'s>(uri: &'s str) -> Scheme<'s> {
        if uri.contains(":") {
            if uri.starts_with("data:") {
                let match0 = &uri["data:".len()..].split(";base64,").nth(0);
                let match1 = &uri["data:".len()..].split(";base64,").nth(1);
                if match1.is_some() {
                    Scheme::Data(Some(match0.unwrap()), match1.unwrap())
                } else if match0.is_some() {
                    Scheme::Data(None, match0.unwrap())
                } else {
                    Scheme::Unsupported
                }
            } else if uri.starts_with("file://") {
                Scheme::File(&uri["file://".len()..])
            } else if uri.starts_with("file:") {
                Scheme::File(&uri["file:".len()..])
            } else {
                Scheme::Unsupported
            }
        } else {
            Scheme::Relative
        }
    }

    fn read(base: &Path, uri: &str) -> Result<Vec<u8>> {
        match Scheme::parse(uri) {
            Scheme::Data(_, base64) => base64::decode(&base64).map_err(Error::Base64),
            Scheme::File(path) => read_to_end(path),
            Scheme::Relative => read_to_end(base.join(uri)),
            Scheme::Unsupported => Err(Error::UnsupportedScheme),
        }
    }
}

fn read_to_end<P>(path: P) -> Result<Vec<u8>>
where P: AsRef<Path>
{
    use io::Read;
    let file = fs::File::open(path.as_ref()).map_err(Error::Io)?;
    let length = file.metadata().map(|x| x.len()).unwrap_or(0);
    let mut reader = io::BufReader::new(file);
    let mut data = Vec::with_capacity(length as usize);
    reader.read_to_end(&mut data).map_err(Error::Io)?;
    Ok(data)
}

/// Import the buffer data referenced by a glTF document.
pub fn import_buffer_data(
    document: &Document,
    base: &Path,
    mut blob: Option<Vec<u8>>,
) -> Result<Vec<buffer::Data>> {
    let mut buffers = Vec::new();
    for buffer in document.buffers() {
        let data = match buffer.source() {
            buffer::Source::Uri(uri) => Scheme::read(base, uri),
            buffer::Source::Bin => blob.take().ok_or(Error::MissingBlob),
        }?;
        if data.len() < buffer.length() {
            return Err(
                Error::BufferLength {
                    buffer: buffer.index(),
                    expected: buffer.length(),
                    actual: data.len(),
                }
            );
        }
        buffers.push(buffer::Data(data));
    }
    Ok(buffers)
}

/// Import the image data referenced by a glTF document.
pub fn import_image_data(
    document: &Document,
    base: &Path,
    buffer_data: &[buffer::Data],
) -> Result<Vec<image::Data>> {
    let mut images = Vec::new();
    for image in document.images() {
        match image.source() {
            image::Source::Uri { uri, mime_type } => {
                match Scheme::parse(uri) {
                    Scheme::Data(Some(annoying_case), base64) => {
                        let format = match annoying_case.as_ref() {
                            "image/png" => Png,
                            "image/jpeg" => Jpeg,
                            _ => return Err(Error::UnsupportedImageEncoding),
                        };
                        let encoded_image = base64::decode(&base64).map_err(Error::Base64)?;
                        let decoded_image = image_crate::load_from_memory_with_format(&encoded_image, format)?;
                        images.push(image::Data::new(decoded_image));
                        continue;
                    },
                    Scheme::Unsupported => return Err(Error::UnsupportedScheme),
                    _ => {},
                }
                let encoded_image = Scheme::read(base, uri)?;
                let encoded_format =  match mime_type {
                    Some("image/png") => Png,
                    Some("image/jpeg") => Jpeg,
                    Some(_) => return Err(Error::UnsupportedImageEncoding),
                    None => match uri.rsplit(".").next() {
                        Some("png") => Png,
                        Some("jpg") | Some("jpeg") => Jpeg,
                        _ => return Err(Error::UnsupportedImageEncoding),
                    },
                };
                let decoded_image = image_crate::load_from_memory_with_format(&encoded_image, encoded_format)?;
                images.push(image::Data::new(decoded_image));
            },
            image::Source::View { view, mime_type } => {
                let parent_buffer_data = &buffer_data[view.buffer().index()].0;
                let begin = view.offset();
                let end = begin + view.length();
                let encoded_image = &parent_buffer_data[begin..end];
                let encoded_format = match mime_type {
                    "image/png" => Png,
                    "image/jpeg" => Jpeg,
                    _ => return Err(Error::UnsupportedImageEncoding)
                };
                let decoded_image = image_crate::load_from_memory_with_format(encoded_image, encoded_format)?;
                images.push(image::Data::new(decoded_image));
            },
        }
    }

    Ok(images)
}

fn import_impl(path: &Path) -> Result<Import> {
    let base = path.parent().unwrap_or(Path::new("./"));
    let file = fs::File::open(path).map_err(Error::Io)?;
    let reader = io::BufReader::new(file);
    let Gltf { document, blob } = Gltf::from_reader(reader)?;
    let buffer_data = import_buffer_data(&document, base, blob)?;
    let image_data = import_image_data(&document, base, &buffer_data)?;
    let import = (document, buffer_data, image_data);
    Ok(import)
}

/// Import some glTF 2.0 from the file system.
///
/// ```
/// # extern crate gltf;
/// # fn run() -> Result<(), gltf::Error> {
/// # let path = "examples/Box.gltf";
/// # #[allow(unused)]
/// let (document, buffers, images) = gltf::import(path)?;
/// # Ok(())
/// # }
/// # fn main() {
/// #     run().expect("test failure");
/// # }
/// ```
pub fn import<P>(path: P) -> Result<Import>
    where P: AsRef<Path>
{
    import_impl(path.as_ref())
}
