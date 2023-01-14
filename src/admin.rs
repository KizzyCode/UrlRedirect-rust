//! Implements the admin API

use rouille::{Request, Response};

/// Handle an admin API request
pub fn administer_get(_request: &Request) -> Response {
    Response::text("Forbidden").with_status_code(403)
}

/// Handle an admin API request
pub fn administer_post(_request: &Request) -> Response {
    Response::text("Forbidden").with_status_code(403)
}
