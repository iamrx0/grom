use std::io;

use crate::core::config::Config;
use crate::core::git;

pub fn init(remote: String, config: Config) -> Result<(), io::Error> {
    git::init_sync(config.core.note_dir.clone(), remote)
}

pub fn push(message: String, config: Config) -> Result<(), io::Error> {
    git::push_changes(config.core.note_dir, message)
}

pub fn pull(config: Config) -> Result<(), io::Error> {
    git::pull_changes(config.core.note_dir)
}
