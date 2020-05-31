use std::collections::HashMap;
use std::convert::TryFrom;

use anyhow::anyhow;

pub fn parse_optional_param<'a, T>(
    parameters: &'a HashMap<&str, &str>,
    param_name: &str,
) -> anyhow::Result<Option<T>, anyhow::Error>
where
    T: TryFrom<&'a str, Error = anyhow::Error>,
{
    parameters
        .get(param_name)
        .map(|s| T::try_from(*s))
        .transpose()
}

pub fn parse_mandatory_param<'a, T>(
    parameters: &'a HashMap<&str, &str>,
    param_name: &str,
) -> anyhow::Result<T, anyhow::Error>
where
    T: TryFrom<&'a str, Error = anyhow::Error>,
{
    let &param = parameters
        .get(param_name)
        .ok_or_else(|| anyhow!("{} parameter required", param_name))?;
    T::try_from(param)
}

pub fn parse_mandatory_u32(
    parameters: &HashMap<&str, &str>,
    param_name: &str,
) -> anyhow::Result<u32> {
    Ok(parameters
        .get(param_name)
        .ok_or_else(|| anyhow!(f!("{param_name} parameter not found")))?
        .parse()
        .map_err(|_| anyhow!(f!("{param_name} parameter invalid integer")))?)
}

pub fn parse_optional_u32(
    parameters: &HashMap<&str, &str>,
    param_name: &str,
) -> anyhow::Result<Option<u32>> {
    parameters
        .get(param_name)
        .map(|s| {
            s.parse()
                .map_err(|_| anyhow!(f!("{param_name} parameter invalid integer")))
        })
        .transpose()
}

// Generate an enum with an TryFrom<&str> implementation that converts from a string to a enum variant
// enum will also derive Debug, Copy, Clone, PartialEq
macro_rules! parameter {
    ($enumname:ident, [$($variant: ident), *]) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum $enumname {
            $(
                $variant,
            )*
        }

        impl std::convert::TryFrom<&str> for $enumname {
            type Error = anyhow::Error;

            fn try_from(text: &str) -> anyhow::Result<Self, Self::Error> {
                match text {
                    $(
                        stringify!($variant) => Ok($enumname::$variant),
                    )*
                    _ => bail!(concat!("invalid ", stringify!($enumname), " {}"), text),
                }
            }
        }
    };
    ($enumname:ident, [$($variant: ident), *], default=$default_variant: ident) => {
        parameter!($enumname, [$($variant), *]);
        impl Default for $enumname {
            fn default() -> Self {
                $enumname::$default_variant
            }
        }
    };
}

pub struct GeneratedString {
    pub string: String,
    indent: usize,
}

impl GeneratedString {
    pub fn new() -> Self {
        GeneratedString {
            string: String::new(),
            indent: 0,
        }
    }

    pub fn line<T: AsRef<str>>(&mut self, line: T) {
        let indent = " ".repeat(self.indent);
        let content = line.as_ref();
        self.string.push_str(&f!("{indent}{content}\n"));
    }

    pub fn empty_line(&mut self) {
        self.string.push_str("\n");
    }

    pub fn indent_right(&mut self) {
        self.indent += 4;
    }

    pub fn indent_left(&mut self) {
        self.indent = self.indent.saturating_sub(4);
    }
}
