#![doc = include_str!("../README.md")]

mod config;
mod db;
mod error;
mod redirect;
mod status;

use crate::config::Config;
use crate::error::Error;
use ehttpd::http::{Request, Response};
use ehttpd::Server;
use std::process;

/// Routes a HTTP request to the associated implementation
fn request_handler(request: Request) -> Response {
    match (request.method.as_ref(), request.target.as_ref()) {
        (b"GET", b"/status") => status::status_get(&request),
        _ => redirect::redirect_any(&request),
    }
}

pub fn main() {
    /// The fallible main function code
    fn fallible() -> Result<(), Error> {
        // Setup periodical database refresh and load config
        db::reload_periodically()?;
        let config = Config::load()?;

        // Start the server
        let server = Server::with_request_response(config.server.connection_limit, request_handler);
        server.accept(&config.server.address)?;
        unreachable!("`server.accept` can never exit gracefully")
    }

    // Execute the fallible code and pretty print any error
    if let Err(e) = fallible() {
        // Print error and backtrace
        eprintln!("Fatal error: {e}");
        if e.has_backtrace() {
            eprintln!("{}", e.backtrace);
        }

        // Exit with abnormal status code
        process::exit(1);
    }
}
