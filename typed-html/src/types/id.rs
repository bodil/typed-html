use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use super::Class;

/// A valid HTML ID.
///
/// An ID is a non-empty string that starts with an alphanumeric character
/// and is followed by any number of alphanumeric characters and the
/// `_`, `-` and `.` characters.
///
/// See also [`Class`][Class].
///
/// [Class]: struct.Class.html
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Id(String);

impl Id {
    /// Construct a new ID from a string.
    ///
    /// Panics if the provided string is invalid.
    pub fn new<S: Borrow<str>>(id: S) -> Self {
        let id = id.borrow();
        Self::from_str(id).unwrap_or_else(|err| {
            panic!("typed_html::types::Id: {:?} is not a valid ID: {}", id, err)
        })
    }
}

impl FromStr for Id {
    type Err = &'static str;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let mut chars = id.chars();
        match chars.next() {
            None => return Err("ID cannot be empty"),
            Some(c) if !c.is_alphabetic() => {
                return Err("ID must start with an alphabetic character")
            }
            _ => (),
        }
        for c in chars {
            if !c.is_alphanumeric() && c != '_' && c != '-' && c != '.' {
                return Err("ID can only contain alphanumerics, dash, dot and underscore");
            }
        }
        Ok(Id(id.to_string()))
    }
}

impl<'a> TryFrom<&'a str> for Id {
    type Error = &'static str;
    fn try_from(str: &'a str) -> Result<Self, Self::Error> {
        Id::from_str(str)
    }
}

impl From<Class> for Id {
    fn from(c: Class) -> Self {
        Id(c.to_string())
    }
}

impl<'a> From<&'a Class> for Id {
    fn from(c: &'a Class) -> Self {
        Id(c.to_string())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt(&self.0, f)
    }
}

impl Deref for Id {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
