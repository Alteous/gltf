use {buffer, json};

use Document;

pub use json::accessor::ComponentType as DataType;
pub use json::accessor::Type as Dimensions;

#[cfg(feature = "utils")]
#[doc(inline)]
pub use self::util::Item;

#[cfg(feature = "utils")]
#[doc(inline)]
pub use self::util::Iter;

/// Any type that can appear in an Accessor.
#[cfg(feature = "utils")]
pub mod util {
   use std::mem;
    use byteorder::{LE, ByteOrder};
    use std::marker::PhantomData;

    /// Represents items that can be read by an `Accessor`.
    pub trait Item {
        /// Create an object of this type from a byte slice.
        fn from_slice(slice: &[u8]) -> Self;
    }

    /// Visits the items in an `Accessor`.
    #[derive(Copy, Clone, Debug)]
    pub struct Iter<'a, T> {
        stride: usize,
        data: &'a [u8],
        _phantom: PhantomData<T>,
    }

    impl Item for i8 {
        fn from_slice(slice: &[u8]) -> Self {
            slice[0] as i8
        }
    }

    impl Item for i16 {
        fn from_slice(slice: &[u8]) -> Self {
            LE::read_i16(slice)
        }
    }

    impl Item for u8 {
        fn from_slice(slice: &[u8]) -> Self {
            slice[0]
        }
    }

    impl Item for u16 {
        fn from_slice(slice: &[u8]) -> Self {
            LE::read_u16(slice)
        }
    }

    impl Item for u32 {
        fn from_slice(slice: &[u8]) -> Self {
            LE::read_u32(slice)
        }
    }

    impl Item for f32 {
        fn from_slice(slice: &[u8]) -> Self {
            LE::read_f32(slice)
        }
    }

    impl<T: Item> Item for [T; 2] {
        fn from_slice(slice: &[u8]) -> Self {
            assert!(slice.len() >= 2 * mem::size_of::<T>());
            [T::from_slice(slice),
             T::from_slice(&slice[mem::size_of::<T>() ..])]
        }
    }

    impl<T: Item> Item for [T; 3] {
        fn from_slice(slice: &[u8]) -> Self {
            assert!(slice.len() >= 3 * mem::size_of::<T>());
            [T::from_slice(slice),
             T::from_slice(&slice[1 * mem::size_of::<T>() ..]),
             T::from_slice(&slice[2 * mem::size_of::<T>() ..])]
        }
    }

    impl<T: Item> Item for [T; 4] {
        fn from_slice(slice: &[u8]) -> Self {
            assert!(slice.len() >= 4 * mem::size_of::<T>());
            [T::from_slice(slice),
             T::from_slice(&slice[1 * mem::size_of::<T>() ..]),
             T::from_slice(&slice[2 * mem::size_of::<T>() ..]),
             T::from_slice(&slice[3 * mem::size_of::<T>() ..])]
        }
    }

    impl<'a, T> Iter<'a, T> {
        /// Constructor.
        pub fn new(
            accessor: super::Accessor,
            buffer_data: &'a [u8],
        ) -> Iter<'a, T> {
            debug_assert_eq!(mem::size_of::<T>(), accessor.size());
            debug_assert!(mem::size_of::<T>() > 0);
            let view = accessor.view();
            let stride = view.stride().unwrap_or(mem::size_of::<T>());
            debug_assert!(stride >= mem::size_of::<T>());
            let start = view.offset() + accessor.offset();
            let end = start + stride * (accessor.count() - 1) + mem::size_of::<T>();
            let data = &buffer_data[start .. end];
            Iter { stride, data, _phantom: PhantomData }
        }
    }

    impl<'a, T: Item> ExactSizeIterator for Iter<'a, T> {}
    impl<'a, T: Item> Iterator for Iter<'a, T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            let stride = if self.data.len() >= self.stride {
                Some(self.stride)
            } else if self.data.len() >= mem::size_of::<T>() {
                Some(mem::size_of::<T>())
            } else {
                None
            };
            if let Some(stride) = stride {
                let (val, data) = self.data.split_at(stride);
                let val = T::from_slice(val);
                self.data = data;
                Some(val)
            } else {
                None
            }
        }

        fn nth(&mut self, nth: usize) -> Option<Self::Item> {
            if let Some(val_data) = self.data.get(nth * self.stride ..) {
                if val_data.len() >= mem::size_of::<T>() {
                    let val = T::from_slice(val_data);
                    self.data = &val_data[self.stride.min(val_data.len()) ..];
                    Some(val)
                } else {
                    None
                }
            } else {
                None
            }
        }

        fn last(self) -> Option<Self::Item> {
            if self.data.len() >= mem::size_of::<T>() {
                self.data
                    .get((self.data.len() - 1) / self.stride * self.stride ..)
                    .map(T::from_slice)
            } else {
                None
            }
        }

        fn count(self) -> usize {
            self.size_hint().0
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let hint = self.data.len() / self.stride
                + (self.data.len() % self.stride >= mem::size_of::<T>()) as usize;
            (hint, Some(hint))
        }
    }
}

/// A typed view into a buffer view.
#[derive(Clone, Debug)]
pub struct Accessor<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::accessor::Accessor,

    /// The buffer view this accessor reads from.
    view: buffer::View<'a>,
}

impl<'a> Accessor<'a> {
    /// Constructs an `Accessor`.
    pub(crate) fn new(
        document: &'a Document,
        index: usize,
        json: &'a json::accessor::Accessor,
    ) -> Self {
        let view = document.views().nth(json.buffer_view.value()).unwrap();
        Self {
            document,
            index,
            json,
            view,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the size of each component that this accessor describes.
    pub fn size(&self) -> usize {
        self.data_type().size() * self.dimensions().multiplicity()
    }

    /// Returns the buffer view this accessor reads from.
    pub fn view(&self) -> buffer::View<'a> {
        self.document.views().nth(self.json.buffer_view.value()).unwrap()
    }

    /// Returns the offset relative to the start of the parent buffer view in bytes.
    pub fn offset(&self) -> usize {
        self.json.byte_offset as usize
    }

    /// Returns the number of components within the buffer view - not to be confused
    /// with the number of bytes in the buffer view.
    pub fn count(&self) -> usize {
        self.json.count as usize
    }

    /// Returns the data type of components in the attribute.
    pub fn data_type(&self) -> DataType {
        self.json.component_type.unwrap().0
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &json::Extras {
        &self.json.extras
    }

    /// Specifies if the attribute is a scalar, vector, or matrix.
    pub fn dimensions(&self) -> Dimensions {
        self.json.type_.unwrap()
    }

    /// Returns the minimum value of each component in this attribute.
    pub fn min(&self) -> Option<json::Value> {
        self.json.min.clone()
    }

    /// Returns the maximum value of each component in this attribute.
    pub fn max(&self) -> Option<json::Value> {
        self.json.max.clone()
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    pub fn name(&self) -> Option<&str> {
        self.json.name.as_ref().map(String::as_str)
    }

    /// Specifies whether integer data values should be normalized.
    pub fn normalized(&self) -> bool {
        self.json.normalized
    }

    /// Returns sparse storage of attributes that deviate from their initialization
    /// value.
    pub fn sparse(&self) -> Option<sparse::Sparse> {
        self.json.sparse.as_ref().map(|json| {
            sparse::Sparse::new(self.document, json)
        })
    }
}

/// Contains data structures for sparse storage.
pub mod sparse {
    use Document;
    use {buffer, json};

    /// The index data type.
    #[derive(Clone, Debug)]
    pub enum IndexType {
        /// Corresponds to `GL_UNSIGNED_BYTE`.
        U8 = 5121,

        /// Corresponds to `GL_UNSIGNED_SHORT`.
        U16 = 5123,

        /// Corresponds to `GL_UNSIGNED_INT`.
        U32 = 5125,
    }

    /// Indices of those attributes that deviate from their initialization value.
    pub struct Indices<'a> {
        /// The parent `Document` struct.
        document: &'a Document,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Indices,
    }

    impl<'a> Indices<'a> {
        /// Constructs `sparse::Indices`.
        pub(crate) fn new(
            document: &'a Document,
            json: &'a json::accessor::sparse::Indices,
        ) -> Self {
            Self {
                document: document,
                json: json,
            }
        }

        /// Returns the buffer view containing the sparse indices.
        pub fn view(&self) -> buffer::View<'a> {
            self.document.views().nth(self.json.buffer_view.value()).unwrap()
        }

        /// The offset relative to the start of the parent buffer view in bytes.
        pub fn offset(&self) -> u32 {
            self.json.byte_offset
        }

        /// The data type of each index.
        pub fn index_type(&self) -> IndexType {
            match self.json.component_type.unwrap().0 {
                json::accessor::ComponentType::U8 => IndexType::U8,
                json::accessor::ComponentType::U16 => IndexType::U16,
                json::accessor::ComponentType::U32 => IndexType::U32,
                _ => unreachable!(),
            }
        }


        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
        }
    }

    /// Sparse storage of attributes that deviate from their initialization value.
    pub struct Sparse<'a> {
        /// The parent `Document` struct.
        document: &'a Document,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Sparse,
    }

    impl<'a> Sparse<'a> {
        /// Constructs `Sparse`.
        pub(crate) fn new(
            document: &'a Document,
            json: &'a json::accessor::sparse::Sparse,
        ) -> Self {
            Self {
                document: document,
                json: json,
            }
        }

        /// Returns the number of attributes encoded in this sparse accessor.
        pub fn count(&self) -> u32 {
            self.json.count
        }

        /// Returns an index array of size `count` that points to those accessor
        /// attributes that deviate from their initialization value.
        pub fn indices(&self) -> Indices<'a> {
            Indices::new(self.document, &self.json.indices)
        }

        /// Returns an array of size `count * number_of_components`, storing the
        /// displaced accessor attributes pointed by `indices`.
        pub fn values(&self) -> Values<'a> {
            Values::new(self.document, &self.json.values)
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
        }
    }

    /// Array of size `count * number_of_components` storing the displaced accessor
    /// attributes pointed by `accessor::sparse::Indices`.
    pub struct Values<'a> {
        /// The parent `Document` struct.
        document: &'a Document,

        /// The corresponding JSON struct.
        json: &'a json::accessor::sparse::Values,
    }

    impl<'a> Values<'a> {
        /// Constructs `sparse::Values`.
        pub(crate) fn new(
            document: &'a Document,
            json: &'a json::accessor::sparse::Values,
        ) -> Self {
            Self {
                document: document,
                json: json,
            }
        }

        /// Returns the buffer view containing the sparse values.
        pub fn view(&self) -> buffer::View {
            self.document.views().nth(self.json.buffer_view.value()).unwrap()
        }

        /// The offset relative to the start of the parent buffer view in bytes.
        pub fn offset(&self) -> u32 {
            self.json.byte_offset
        }

        /// Optional application specific data.
        pub fn extras(&self) -> &json::Extras {
            &self.json.extras
        }
    }


    impl IndexType {
        /// Returns the number of bytes this value represents.
        pub fn size(&self) -> usize {
            use self::IndexType::*;
            match *self {
                U8 => 1,
                U16 => 2,
                U32 => 4,
            }
        }
    }
}
