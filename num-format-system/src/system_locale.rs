mod unix;
mod windows;

use std::collections::HashSet;

use num_format_core::Grouping;

use crate::error::Error;

/// TODO
#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SystemLocale {
    dec: String,
    grp: Grouping,
    inf: String,
    min: String,
    name: String,
    nan: String,
    pos: String,
    sep: Option<String>,
}

mod todo {
    use std::fmt;

    use super::*;

    impl fmt::Debug for SystemLocale {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fn inner(f: &mut fmt::Formatter, l: &str, s: &str) -> fmt::Result {
                write!(f, "  {}: \"{}\" [", l, s)?;
                for c in s.chars() {
                    for d in c.escape_unicode() {
                        write!(f, "{}", d)?;
                    }
                }
                writeln!(f, "]")
            }

            writeln!(f, "SystemLocale {{")?;
            inner(f, "dec", self.decimal())?;
            writeln!(f, "  grp: {:?}", self.grouping())?;
            inner(f, "min", self.minus_sign())?;
            inner(f, "nam", self.name())?;
            inner(f, "pos", self.positive_sign())?;
            match self.separator() {
                Some(ref sep) => inner(f, "sep", sep)?,
                None => writeln!(f, "  sep: None")?,
            }
            writeln!(f, "}}")?;
            Ok(())
        }
    }
}

cfg_if! {
    if #[cfg(unix)] {
        impl SystemLocale {
            /// TODO
            pub fn new() -> Result<SystemLocale, Error> {
                SystemLocale::default()
            }

            /// TODO
            pub fn default() -> Result<SystemLocale, Error> {
                unix::new(None)
            }

            /// TODO
            pub fn from_name<S>(name: S) -> Result<SystemLocale, Error>
            where
                S: Into<String>,
            {
                unix::new(Some(name.into()))
            }

            /// TODO
            pub fn available_names() -> Result<HashSet<String>, Error> {
                Ok(unix::available_names())
            }
        }
    } else {
        impl SystemLocale {
            /// TODO
            pub fn new() -> Result<SystemLocale, Error> {
                SystemLocale::default()
            }

            /// TODO
            pub fn default() -> Result<SystemLocale, Error> {
                windows::default()
            }

            /// TODO
            pub fn from_name<S>(name: S) -> Result<SystemLocale, Error>
            where
                S: Into<String>,
            {
                windows::from_name(name)
            }

            /// TODO
            pub fn available_names() -> Result<HashSet<String>, Error> {
                windows::available_names()
            }
        }
    }
}

impl SystemLocale {
    /// TODO
    pub fn decimal(&self) -> &str {
        &self.dec
    }

    /// TODO
    pub fn grouping(&self) -> Grouping {
        self.grp
    }

    /// TODO
    pub fn infinity(&self) -> &str {
        &self.inf
    }

    /// TODO
    pub fn minus_sign(&self) -> &str {
        &self.min
    }

    /// TODO
    pub fn name(&self) -> &str {
        &self.name
    }

    /// TODO
    pub fn nan(&self) -> &str {
        &self.nan
    }

    /// TODO
    pub fn positive_sign(&self) -> &str {
        &self.pos
    }

    /// TODO
    pub fn separator(&self) -> Option<&str> {
        self.sep.as_ref().map(|s| s.as_ref())
    }

    #[cfg(unix)]
    /// TODO
    pub fn set_infinity<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        use num_format_core::constants::MAX_INF_LEN;

        let s = s.into();
        if s.len() > MAX_INF_LEN {
            return Err(Error::new("TODO"));
        }
        self.nan = s;
        Ok(())
    }

    #[cfg(unix)]
    /// TODO
    pub fn set_nan<S>(&mut self, s: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        use num_format_core::constants::MAX_NAN_LEN;

        let s = s.into();
        if s.len() > MAX_NAN_LEN {
            return Err(Error::new("TODO"));
        }
        self.nan = s;
        Ok(())
    }
}

impl std::str::FromStr for SystemLocale {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SystemLocale::from_name(s)
    }
}
