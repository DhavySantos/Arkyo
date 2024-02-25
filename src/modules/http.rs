use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub enum Method {
    Delete,
    Post,
    Get,
    Put,
}

impl FromStr for Method {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DELETE" => Ok(Method::Delete),
            "POST" => Ok(Method::Post),
            "GET" => Ok(Method::Get),
            "PUT" => Ok(Method::Put),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Ok => "200 Ok".to_string(),
            Status::NotFound => "404 Not Found".to_string(),
            Status::BadRequest => "400 Bad Request".to_string(),
            Status::InternalServerError => "500 Internal Server Error".to_string(),
        }
    }
}