#![doc = include_str!("../README.md")]
// Clippy lints
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::panic)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unreachable)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::cognitive_complexity)]
#![forbid(unsafe_code)]

mod config;
mod db;
mod error;
mod redirect;
mod status;

use crate::config::Config;
use crate::error::Error;
use ehttpd::Server;
use std::process;

pub fn main() {
    /// The fallible main function code
    fn fallible() -> Result<(), Error> {
        // Setup periodical database refresh and load config
        db::reload_periodically()?;
        let config = Config::load()?;

        // Start the server
        let server = Server::with_request_response(config.server.connection_limit, move |request| {
            // Route the HTTP request to the associated implementation
            match (request.method.as_ref(), request.target.as_ref()) {
                (b"GET", b"/status") => status::status_get(&request),
                _ => redirect::redirect_any(&request),
            }
        });

        // Start the server and dispatch connections
        let Err(e) = server.accept(&config.server.address);
        Err(error!(with: e, "server task failed"))
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
