#![doc = include_str!("../README.md")]

mod admin;
mod config;
mod db;
mod error;
mod redirect;
mod stats;

use ehttpd::Server;

use crate::{config::Config, error::Error};
use std::process;

pub fn main() {
    /// The fallible main function code
    fn fallible() -> Result<(), Error> {
        // Setup periodical database refresh and load config
        db::reload_periodically()?;
        let config = Config::load()?;

        // Initialize the server
        let server: Server = Server::new(
            &config.server.address,
            config.server.connection_soft_limit,
            config.server.connection_hard_limit,
        )?;

        // Start the server loop
        server.exec(|request| match (request.method.as_ref(), request.target.as_ref()) {
            (b"GET", b"/_admin") => admin::administer_get(request),
            (b"POST", b"/_admin") => admin::administer_post(request),
            (b"GET", b"/_stats") => stats::stats_get(request),
            _ => redirect::redirect_any(request),
        })?;

        // `server.exec` can never successfully return; it's `Ok`-type is literally `Infallible`
        unreachable!("`server.exec` can never successfully return")
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
