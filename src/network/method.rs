#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Method { 
    Get,
    Post,
    Patch,
    Delete,
}

impl Method {
    pub fn from_str(input: &str) -> Result<Self, ()> {
        match input.to_uppercase().as_str() {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            _ => Err(())
        }
    }
}
