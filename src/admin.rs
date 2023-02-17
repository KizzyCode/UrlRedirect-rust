//! Implements the admin API

use ehttpd::http::{Request, Response, ResponseExt};

/// Creates a 403 response
fn response_403(request: &Request) -> Response {
    // Create a basic response
    let mut response = Response::new_403_forbidden();
    response.set_content_length(0);

    // Append a human readable body if this is a GET-request
    if request.method.eq(b"GET") {
        // Set the body (this also overwrites the `set_content_length(0)` from above)
        const HTML_TEMPLATE: &str = include_str!("admin-403.html");
        response.set_body_data(HTML_TEMPLATE);
    }
    response
}

/// Handle an admin API request
pub fn administer_get(request: &Request) -> Response {
    response_403(request)
}

/// Handle an admin API request
pub fn administer_post(request: &Request) -> Response {
    response_403(request)
}
