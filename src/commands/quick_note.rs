use crate::core::config::Config;
use crate::core::utils;

pub fn quick_note(note_name: &str, config: Config) -> Result<(),std::io::Error> {
    let filepath = format!("{}/quick-notes/{}.md", config.core.note_dir, note_name);
    if utils::path_exists(&filepath) {
        return utils::open_file(&config.core.editor, &filepath)
    }
    utils::ensure_all_dirs(&filepath)?;
    utils::save_file(&filepath)?;
    utils::open_file(&config.core.editor, &filepath)
}
