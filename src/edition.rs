//! The edition of the language specification is detached from the implementation version. This
//! allows different editions to be run in the same project, by specifiying which sources belong
//! to which edition.

/// The edition that is used for a particular source.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Edition {
    /// Edition 1.
    Edition1
}

impl Edition {
    /// Returns a value that should be used for the default edition. This is always the latest.
    pub fn latest() -> Edition {
        Edition::Edition1
    }
}
