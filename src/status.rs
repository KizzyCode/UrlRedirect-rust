//! Implements a simple status API

use ehttpd::http::{Request, Response, ResponseExt};
use serde::Serialize;

/// The possible status values
#[derive(Debug, Clone, Serialize)]
enum Status {
    /// The server is online
    Online,
}

/// Gets the current server status
pub fn status_get(_request: &Request) -> Response {
    // Serialize the status
    let status = Status::Online;
    let status_json = serde_json::to_string_pretty(&status).expect("failed to serialize stats");

    // Create the response
    let mut response = Response::new_200_ok();
    response.set_field("Content-Type", "application/json");
    response.set_body_data(status_json);
    response
}
