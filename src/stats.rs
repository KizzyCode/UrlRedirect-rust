//! Implements a simple stats API

use rouille::{Request, Response};
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
    let stats = Stats { status: Status::Online };
    Response::json(&stats)
}
