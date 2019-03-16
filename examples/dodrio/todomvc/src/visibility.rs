//! Visibility filtering.

use std::fmt;
use std::str::FromStr;

/// The visibility filtering for todo items.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    /// Show all todos.
    All,
    /// Show only active, incomplete todos.
    Active,
    /// Show only inactive, completed todos.
    Completed,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Visibility::All
    }
}

impl FromStr for Visibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Visibility::All),
            "active" => Ok(Visibility::Active),
            "completed" => Ok(Visibility::Completed),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.label().to_lowercase())
    }
}

impl Visibility {
    /// Get a string label for this visibility.
    pub fn label(self) -> &'static str {
        match self {
            Visibility::All => "All",
            Visibility::Active => "Active",
            Visibility::Completed => "Completed",
        }
    }
}
