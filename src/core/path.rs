use lazy_static::lazy_static;
use regex::Error as RError;
use regex::Regex;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Errors {
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
    /// # Examples
    ///
    ///```
    ///let x = 5;
    ///assert_eq!(x, 5);
    ///```
    pub fn parse(value: String) -> Result<Self, Errors> {
        let parsed_path = PATH_ARG_REGEX.replace_all(&value, r"([^/]+)") + "/?";
        match Regex::new(&parsed_path) {
            Ok(regex) => Ok(Self { value, regex }),
            Err(error) => Err(Errors::RegexError(error)),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    #[must_use]
    pub const fn as_regex(&self) -> &Regex {
        &self.regex
    }

    #[must_use]
    pub fn is_match(&self, input: &str) -> bool {
        self.regex.is_match(input)
    }

    #[must_use]
    pub fn is_exact_match(&self, input: &str) -> bool {
        self.regex
            .find_at(input, 0)
            .map_or(false, |item| item.as_str().len() == input.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    impl PartialEq for Path {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value && self.regex.as_str() == other.regex.as_str()
        }
    }

    proptest! {
        #[test]
        fn no_crash_path_parse(s in "\\PC*"){
            let _ = Path::parse(s);
        }

        #[test]
        fn match_tests_with_arguments(path_str in "\\w+", var in "\\w+", suffix in "\\w+") {
            let path = Path::parse(format!("{path_str}/:{var}")).unwrap();

            let exact_request = format!("{path_str}/var_content/");
            let not_exact_request = format!("{path_str}/var_content/{suffix}");
            prop_assert!(path.is_match(&exact_request));
            prop_assert!(path.is_exact_match(&exact_request));

            prop_assert!(path.is_match(&not_exact_request));
            prop_assert!(!path.is_exact_match(&not_exact_request));
        }
    }

    #[test]
    fn successful_path_parsing() {
        let cases = vec!["basic string", "http://my.url/test/path/:id"];

        for case in cases {
            let solution = Ok(Path {
                value: String::from(case),
                regex: Regex::new(
                    format!("{}/?", PATH_ARG_REGEX.replace_all(case, r"([^/]+)")).as_str(),
                )
                .unwrap(),
            });

            let path = Path::parse(case.to_string());
            assert_eq!(path, solution);
        }
    }

    #[test]
    fn path_match_correcness() {
        let path = Path::parse("/profile/:picture".to_string()).unwrap();
        println!("{path:?}");

        let _ = path.as_str();

        assert!(path.is_match("/profile/1/asdsdf"));
        assert!(!path.is_exact_match("/profile/1/asdfasdf"));
    }

    #[test]
    fn syntax_error_path_parsing() {
        let cases: Vec<&str> = vec!["try (this"];

        for case in cases {
            let path = Path::parse(case.to_string());
            assert_eq!(
                path,
                Err(Errors::RegexError(RError::Syntax(format!(
                    "regex parse error:\n    {case}/?\n        ^\nerror: unclosed group"
                ))))
            );
        }
    }
}
