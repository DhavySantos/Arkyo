use once_cell::sync::Lazy;
use regex::Regex;
use std::error::Error;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct Location {
    value: String,
    regex: Regex,
    prefix_regex: Regex,
}

static R_PARAM: Lazy<Regex> = Lazy::new(|| Regex::new(r":(\w+)").unwrap());

impl Location {
    pub fn new(value: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        let value = value.into();
        let pattern = format!("^{}$", R_PARAM.replace_all(&value, r"([^/]+)"));
        let regex = Regex::new(&pattern)?;

        let prefix_pattern = format!("^{}", R_PARAM.replace_all(&value, r"([^/]+)"));
        let prefix_regex = Regex::new(&prefix_pattern)?;

        Ok(Self {
            value,
            regex,
            prefix_regex,
        })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn as_regex(&self) -> &Regex {
        &self.regex
    }

    pub fn is_match(&self, value: &str) -> bool {
        self.regex.is_match(value)
    }

    pub fn start_with(&self, value: &str) -> bool {
        self.prefix_regex.is_match(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_regex() {
        let loc = Location::new("/user/:id").unwrap();
        assert!(loc.is_match("/user/123"));
        assert!(!loc.is_match("/user/"));
        assert!(!loc.is_match("/user/123/more"));

        let loc2 = Location::new("/:uuid/info").unwrap();
        assert!(loc2.is_match("/abcd-efgh-1234/info"));
        assert!(!loc2.is_match("/abcd-efgh-1234"));
    }

    #[test]
    fn test_location_start_with() {
        let loc = Location::new("/user/:id").unwrap();
        assert!(loc.start_with("/user/123"));
        assert!(!loc.start_with("/user/"));
        assert!(!loc.start_with("/admin/"));
    }
}
