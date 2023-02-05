//! Implements the admin API

use ehttpd::{
    bytes::Source,
    http::{Request, Response, ResponseExt},
};

/// Creates a 403 response
fn response_403(request: &Request) -> Response {
    // Create a basic response
    let mut response = Response::new_403_forbidden();
    response.set_content_length(0);

    // Append a human readable body if this is a GET-request
    if request.method.eq(b"GET") {
        // Build the HTML response to display something to the user
        let html = r#"
            <html>
                <head>
                    <title>403 Forbidden</title>
                </head>
                <body>
                    You are not allowed to access the requested resource (403 Forbidden)
                </body>
            </html>"#;

        // Set the body (this also overwrites the `set_content_length(0)` from above)
        response.set_body(Source::from(html)).expect("failed to set static data as request body");
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
