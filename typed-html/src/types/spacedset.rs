use std::collections::BTreeSet;
use std::convert::{TryFrom, TryInto};
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
/// # use std::convert::{TryFrom, TryInto};
/// use axohtml::types::{Class, SpacedSet};
///
/// # fn main() -> Result<(), &'static str> {
/// let classList: SpacedSet<Class> = "foo bar baz".try_into()?;
/// let classList: SpacedSet<Class> = ["foo", "bar", "baz"].try_into()?;
/// let classList: SpacedSet<Class> = ("foo", "bar", "baz").try_into()?;
///
/// let classList1: SpacedSet<Class> = "foo bar foo".try_into()?;
/// let classList2: SpacedSet<Class> = "bar foo bar".try_into()?;
/// assert_eq!(classList1, classList2);
/// # Ok(()) }
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SpacedSet<A: Ord>(BTreeSet<A>);

impl<A: Ord> SpacedSet<A> {
    /// Construct an empty `SpacedSet`.
    pub fn new() -> Self {
        SpacedSet(BTreeSet::new())
    }

    /// Add a value to the `SpacedSet`, converting it as necessary.
    ///
    /// Panics if the conversion fails.
    pub fn add<T: TryInto<A>>(&mut self, value: T) -> bool
    where
        <T as TryInto<A>>::Error: Debug,
    {
        self.0.insert(value.try_into().unwrap())
    }

    /// Add a value to the `SpacedSet`, converting it as necessary.
    ///
    /// Returns an error if the conversion fails.
    pub fn try_add<T: TryInto<A>>(&mut self, value: T) -> Result<bool, <T as TryInto<A>>::Error> {
        Ok(self.0.insert(value.try_into()?))
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

impl<A: Ord + FromStr> FromStr for SpacedSet<A>
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

impl<'a, A> TryFrom<&'a str> for SpacedSet<A>
where
    A: Ord + FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.split_whitespace().map(FromStr::from_str).collect()
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

impl<'a, 'b, A> TryFrom<(&'a str, &'b str)> for SpacedSet<A>
where
    A: Ord + FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0)?);
        list.insert(FromStr::from_str(s.1)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, A> TryFrom<(&'a str, &'b str, &'c str)> for SpacedSet<A>
where
    A: Ord + FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0)?);
        list.insert(FromStr::from_str(s.1)?);
        list.insert(FromStr::from_str(s.2)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, 'd, A> TryFrom<(&'a str, &'b str, &'c str, &'d str)> for SpacedSet<A>
where
    A: Ord + FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0)?);
        list.insert(FromStr::from_str(s.1)?);
        list.insert(FromStr::from_str(s.2)?);
        list.insert(FromStr::from_str(s.3)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, 'd, 'e, A> TryFrom<(&'a str, &'b str, &'c str, &'d str, &'e str)> for SpacedSet<A>
where
    A: Ord + FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0)?);
        list.insert(FromStr::from_str(s.1)?);
        list.insert(FromStr::from_str(s.2)?);
        list.insert(FromStr::from_str(s.3)?);
        list.insert(FromStr::from_str(s.4)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, A> TryFrom<(&'a str, &'b str, &'c str, &'d str, &'e str, &'f str)>
    for SpacedSet<A>
where
    A: Ord + FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0)?);
        list.insert(FromStr::from_str(s.1)?);
        list.insert(FromStr::from_str(s.2)?);
        list.insert(FromStr::from_str(s.3)?);
        list.insert(FromStr::from_str(s.4)?);
        list.insert(FromStr::from_str(s.5)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, A>
    TryFrom<(
        &'a str,
        &'b str,
        &'c str,
        &'d str,
        &'e str,
        &'f str,
        &'g str,
    )> for SpacedSet<A>
where
    A: Ord + FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0)?);
        list.insert(FromStr::from_str(s.1)?);
        list.insert(FromStr::from_str(s.2)?);
        list.insert(FromStr::from_str(s.3)?);
        list.insert(FromStr::from_str(s.4)?);
        list.insert(FromStr::from_str(s.5)?);
        list.insert(FromStr::from_str(s.6)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, A>
    TryFrom<(
        &'a str,
        &'b str,
        &'c str,
        &'d str,
        &'e str,
        &'f str,
        &'g str,
        &'h str,
    )> for SpacedSet<A>
where
    A: Ord + FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str, &str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.insert(FromStr::from_str(s.0)?);
        list.insert(FromStr::from_str(s.1)?);
        list.insert(FromStr::from_str(s.2)?);
        list.insert(FromStr::from_str(s.3)?);
        list.insert(FromStr::from_str(s.4)?);
        list.insert(FromStr::from_str(s.5)?);
        list.insert(FromStr::from_str(s.6)?);
        list.insert(FromStr::from_str(s.7)?);
        Ok(list)
    }
}

macro_rules! spacedset_from_array {
    ($num:tt) => {
        impl<'a, A> TryFrom<[&'a str; $num]> for SpacedSet<A>
        where
            A: Ord + FromStr,
        {
            type Error = <A as FromStr>::Err;
            fn try_from(s: [&str; $num]) -> Result<Self, Self::Error> {
                s.iter().map(|s| FromStr::from_str(*s)).collect()
            }
        }
    };
}
spacedset_from_array!(1);
spacedset_from_array!(2);
spacedset_from_array!(3);
spacedset_from_array!(4);
spacedset_from_array!(5);
spacedset_from_array!(6);
spacedset_from_array!(7);
spacedset_from_array!(8);
spacedset_from_array!(9);
spacedset_from_array!(10);
spacedset_from_array!(11);
spacedset_from_array!(12);
spacedset_from_array!(13);
spacedset_from_array!(14);
spacedset_from_array!(15);
spacedset_from_array!(16);
spacedset_from_array!(17);
spacedset_from_array!(18);
spacedset_from_array!(19);
spacedset_from_array!(20);
spacedset_from_array!(21);
spacedset_from_array!(22);
spacedset_from_array!(23);
spacedset_from_array!(24);
spacedset_from_array!(25);
spacedset_from_array!(26);
spacedset_from_array!(27);
spacedset_from_array!(28);
spacedset_from_array!(29);
spacedset_from_array!(30);
spacedset_from_array!(31);
spacedset_from_array!(32);
