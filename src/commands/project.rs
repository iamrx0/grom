use crate::core::config::Config;
use crate::core::utils;
use std::io;

pub fn create(project_name: String, config: Config) -> Result<(), std::io::Error> {
    let path = format!("{}/projects/{}/.start.md", config.core.note_dir, project_name);

    if utils::path_exists(&path) {
        Err(io::Error::new(io::ErrorKind::AlreadyExists, "Project already exists."))
    } else {
        utils::ensure_all_dirs(&path)?;
        utils::save_file(&path)
    }
}

pub fn open(project_name: String, config: Config) -> Result<(), std::io::Error> {
    let project_base = format!("{}/projects/{}/start.md", config.core.note_dir, project_name);
    if utils::path_exists(&project_base) {
        utils::open_file(&config.core.editor, &project_base)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Project does not exist."))
    }
}

pub fn interactive_selecion(config: Config) -> Result<(), std::io::Error> {
    let project = utils::select_project(&config.core.note_dir)?;
    utils::open_file(&config.core.editor,&project)
}
