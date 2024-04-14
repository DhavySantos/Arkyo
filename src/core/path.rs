#[derive(Clone)]
pub struct Path {
	value: String
}

impl Path {
	pub fn parse(value: String) -> Option<Self> {
		Some(Self {
			value,
		})
	}

	pub fn as_str(&self) -> &str {
		self.value.as_str()
	}
}
