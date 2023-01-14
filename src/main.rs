#![doc = include_str!("../README.md")]

mod admin;
mod config;
mod db;
mod error;
mod redirect;
mod stats;

use crate::{config::Config, error::Error};
use rouille::router;
use std::process;

pub fn main() {
    /// The fallible main function code
    fn fallible() -> Result<(), Error> {
        // Setup periodical database refresh and load config
        db::reload_periodically()?;
        let config = Config::load()?;

        // Start Rouille
        rouille::start_server(config.server.address, |request| {
            router!(request,
                (GET) (/_admin) => {
                    admin::administer_get(request)
                },
                (POST) (/_admin) => {
                    admin::administer_post(request)
                },
                (GET) (/_stats) => {
                    stats::stats_get(request)
                },
                _ => {
                    redirect::redirect_any(request)
                }
            )
        });
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
