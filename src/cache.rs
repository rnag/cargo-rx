use crate::*;

use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;

/// Get the *local cache* directory where we store the *last replay* info
pub fn last_replay_path() -> PathBuf {
    home::cargo_home()
        .unwrap()
        .join(BINARY_NAME)
        .join("replays")
        .join(SETTINGS_TOML)
}

/// Return the *last replay* info, which includes *example name* along with
/// the *arguments* it was last called with.
pub fn get_last_replay() -> Result<ReplayConfig> {
    let bytes_data = fs::read(last_replay_path())?;
    Ok(toml::from_slice(&bytes_data)?)
}

/// Save the *last replay* info, which includes *example name* along with
/// the *arguments* it was last called with.
pub fn save_last_replay<'a>(name: &'a str, args: &[&'a str]) -> Result<()> {
    let example = LastRun {
        name: name.to_string(),
        arguments: args.iter().map(|&s| s.to_owned()).collect(),
    };
    let config = ReplayConfig { last_run: example };

    let cache = last_replay_path();
    create_dir_all(cache.parent().unwrap())?;

    let data = toml::to_vec(&config)?;
    fs::write(cache, data)?;

    Ok(())
}
