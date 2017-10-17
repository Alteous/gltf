use json;
use std::{error, fmt};

use Gltf;

/// Validation error type.
#[derive(Debug)]
pub struct Error {
    errs: Vec<(json::Path, json::validation::Error)>,
}

/// Represents `glTF` that hasn't been validated yet.
pub struct Unvalidated(pub(crate) Gltf);

impl Unvalidated {
    /// Returns the unvalidated JSON.
    pub fn as_json(&self) -> &json::Root {
        self.0.as_json()
    }

    /// Skip validation.  **Using this is highly recommended against** as
    /// malformed glTF assets might lead to program panics, huge values, NaNs
    /// and general evil deeds.
    ///
    /// # Panics
    ///
    /// This function does not panic, but might cause an inherent panic later in
    /// your program during reading of the malformed asset.
    pub fn skip_validation(self) -> Gltf {
        self.0
    }

    /// Validates only the invariants required for the library to function safely.
    pub fn validate_minimally(self) -> Result<Gltf, Error> {
        use json::validation::Validate;
        let mut errs = vec![];
        {
            let json = self.as_json();
            json.validate_minimally(
                json,
                json::Path::new,
                &mut |path, err| errs.push((path(), err)),
            );
        }
        if errs.is_empty() {
            Ok(self.0)
        } else {
            Err(Error { errs })
        }
    }

    /// Validates the data against the `glTF` 2.0 specification.
    pub fn validate_completely(self) -> Result<Gltf, Error> {
        use json::validation::Validate;
        let mut errs = vec![];
        {
            let json = self.as_json();
            json.validate_minimally(
                json,
                json::Path::new,
                &mut |path, err| errs.push((path(), err)),
            );
            json.validate_completely(
                json,
                json::Path::new,
                &mut |path, err| errs.push((path(), err)),
            );
        }
        if errs.is_empty() {
            Ok(self.0)
        } else {
            Err(Error { errs })
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "validation failed"
    }

    fn cause(&self) -> Option<&error::Error> {
        self.errs.first().map(|&(_, ref err)| err as &error::Error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use error::Error;
        write!(f, "{}", self.description())
    }
}
