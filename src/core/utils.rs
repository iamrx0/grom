use std::{
    fs::{self, OpenOptions},
    io::{self, ErrorKind},
    path::Path,
    process::Command,
};

pub fn save_file(file: &str) -> Result<(), io::Error> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file)?;
    Ok(())
}

pub fn open_file(cmd: &str, file: &str) -> Result<(), io::Error> {
    Command::new(cmd).arg(file).status()?;
    Ok(())
}

pub fn ensure_all_dirs(path: &str) -> Result<(), io::Error> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        match fs::create_dir_all(parent) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.exists()
}

pub fn find_projects(dir: &str) -> Result<Vec<(String, String)>, io::Error> {
    let mut projects = Vec::new();
    let dir = Path::new(&dir);

    let entries = fs::read_dir(dir).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            let start_md_path = path.join("start.md");
            if start_md_path.exists() {
                if let Some(parent) = path.file_name().and_then(|n| n.to_str()) {
                    projects.push((
                        String::from(start_md_path.to_str().unwrap()),
                        parent.to_string(),
                    ))
                }
            }
        }
    }
    Ok(projects)
}

pub fn select_project(note_dir: &str) -> Result<String, io::Error> {
    let projects_dir = format!("{}/projects", note_dir);
    let projects = find_projects(&projects_dir)?;
    if projects.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No Projects found.",
        ));
    }
    cliclack::intro(console::style(" Grom ").on_cyan().black()).unwrap();

    let items: Vec<_> = projects
        .iter()
        .map(|(name, path)| (name.clone(), path.clone(), String::new()))
        .collect();

    cliclack::select("Select a Project".to_string())
        .items(&items)
        .interact()
        .map_err(|_| io::Error::new(ErrorKind::Other, "Error selecting project"))
}
