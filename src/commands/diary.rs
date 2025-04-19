use crate::core::config::Config;
use crate::core::utils;
use chrono::{Datelike, Local};

pub fn daily_diary(config: Config) -> Result<(), std::io::Error> {
    let today = Local::now();
    let file = format!(
        "{}/diary/{}/{}/week{}/{}.md",
        config.core.note_dir,
        today.year(),
        today.format("%B"),
        today.iso_week().week(),
        today.format("%m-%d-%Y")
    );
    if utils::path_exists(&file) {
        return utils::open_file(&config.core.editor, &file) 
    }
    utils::ensure_all_dirs(&file)?;
    utils::save_file(&file)?;
    utils::open_file(&config.core.editor, &file)
}

pub fn weekly_diary(config: Config) -> Result<(), std::io::Error>{
    let today = Local::now();
    let file = format!(
        "{}/diary/{}/{}/week{}/week.md",
        config.core.note_dir,
        today.year(),
        today.format("%B"),
        today.iso_week().week(),
    );
    if utils::path_exists(file.clone()) {
        return utils::open_file(&config.core.editor, &file)
    }
    utils::ensure_all_dirs(&file)?;
    utils::save_file(&file)?;
    utils::open_file(&config.core.editor, &file)
}

pub fn monthly_diary(config: Config) -> Result<(), std::io::Error>{
    let today = Local::now();
    let file = format!(
        "{}/diary/{}/{}/month.md",
        config.core.note_dir,
        today.year(),
        today.format("%B"),
    );
    if utils::path_exists(file.clone()) {
        return utils::open_file(&config.core.editor, &file)
    }
    utils::ensure_all_dirs(&file)?;
    utils::save_file(&file)?;
    utils::open_file(&config.core.editor, &file)
}
