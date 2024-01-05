use project_root::get_project_root;
use std::path::PathBuf;

pub fn get_absolute_path(relative_path: &str) -> Result<PathBuf, std::io::Error> {
    let p_root = get_project_root()?;
    let full_path = p_root.join(relative_path);

    Ok(full_path)
}

pub(super) fn file_exists(path: &PathBuf) -> bool {
    path.exists() && path.is_file()
}
