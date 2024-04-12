pub enum Status { 
    Ok,
    NotFound,
    BadGateway,
}

impl Status { 

    pub fn code(&self) -> u16 {
        match self {
            Status::Ok => 200,
            Status::NotFound => 404,
            Status::BadGateway => 401,
        }
    }

}

impl ToString for Status {

    fn to_string(&self) -> String {
        match &self { 
            Status::Ok => String::from("OK"),
            Status::NotFound => String::from("NOT FOUND"),
            Status::BadGateway => String::from("BAD GATEWAY"),
        }
    }

}
