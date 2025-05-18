use std::{
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
};

/// Ensures that a directory exists, creating it if necessary.
pub fn ensure_directory_exists(path: &Path) -> io::Result<PathBuf> {
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

/// Writes content to a file in the specified directory.
pub fn write_to_file(dir: &Path, file_name: &str, content: &str) {
    let file_path = dir.join(file_name);
    fs::write(file_path, content)
        .expect(&format!("Unable to write to file: {}", file_name));
}
