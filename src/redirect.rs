//! Implements the URL redirect

use std::str;

use crate::db::DB;
use ehttpd::http::{
    request::Request,
    response::Response,
    responseext::{ResponseBodyExt, ResponseExt},
};

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

/// Redirects the URL request
pub fn redirect_any(request: &Request) -> Response {
    // Parse the URL as UTF-8
    let Ok(url) = str::from_utf8(&request.target) else {
        eprintln!("invalid redirect key: target is not UTF-8");
        return Response::new_404_notfound();
    };

    // Lookup the redirect
    let Some(target) = lookup_any(url) else {
        eprintln!("invalid redirect key: {url}");
        return Response::new_404_notfound();
    };

    // Don't send any data if the the method is not a GET request
    if !request.method.eq(b"GET") {
        return Response::new_status_reason(307, "Temporary Redirect");
    }

    // Build the HTML response to display something to the user in case the 307 is ignored
    let html = format!(
        r#"
        <html>
            <head>
                <title>Redirecting...</title>
            </head>
            <body>
                <meta http-equiv="Refresh" content="0; URL={target}" />
                If the site does not redirect you automatically, please <a href="{target}">click here</a>.
            </body>
        </html>"#
    );

    // Create and return the response
    let mut response = Response::new_status_reason(307, "Temporary Redirect");
    response.set_field("Location", target);
    response.set_field("Content-Type", "text/html");
    response.set_body_data(html.into_bytes());
    response
}
