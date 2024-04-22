pub enum Status {
    Ok,
    NotFound,
    BadGateway,
}

impl Status {
    #[must_use]
    pub const fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::NotFound => 404,
            Self::BadGateway => 401,
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match &self {
            Self::Ok => String::from("OK"),
            Self::NotFound => String::from("NOT FOUND"),
            Self::BadGateway => String::from("BAD GATEWAY"),
        }
    }
}
