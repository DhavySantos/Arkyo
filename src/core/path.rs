use regex::Regex;

pub enum PathErrors {

}

#[derive(Clone)]
pub struct Path {
	value: String,
	regex: Regex,
}

impl Path {
	pub fn parse(value: String) -> Result<Self, PathErrors> {
		let regex = Regex::new(r"(:\w+)").unwrap();
        let regex = regex.replace_all(&value, r"([^/]+)") + "/?$";
        let regex = Regex::new(&regex).unwrap();

		Ok(Self {
			value,
			regex,
		})
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
