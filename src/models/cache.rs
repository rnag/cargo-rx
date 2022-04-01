use serde::{Deserialize, Serialize};

/// Config which contains *replay* details on the *most recent* run example.
#[derive(Deserialize, Serialize, Default)]
pub struct ReplayConfig {
    /// Represents the *last run* example
    pub last_run: LastRun,
}

/// The details on the *last run* example.
#[derive(Deserialize, Serialize, Default)]
pub struct LastRun {
    /// Example *name*
    pub name: String,
    /// Example *arguments* passed in via command line
    pub arguments: Vec<String>,
}
