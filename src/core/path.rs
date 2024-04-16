use regex::Regex;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum PathErrors {}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Path {
    value: String,
    regex: Regex,
}

impl Path {
    pub fn parse(value: String) -> Result<Self, PathErrors> {
        let regex = Regex::new(r"(:\w+)").unwrap();
        let regex = regex.replace_all(&value, r"([^/]+)") + "/?$";
        let regex = Regex::new(&regex).unwrap();

        Ok(Self { value, regex })
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn as_regex(&self) -> &Regex {
        &self.regex
    }

    pub fn is_match(&self, input: &str) -> bool {
        self.regex.is_match(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Formatter;

    impl PartialEq for Path {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    #[test]
    fn parse_path() {
        let cases = vec![(
            "aaa",
            Ok(Path {
                value: String::from("aaa"),
                regex: Regex::new("aaa").unwrap(),
            }),
        )];

        for (path_str, correct) in cases {
            let path = Path::parse(path_str.to_string());
            assert_eq!(path, correct);
        }
    }
}
