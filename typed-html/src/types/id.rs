use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;

use super::Class;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Id(String);

impl Id {
    // Construct a new ID from a string.
    //
    // Returns `None` if the provided string is invalid.
    pub fn try_new<S: Into<String>>(id: S) -> Result<Self, &'static str> {
        let id = id.into();
        {
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
        }
        Ok(Id(id))
    }

    // Construct a new ID from a string.
    //
    // Panics if the provided string is invalid.
    pub fn new<S: Into<String>>(id: S) -> Self {
        let id = id.into();
        Self::try_new(id.clone()).unwrap_or_else(|err| {
            panic!("typed_html::types::Id: {:?} is not a valid ID: {}", id, err)
        })
    }
}

impl TryFrom<String> for Id {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_new(s)
    }
}

impl<'a> TryFrom<&'a str> for Id {
    type Error = &'static str;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::try_new(s)
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
