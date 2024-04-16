use std::str::FromStr;
use strum_macros::{EnumString, IntoStaticStr};

#[cfg_attr(test, derive(Debug))]
#[derive(Clone, EnumString, IntoStaticStr, PartialEq)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Method {
    Get,
    Post,
    Patch,
    Delete,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_method() {
        let cases = vec!["GET", "POST"];
    }
}
