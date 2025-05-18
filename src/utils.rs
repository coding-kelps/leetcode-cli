use std::{
    io,
    path::{
        Path,
        PathBuf,
    },
};

/// Ensures that a directory exists, creating it if necessary.
pub fn ensure_directory_exists(path: &Path) -> io::Result<PathBuf>
{
    // check if the path doesn't have ~, if it does, replace it with the home
    // dir
    let home_dir =
        dirs::home_dir().expect("Unable to determine home directory");
    let path = path
        .to_str()
        .unwrap()
        .replace("~", home_dir.to_str().unwrap());
    let path = Path::new(&path);
    if !path.exists() {
        std::fs::create_dir_all(path).expect("Unable to create directory");
    }
    Ok(path.to_path_buf())
}
