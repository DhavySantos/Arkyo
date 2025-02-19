use crate::http::{Request, Response};
use crate::util::Location;
use std::sync::Arc;

#[derive(Clone)]
pub struct Middleware {
    callback: Arc<dyn Fn(&mut Request, &mut Response)>,
    location: Location,
}

impl Middleware {
    pub fn new<T: Fn(&mut Request, &mut Response) + 'static>(
        location: Location, callback: T,
    ) -> Self {
        let callback = Arc::new(callback);
        Self { location, callback }
    }

    pub fn handle(&self, req: &mut Request, res: &mut Response) {
        (self.callback)(req, res)
    }

    pub fn is_match(&self, location: &str) -> bool {
        self.location.is_match(location)
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for Middleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Middleware")
            .field("location", &self.location)
            .finish()
    }
}
