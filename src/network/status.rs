use std::convert::*;
#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use strum::{IntoEnumIterator, ParseError};
use strum_macros::{AsRefStr, EnumString, FromRepr};

// import re
// code = list(set(re.findall("<code>([0-9]+)\s+(\w+)</code>", src)))
// code.sort()
// print("\n".join([f"\t{i[1]} = {i[0]}," for i in code if i[1] != "unused"]))

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Copy, Clone, FromRepr, EnumString, AsRefStr)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Status {
    Continue = 100,
    Processing = 102,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    Found = 302,
    Unauthorized = 401,
    Forbidden = 403,
    Conflict = 409,
    Gone = 410,
    Locked = 423,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        String::from(self.as_ref())
    }
}

impl TryFrom<usize> for Status {
    type Error = ParseError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match Self::from_repr(value) {
            Some(value) => Ok(value),
            None => Err(ParseError::VariantNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_status_operations() {
        for code in Status::iter() {
            assert_eq!(Ok(code), Status::try_from(code as usize));
            assert_eq!(Ok(code), Status::from_str(&code.to_string()))
        }
    }
}
