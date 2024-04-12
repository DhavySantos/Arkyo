#[derive(PartialEq, Debug, Clone)]
pub enum Method { 
    Get,
    Post,
    Patch,
    Delete,
}

impl Method {
    pub fn from_str(input: &str) -> Result<Method, ()> {
        match input.to_uppercase().as_str() {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PATCH" => Ok(Method::Patch),
            "DELETE" => Ok(Method::Delete),
            _ => Err(())
        }
    }
}
