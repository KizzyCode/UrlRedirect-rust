//! Implements the URL redirect

use crate::db::DB;
use ehttpd::{
    bytes::Source,
    http::{Request, Response, ResponseExt},
};
use std::str;

/// Resolves an URL mapping
fn lookup_any(url: &str) -> Option<String> {
    // Acquire/recover database lock
    let database = match DB.read() {
        Ok(database) => database,
        Err(database) => database.into_inner(),
    };

    // Resolve entry
    database.redirects.get(url).map(Clone::clone)
}

/// Creates a 404 response
fn response_404(request: &Request) -> Response {
    // Create a basic response
    let mut response = Response::new_404_notfound();
    response.set_content_length(0);

    // Append a human readable body if this is a GET-request
    if request.method.eq(b"GET") {
        // Build the HTML response to display something to the user
        let html = r#"
            <html>
                <head>
                    <title>404 Not Found</title>
                </head>
                <body>
                    The requested resource cannot be found on the server (404 Not Found)
                </body>
            </html>"#;

        // Set the body (this also overwrites the `set_content_length(0)` from above)
        response.set_body(Source::from(html)).expect("failed to set static data as request body");
    }
    response
}

/// Redirects the URL request
pub fn redirect_any(request: &Request) -> Response {
    // Parse the URL as UTF-8
    let Ok(url) = str::from_utf8(&request.target) else {
        eprintln!("invalid redirect key: target is not UTF-8");
        return response_404(request);
    };

    // Lookup the redirect
    let Some(target) = lookup_any(url) else {
        eprintln!("invalid redirect key: {url}");
        return response_404(request);
    };

    // Create the response
    let mut response = Response::new_status_reason(307, "Temporary Redirect");
    response.set_field("Location", target.as_bytes());
    response.set_content_length(0);
    response.set_connection_close();

    // Append a human readable body if this is a GET-request
    if request.method.eq(b"GET") {
        // Build the HTML response to display something to the user in case the 307 is ignored
        let html = format!(
            r#"
            <html>
                <head>
                    <title>Redirecting...</title>
                </head>
                <body>
                    <meta http-equiv="Refresh" content="0; URL={target}" />
                    This site will redirect you to <a href="{target}">{target}</a>.
                </body>
            </html>"#
        );

        // Set the body (this also overwrites the `set_content_length(0)` from above)
        response.set_body(Source::from(html)).expect("failed to set in-memory HTML data as body");
    }
    response
}
