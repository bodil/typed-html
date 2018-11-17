use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Error, Formatter};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// A space separated set of unique values.
///
/// This type represents a set of unique values represented as a string of
/// values separated by spaces in HTML attributes.
///
/// # Examples
///
/// ```
/// # extern crate typed_html;
/// # use std::convert::From;
/// use typed_html::types::{Class, SpacedSet};
///
/// # fn main() {
/// let classList: SpacedSet<Class> = "foo bar baz".into();
/// let classList: SpacedSet<Class> = ["foo", "bar", "baz"].into();
/// let classList: SpacedSet<Class> = ("foo", "bar", "baz").into();
///
/// let classList1: SpacedSet<Class> = "foo bar foo".into();
/// let classList2: SpacedSet<Class> = "bar foo bar".into();
/// assert_eq!(classList1, classList2);
/// # }
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SpacedSet<A: Ord>(BTreeSet<A>);

impl<A: Ord> SpacedSet<A> {
    /// Construct an empty `SpacedSet`.
    pub fn new() -> Self {
        SpacedSet(BTreeSet::new())
    }
}

impl<A: Ord> Default for SpacedSet<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: Ord> FromIterator<A> for SpacedSet<A> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        SpacedSet(iter.into_iter().collect())
    }
}

impl<'a, A: 'a + Ord + Clone> FromIterator<&'a A> for SpacedSet<A> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a A>,
    {
        SpacedSet(iter.into_iter().cloned().collect())
    }
}

impl<'a, A: Ord + FromStr> FromStr for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    type Err = <A as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<Vec<A>, Self::Err> =
            s.split_whitespace().map(|s| FromStr::from_str(s)).collect();
        result.map(Self::from_iter)
    }
}

impl<'a, A: Ord + FromStr> From<&'a str> for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    fn from(s: &'a str) -> Self {
        Self::from_iter(s.split_whitespace().map(|s| FromStr::from_str(s).unwrap()))
    }
}

impl<A: Ord> Deref for SpacedSet<A> {
    type Target = BTreeSet<A>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A: Ord> DerefMut for SpacedSet<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<A: Ord + Display> Display for SpacedSet<A> {
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

impl<A: Ord + Debug> Debug for SpacedSet<A> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<A: Ord + FromStr> From<(&str, &str)> for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    fn from(s: (&str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0).unwrap());
        list.insert(FromStr::from_str(s.1).unwrap());
        list
    }
}

impl<A: Ord + FromStr> From<(&str, &str, &str)> for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    fn from(s: (&str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0).unwrap());
        list.insert(FromStr::from_str(s.1).unwrap());
        list.insert(FromStr::from_str(s.2).unwrap());
        list
    }
}

impl<A: Ord + FromStr> From<(&str, &str, &str, &str)> for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    fn from(s: (&str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0).unwrap());
        list.insert(FromStr::from_str(s.1).unwrap());
        list.insert(FromStr::from_str(s.2).unwrap());
        list.insert(FromStr::from_str(s.3).unwrap());
        list
    }
}

impl<A: Ord + FromStr> From<(&str, &str, &str, &str, &str)> for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    fn from(s: (&str, &str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0).unwrap());
        list.insert(FromStr::from_str(s.1).unwrap());
        list.insert(FromStr::from_str(s.2).unwrap());
        list.insert(FromStr::from_str(s.3).unwrap());
        list.insert(FromStr::from_str(s.4).unwrap());
        list
    }
}

impl<A: Ord + FromStr> From<(&str, &str, &str, &str, &str, &str)> for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    fn from(s: (&str, &str, &str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0).unwrap());
        list.insert(FromStr::from_str(s.1).unwrap());
        list.insert(FromStr::from_str(s.2).unwrap());
        list.insert(FromStr::from_str(s.3).unwrap());
        list.insert(FromStr::from_str(s.4).unwrap());
        list.insert(FromStr::from_str(s.5).unwrap());
        list
    }
}

impl<A: Ord + FromStr> From<(&str, &str, &str, &str, &str, &str, &str)> for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    fn from(s: (&str, &str, &str, &str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0).unwrap());
        list.insert(FromStr::from_str(s.1).unwrap());
        list.insert(FromStr::from_str(s.2).unwrap());
        list.insert(FromStr::from_str(s.3).unwrap());
        list.insert(FromStr::from_str(s.4).unwrap());
        list.insert(FromStr::from_str(s.5).unwrap());
        list.insert(FromStr::from_str(s.6).unwrap());
        list
    }
}

impl<A: Ord + FromStr> From<(&str, &str, &str, &str, &str, &str, &str, &str)> for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    fn from(s: (&str, &str, &str, &str, &str, &str, &str, &str)) -> Self {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0).unwrap());
        list.insert(FromStr::from_str(s.1).unwrap());
        list.insert(FromStr::from_str(s.2).unwrap());
        list.insert(FromStr::from_str(s.3).unwrap());
        list.insert(FromStr::from_str(s.4).unwrap());
        list.insert(FromStr::from_str(s.5).unwrap());
        list.insert(FromStr::from_str(s.6).unwrap());
        list.insert(FromStr::from_str(s.7).unwrap());
        list
    }
}

macro_rules! spacedlist_from_array {
    ($num:tt) => {
        impl<A: Ord + FromStr> From<[&str; $num]> for SpacedSet<A>
        where
            <A as FromStr>::Err: Debug,
        {
            fn from(s: [&str; $num]) -> Self {
                Self::from_iter(s.into_iter().map(|s| FromStr::from_str(*s).unwrap()))
            }
        }
    };
}
spacedlist_from_array!(1);
spacedlist_from_array!(2);
spacedlist_from_array!(3);
spacedlist_from_array!(4);
spacedlist_from_array!(5);
spacedlist_from_array!(6);
spacedlist_from_array!(7);
spacedlist_from_array!(8);
spacedlist_from_array!(9);
spacedlist_from_array!(10);
spacedlist_from_array!(11);
spacedlist_from_array!(12);
spacedlist_from_array!(13);
spacedlist_from_array!(14);
spacedlist_from_array!(15);
spacedlist_from_array!(16);
spacedlist_from_array!(17);
spacedlist_from_array!(18);
spacedlist_from_array!(19);
spacedlist_from_array!(20);
spacedlist_from_array!(21);
spacedlist_from_array!(22);
spacedlist_from_array!(23);
spacedlist_from_array!(24);
spacedlist_from_array!(25);
spacedlist_from_array!(26);
spacedlist_from_array!(27);
spacedlist_from_array!(28);
spacedlist_from_array!(29);
spacedlist_from_array!(30);
spacedlist_from_array!(31);
spacedlist_from_array!(32);
