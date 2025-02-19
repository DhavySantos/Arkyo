#![cfg_attr(debug_assertions, allow(dead_code, unused))]
pub mod component;
pub mod http;
pub mod util;

pub mod prellude {
    pub use crate::component::Middleware;
    pub use crate::component::Route;
    pub use crate::http::Method;
    pub use crate::http::Request;
    pub use crate::http::Response;
    pub use crate::http::Server;
}
