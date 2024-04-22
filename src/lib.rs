pub mod core;
pub mod network;

pub mod prelude {
    pub use crate::core::{Middleware, Route, Server};

    pub use crate::network::{Method, Request, Response, Status};

    pub mod errors {
        pub use crate::core::path::Error as PathError;
        pub use crate::core::server::Error as ServerError;
        pub use crate::network::RequestError;
    }
}
