use crate::core::{Middleware, Route};

#[derive(Clone)]
pub enum Pipeline { 
    Middleware(Middleware),
    Route(Route)
}

