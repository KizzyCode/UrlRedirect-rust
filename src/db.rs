//! The URL database

use crate::error::Error;
use serde::Deserialize;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    env,
    ops::Deref,
    sync::RwLock,
    thread::{self, Builder},
    time::Duration,
};

/// A global database singleton
pub static DB: RwLock<Db> = RwLock::new(Db::new());

/// The URL database
#[derive(Debug, Clone, Deserialize)]
pub struct Db {
    /// The URL redirects
    pub redirects: BTreeMap<String, String>,
}
impl Db {
    /// Creates a new empty database
    pub const fn new() -> Self {
        Self { redirects: BTreeMap::new() }
    }

    /// Loads the database from the file
    pub fn load() -> Result<Self, Error> {
        // Get the path from the environment or fallback to a default path
        let path = match env::var("DB_FILE") {
            Ok(path) => Cow::Owned(path),
            Err(_) => Cow::Borrowed("url_db.toml"),
        };

        // Decode the database
        let data = std::fs::read(path.deref())?;
        let database: Self = toml::from_slice(&data)?;
        Ok(database)
    }
}

/// Starts a thread to reload the the `URL_DB` database periodically (all 10s)
pub fn reload_periodically() -> Result<(), Error> {
    /// The refresh worker
    fn reload() {
        /// Tries a refresh or logs the error
        fn try_reload() -> Result<(), Error> {
            // Load database and acquire/recover lock
            let database = Db::load()?;
            let mut global = match DB.write() {
                Ok(global) => global,
                Err(global) => global.into_inner(),
            };

            // Update database
            *global = database;
            Ok(())
        }

        // Loop forever
        loop {
            // Log any errors
            if let Err(e) = try_reload() {
                eprintln!("Failed to reload URL database: {e}");
            }

            // Sleep 10s
            const INTERVAL_10S: Duration = Duration::from_secs(10);
            thread::sleep(INTERVAL_10S);
        }
    }

    // Spawn the thread
    Builder::new().name("url_db::reload_periodically".to_string()).spawn(reload)?;
    Ok(())
}
