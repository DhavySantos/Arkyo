use std::str::FromStr;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Eq, Clone)]
pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PATCH" => Ok(Method::PATCH),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(format!("Invalid HTTP method: {}", s)),
        }
    }
}
