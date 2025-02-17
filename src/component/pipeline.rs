use super::{Middleware, Route};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub enum Pipeline {
    Middleware(Middleware),
    Route(Route),
}
