use crate::http::{Method, Request, Response};
use crate::util::Location;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Route {
    callback: Arc<dyn Fn(&mut Request, &mut Response) + Send + Sync>,
    location: Location,
    method: Method,
}

impl Route {
    pub fn new<T: Fn(&mut Request, &mut Response) + Send + Sync + 'static>(
        location: Location, method: Method, callback: T,
    ) -> Self {
        let callback = Arc::new(callback);
        Self {
            location,
            callback,
            method,
        }
    }

    pub fn handle(&self, req: &mut Request, res: &mut Response) {
        (self.callback)(req, res);
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn is_match(&self, location: &str) -> bool {
        self.location.start_with(location)
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("location", &self.location)
            .field("method", &self.method)
            .finish()
    }
}
