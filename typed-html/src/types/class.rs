use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use super::Id;

/// A valid CSS class.
///
/// A CSS class is a non-empty string that starts with an alphanumeric character
/// and is followed by any number of alphanumeric characters and the
/// `_`, `-` and `.` characters.
///
/// See also [`Id`][Id].
///
/// [Id]: struct.Id.html
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Class(String);

impl Class {
    /// Construct a new class name from a string.
    ///
    /// Returns `Err` if the provided string is invalid.
    pub fn try_new<S: Into<String>>(id: S) -> Result<Self, &'static str> {
        let id = id.into();
        {
            let mut chars = id.chars();
            match chars.next() {
                None => return Err("class name cannot be empty"),
                Some(c) if !c.is_alphabetic() => {
                    return Err("class name must start with an alphabetic character")
                }
                _ => (),
            }
            for c in chars {
                if !c.is_alphanumeric() && c != '_' && c != '-' && c != '.' {
                    return Err(
                        "class name can only contain alphanumerics, dash, dot and underscore",
                    );
                }
            }
        }
        Ok(Class(id))
    }

    /// Construct a new class name from a string.
    ///
    /// Panics if the provided string is invalid.
    pub fn new<S: Into<String>>(id: S) -> Self {
        let id = id.into();
        Self::try_new(id.clone()).unwrap_or_else(|err| {
            panic!(
                "typed_html::types::Class: {:?} is not a valid class name: {}",
                id, err
            )
        })
    }
}

impl FromStr for Class {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Class::try_new(s)
    }
}

impl From<Id> for Class {
    fn from(id: Id) -> Self {
        Class(id.to_string())
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt(&self.0, f)
    }
}

impl Deref for Class {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
