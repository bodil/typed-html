use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Display, Error, Formatter};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// A space separated list of values.
///
/// This type represents a list of non-unique values represented as a string of
/// values separated by spaces in HTML attributes. This is rarely used; a
/// [`SpacedSet`][SpacedSet] of unique values is much more common.
///
/// [SpacedSet]: struct.SpacedSet.html
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SpacedList<A>(Vec<A>);

impl<A> SpacedList<A> {
    /// Construct an empty `SpacedList`.
    pub fn new() -> Self {
        SpacedList(Vec::new())
    }

    /// Add a value to the `SpacedList`, converting it as necessary.
    ///
    /// Panics if the conversion fails.
    pub fn add<T: TryInto<A>>(&mut self, value: T)
    where
        <T as TryInto<A>>::Error: Debug,
    {
        self.0.push(value.try_into().unwrap())
    }

    /// Add a value to the `SpacedList`, converting it as necessary.
    ///
    /// Returns an error if the conversion fails.
    pub fn try_add<T: TryInto<A>>(&mut self, value: T) -> Result<(), <T as TryInto<A>>::Error> {
        self.0.push(value.try_into()?);
        Ok(())
    }
}

impl<A> Default for SpacedList<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A> FromIterator<A> for SpacedList<A> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        SpacedList(iter.into_iter().collect())
    }
}

impl<'a, A: 'a + Clone> FromIterator<&'a A> for SpacedList<A> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a A>,
    {
        SpacedList(iter.into_iter().cloned().collect())
    }
}

impl<'a, A> TryFrom<&'a str> for SpacedList<A>
where
    A: FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.split_whitespace().map(FromStr::from_str).collect()
    }
}

impl<A> Deref for SpacedList<A> {
    type Target = Vec<A>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A> DerefMut for SpacedList<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<A: Display> Display for SpacedList<A> {
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

impl<A: Debug> Debug for SpacedList<A> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<'a, 'b, A> TryFrom<(&'a str, &'b str)> for SpacedList<A>
where
    A: FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.push(FromStr::from_str(s.0)?);
        list.push(FromStr::from_str(s.1)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, A> TryFrom<(&'a str, &'b str, &'c str)> for SpacedList<A>
where
    A: FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.push(FromStr::from_str(s.0)?);
        list.push(FromStr::from_str(s.1)?);
        list.push(FromStr::from_str(s.2)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, 'd, A> TryFrom<(&'a str, &'b str, &'c str, &'d str)> for SpacedList<A>
where
    A: FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.push(FromStr::from_str(s.0)?);
        list.push(FromStr::from_str(s.1)?);
        list.push(FromStr::from_str(s.2)?);
        list.push(FromStr::from_str(s.3)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, 'd, 'e, A> TryFrom<(&'a str, &'b str, &'c str, &'d str, &'e str)> for SpacedList<A>
where
    A: FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.push(FromStr::from_str(s.0)?);
        list.push(FromStr::from_str(s.1)?);
        list.push(FromStr::from_str(s.2)?);
        list.push(FromStr::from_str(s.3)?);
        list.push(FromStr::from_str(s.4)?);
        Ok(list)
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, A> TryFrom<(&'a str, &'b str, &'c str, &'d str, &'e str, &'f str)>
    for SpacedList<A>
where
    A: FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.push(FromStr::from_str(s.0)?);
        list.push(FromStr::from_str(s.1)?);
        list.push(FromStr::from_str(s.2)?);
        list.push(FromStr::from_str(s.3)?);
        list.push(FromStr::from_str(s.4)?);
        list.push(FromStr::from_str(s.5)?);
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
    )> for SpacedList<A>
where
    A: FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.push(FromStr::from_str(s.0)?);
        list.push(FromStr::from_str(s.1)?);
        list.push(FromStr::from_str(s.2)?);
        list.push(FromStr::from_str(s.3)?);
        list.push(FromStr::from_str(s.4)?);
        list.push(FromStr::from_str(s.5)?);
        list.push(FromStr::from_str(s.6)?);
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
    )> for SpacedList<A>
where
    A: FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: (&str, &str, &str, &str, &str, &str, &str, &str)) -> Result<Self, Self::Error> {
        let mut list = Self::new();
        list.push(FromStr::from_str(s.0)?);
        list.push(FromStr::from_str(s.1)?);
        list.push(FromStr::from_str(s.2)?);
        list.push(FromStr::from_str(s.3)?);
        list.push(FromStr::from_str(s.4)?);
        list.push(FromStr::from_str(s.5)?);
        list.push(FromStr::from_str(s.6)?);
        list.push(FromStr::from_str(s.7)?);
        Ok(list)
    }
}

macro_rules! spacedlist_from_array {
    ($num:tt) => {
        impl<'a, A> TryFrom<[&'a str; $num]> for SpacedList<A>
        where
            A: FromStr,
        {
            type Error = <A as FromStr>::Err;
            fn try_from(s: [&str; $num]) -> Result<Self, Self::Error> {
                s.into_iter().map(|s| FromStr::from_str(*s)).collect()
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
