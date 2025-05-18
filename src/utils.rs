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

pub fn parse_programming_language(
    lang: &str,
) -> leetcoderustapi::ProgrammingLanguage {
    match lang.to_ascii_lowercase().as_str() {
        "cpp" | "c++" => leetcoderustapi::ProgrammingLanguage::CPP,
        "java" => leetcoderustapi::ProgrammingLanguage::Java,
        "python" | "py" => leetcoderustapi::ProgrammingLanguage::Python,
        "python3" | "py3" => leetcoderustapi::ProgrammingLanguage::Python3,
        "c" => leetcoderustapi::ProgrammingLanguage::C,
        "csharp" | "c#" => leetcoderustapi::ProgrammingLanguage::CSharp,
        "javascript" | "js" => leetcoderustapi::ProgrammingLanguage::JavaScript,
        "typescript" | "ts" => leetcoderustapi::ProgrammingLanguage::TypeScript,
        "ruby" => leetcoderustapi::ProgrammingLanguage::Ruby,
        "swift" => leetcoderustapi::ProgrammingLanguage::Swift,
        "go" | "golang" => leetcoderustapi::ProgrammingLanguage::Go,
        "bash" | "shell" => leetcoderustapi::ProgrammingLanguage::Bash,
        "scala" => leetcoderustapi::ProgrammingLanguage::Scala,
        "kotlin" | "kt" => leetcoderustapi::ProgrammingLanguage::Kotlin,
        "rust" | "rs" => leetcoderustapi::ProgrammingLanguage::Rust,
        "php" => leetcoderustapi::ProgrammingLanguage::PHP,
        "racket" => leetcoderustapi::ProgrammingLanguage::Racket,
        "erlang" => leetcoderustapi::ProgrammingLanguage::Erlang,
        "elixir" => leetcoderustapi::ProgrammingLanguage::Elixir,
        "dart" => leetcoderustapi::ProgrammingLanguage::Dart,
        "pandas" => leetcoderustapi::ProgrammingLanguage::Pandas,
        "react" => leetcoderustapi::ProgrammingLanguage::React,
        _ => panic!("Unsupported language: {}", lang),
    }
}
