use serde::ser;
use serde_json;
use std;

use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::hash::Hash;

use {Path, Root};

/// Trait for validating glTF JSON data against the 2.0 specification.
pub trait Validate {
    /// Validates only the invariants required for the library to function safely.
    fn validate_minimally<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&Fn() -> Path, Error),
    {
        // nop
    }

    /// Validates the data against the glTF 2.0 specification.
    ///
    /// # Notes
    ///
    /// The caller must also call `validate_minimally()` for full validation.
    fn validate_completely<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&Fn() -> Path, Error),
    {
        // nop
    }
}

/// Specifies what kind of error occured during validation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// An index was found to be out of bounds.
    IndexOutOfBounds,

    /// An invalid value was identified.
    Invalid,

    /// Some required data has been omitted.
    Missing,
}

/// Specifies a type that has been pre-validated during deserialization or otherwise.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Checked<T> {
    /// The item is valid.
    Valid(T),

    /// The item is invalid.
    Invalid,
}

impl<T> Checked<T> {
    /// Converts from `Checked<T>` to `Checked<&T>`.
    pub fn as_ref(&self) -> Checked<&T> {
        match *self {
            Checked::Valid(ref item) => Checked::Valid(item),
            Checked::Invalid => Checked::Invalid,
        }
    }

    /// Takes ownership of the contained item if it is `Valid`.
    ///
    /// # Panics
    ///
    /// Panics if called on an `Invalid` item.
    pub fn unwrap(self) -> T {
        match self {
            Checked::Valid(item) => item,
            Checked::Invalid => panic!("attempted to unwrap an invalid item"),
        }
    }
}

impl<T: Serialize> Serialize for Checked<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            Checked::Valid(ref item) => item.serialize(serializer),
            Checked::Invalid => Err(ser::Error::custom("invalid item")),
        }
    }
}

impl<T: Clone> Clone for Checked<T> {
    fn clone(&self) -> Self {
        match *self {
            Checked::Valid(ref item) => Checked::Valid(item.clone()),
            Checked::Invalid => Checked::Invalid,
        }
    }
}

impl<T: Copy> Copy for Checked<T> {}

impl<T: Default> Default for Checked<T> {
    fn default() -> Self {
        Checked::Valid(T::default())
    }
}

impl<T> Validate for Checked<T> {
    fn validate_minimally<P, R>(&self, _root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        match *self {
            Checked::Valid(_) => {},
            Checked::Invalid => report(&path, Error::Invalid),
        }
    }
}

impl<K: Eq + Hash + ToString + Validate, V: Validate> Validate for HashMap<K, V> {
    fn validate_minimally<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        for (key, value) in self.iter() {
            key.validate_minimally(root, || path().key(&key.to_string()), report);
            value.validate_minimally(root, || path().key(&key.to_string()), report);
        }
    }

    fn validate_completely<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        for (key, value) in self.iter() {
            key.validate_completely(root, || path().key(&key.to_string()), report);
            value.validate_completely(root, || path().key(&key.to_string()), report);
        }
    }
}

impl<T: Validate> Validate for Option<T> {
    fn validate_minimally<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        if let Some(value) = self.as_ref() {
            value.validate_minimally(root, path, report);
        }
    }

    fn validate_completely<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        if let Some(value) = self.as_ref() {
            value.validate_completely(root, path, report);
        }
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate_minimally<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        for (index, value) in self.iter().enumerate() {
            value.validate_minimally(root, || path().index(index), report);
        }
    }

    fn validate_completely<P, R>(&self, root: &Root, path: P, report: &mut R)
        where P: Fn() -> Path, R: FnMut(&Fn() -> Path, Error)
    {
        for (index, value) in self.iter().enumerate() {
            value.validate_completely(root, || path().index(index), report);
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IndexOutOfBounds => "Index out of bounds",
            Error::Invalid => "Invalid value",
            Error::Missing => "Missing data",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

// These types are assumed to be always valid.
impl Validate for bool {}
impl Validate for u32 {}
impl Validate for i32 {}
impl Validate for f32 {}
impl Validate for [f32; 3] {}
impl Validate for [f32; 4] {}
impl Validate for [f32; 16] {}
impl Validate for () {}
impl Validate for String {}
impl Validate for serde_json::Value {}
