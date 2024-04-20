pub mod core;
pub mod network;

pub mod prelude { 
    pub use crate::core::{
        Middleware,
        Server,
        Route,
    };
    
    pub use crate::network::{
        Response,
        Request,
        Method,
        Status,
    };
}
