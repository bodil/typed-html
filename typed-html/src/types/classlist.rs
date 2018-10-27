use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Error, Formatter};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use super::Class;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ClassList(BTreeSet<Class>);

impl ClassList {
    pub fn new() -> Self {
        ClassList(BTreeSet::new())
    }
}

impl Default for ClassList {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<Class> for ClassList {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Class>,
    {
        ClassList(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a Class> for ClassList {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a Class>,
    {
        ClassList(iter.into_iter().cloned().collect())
    }
}

impl FromIterator<String> for ClassList {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        ClassList(iter.into_iter().map(Class::new).collect())
    }
}

impl<'a> FromIterator<&'a str> for ClassList {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        ClassList(iter.into_iter().map(Class::new).collect())
    }
}

impl<'a> From<&'a str> for ClassList {
    fn from(s: &'a str) -> Self {
        Self::from_iter(s.split_whitespace().map(Class::new))
    }
}

impl Deref for ClassList {
    type Target = BTreeSet<Class>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ClassList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for ClassList {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut it = self.0.iter().peekable();
        while let Some(class) = it.next() {
            Display::fmt(class, f)?;
            if it.peek().is_some() {
                Display::fmt(" ", f)?;
            }
        }
        Ok(())
    }
}

impl Debug for ClassList {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl From<(&str, &str)> for ClassList {
    fn from(s: (&str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(Class::new(s.0));
        list.insert(Class::new(s.1));
        list
    }
}

impl From<(&str, &str, &str)> for ClassList {
    fn from(s: (&str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(Class::new(s.0));
        list.insert(Class::new(s.1));
        list.insert(Class::new(s.2));
        list
    }
}

impl From<(&str, &str, &str, &str)> for ClassList {
    fn from(s: (&str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(Class::new(s.0));
        list.insert(Class::new(s.1));
        list.insert(Class::new(s.2));
        list.insert(Class::new(s.3));
        list
    }
}

impl From<(&str, &str, &str, &str, &str)> for ClassList {
    fn from(s: (&str, &str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(Class::new(s.0));
        list.insert(Class::new(s.1));
        list.insert(Class::new(s.2));
        list.insert(Class::new(s.3));
        list.insert(Class::new(s.4));
        list
    }
}

impl From<(&str, &str, &str, &str, &str, &str)> for ClassList {
    fn from(s: (&str, &str, &str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(Class::new(s.0));
        list.insert(Class::new(s.1));
        list.insert(Class::new(s.2));
        list.insert(Class::new(s.3));
        list.insert(Class::new(s.4));
        list.insert(Class::new(s.5));
        list
    }
}

impl From<(&str, &str, &str, &str, &str, &str, &str)> for ClassList {
    fn from(s: (&str, &str, &str, &str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(Class::new(s.0));
        list.insert(Class::new(s.1));
        list.insert(Class::new(s.2));
        list.insert(Class::new(s.3));
        list.insert(Class::new(s.4));
        list.insert(Class::new(s.5));
        list.insert(Class::new(s.6));
        list
    }
}

impl From<(&str, &str, &str, &str, &str, &str, &str, &str)> for ClassList {
    fn from(s: (&str, &str, &str, &str, &str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(Class::new(s.0));
        list.insert(Class::new(s.1));
        list.insert(Class::new(s.2));
        list.insert(Class::new(s.3));
        list.insert(Class::new(s.4));
        list.insert(Class::new(s.5));
        list.insert(Class::new(s.6));
        list.insert(Class::new(s.7));
        list
    }
}

macro_rules! classlist_from_array {
    ($num:tt) => {
        impl From<[&str; $num]> for ClassList {
            fn from(s: [&str; $num]) -> Self {
                Self::from_iter(s.into_iter().map(|s| Class::new(*s)))
            }
        }
    };
}
classlist_from_array!(1);
classlist_from_array!(2);
classlist_from_array!(3);
classlist_from_array!(4);
classlist_from_array!(5);
classlist_from_array!(6);
classlist_from_array!(7);
classlist_from_array!(8);
classlist_from_array!(9);
classlist_from_array!(10);
classlist_from_array!(11);
classlist_from_array!(12);
classlist_from_array!(13);
classlist_from_array!(14);
classlist_from_array!(15);
classlist_from_array!(16);
classlist_from_array!(17);
classlist_from_array!(18);
classlist_from_array!(19);
classlist_from_array!(20);
classlist_from_array!(21);
classlist_from_array!(22);
classlist_from_array!(23);
classlist_from_array!(24);
classlist_from_array!(25);
classlist_from_array!(26);
classlist_from_array!(27);
classlist_from_array!(28);
classlist_from_array!(29);
classlist_from_array!(30);
classlist_from_array!(31);
classlist_from_array!(32);
