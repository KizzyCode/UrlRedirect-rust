//! Implements a simple stats API

use ehttpd::http::{
    request::Request,
    response::Response,
    responseext::{ResponseBodyExt, ResponseExt},
};
use serde::Serialize;

/// The possible status values
#[derive(Debug, Clone, Serialize)]
enum Status {
    /// The server is online
    Online,
}

/// The stats object
#[derive(Debug, Clone, Serialize)]
struct Stats {
    /// The server status
    pub status: Status,
}

/// Returns a simple stat object
pub fn stats_get(_request: &Request) -> Response {
    // Serialize the stats
    let stats = Stats { status: Status::Online };
    let stats_raw = serde_json::to_string_pretty(&stats).expect("failed to serialize stats");

    // Create the response
    let mut response = Response::new_200_ok();
    response.set_field("Content-Type", "application/json");
    response.set_body_data(stats_raw.into_bytes());
    response
}
