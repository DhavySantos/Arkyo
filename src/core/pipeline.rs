use crate::core::Middleware;
use crate::core::Route;

#[derive(Clone)]
pub enum Pipeline {
    Middleware(Middleware),
    Route(Route)
}
