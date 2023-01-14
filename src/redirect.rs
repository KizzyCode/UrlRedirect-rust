//! Implements the URL redirect

use crate::db::DB;
use rouille::{Request, Response};

/// Resolves an URL mapping
fn lookup(url: &str) -> Option<String> {
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
    // Resolve the URL mapping
    let url = request.url();
    let Some(target) = lookup(&url) else {
        eprintln!("invalid redirect key: {url}");
        return Response::empty_404();
    };

    // Don't send any data if the the method is not a GET request
    if request.method() != "GET" {
        return Response::redirect_307(target);
    }

    // Build redirect response
    let html = format!(
        r#"<html>
            <head>
                <title>Redirecting...</title>
            </head>
            <body>
                <meta http-equiv="Refresh" content="0; URL={target}" />
                If the site does not redirect you automatically, please <a href="{target}">click here</a>.
            </body>
        </html>"#
    );
    Response::html(html).with_status_code(307).with_additional_header("Location", target)
}
