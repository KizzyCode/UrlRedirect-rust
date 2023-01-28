//! Implements the admin API

use ehttpd::http::{
    request::Request,
    response::Response,
    responseext::{ResponseBodyExt, ResponseExt},
};

/// Handle an admin API request
pub fn administer_get(_request: &Request) -> Response {
    let mut response = Response::new_403_forbidden();
    response.set_body_static(b"Forbidden");
    response
}

/// Handle an admin API request
pub fn administer_post(_request: &Request) -> Response {
    let mut response = Response::new_403_forbidden();
    response.set_body_static(b"Forbidden");
    response
}
