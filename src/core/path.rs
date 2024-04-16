use lazy_static::lazy_static;
use regex::Error as RError;
use regex::Regex;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum PathErrors {
    RegexError(RError),
}

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Path {
    value: String,
    regex: Regex,
}

lazy_static! {
    static ref PATH_ARG_REGEX: Regex = Regex::new(r"(:\w+)").unwrap();
}

impl Path {
    pub fn parse(value: String) -> Result<Self, PathErrors> {
        let parsed_path = PATH_ARG_REGEX.replace_all(&value, r"([^/]+)") + "/?$";
        match Regex::new(&parsed_path) {
            Ok(regex) => Ok(Self { value, regex }),
            Err(error) => Err(PathErrors::RegexError(error)),
        }
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

    impl PartialEq for Path {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value && self.regex.as_str() == other.regex.as_str()
        }
    }

    #[test]
    fn successful_path_parsing() {
        let cases = vec!["basic string", "http://my.url/test/path/:id"];

        for case in cases {
            let solution = Ok(Path {
                value: String::from(case),
                regex: Regex::new(
                    format!("{}/?$", PATH_ARG_REGEX.replace_all(case, r"([^/]+)")).as_str(),
                )
                .unwrap(),
            });

            let path = Path::parse(case.to_string());
            assert_eq!(path, solution);
        }
    }

    #[test]
    fn syntax_error_path_parsing() {
        let cases: Vec<&str> = vec!["try (this"];

        for case in cases {
            let path = Path::parse(case.to_string());
            assert_eq!(
                path,
                Err(PathErrors::RegexError(RError::Syntax(format!(
                    "regex parse error:\n    {case}/?$\n        ^\nerror: unclosed group"
                ))))
            );
        }
    }
}
