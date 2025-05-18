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

pub fn get_file_name(lang: &leetcoderustapi::ProgrammingLanguage) -> String {
    match lang {
        leetcoderustapi::ProgrammingLanguage::CPP => "main.cpp".to_string(),
        leetcoderustapi::ProgrammingLanguage::Java => "Main.java".to_string(),
        leetcoderustapi::ProgrammingLanguage::Python => "main.py".to_string(),
        leetcoderustapi::ProgrammingLanguage::Python3 => "main.py".to_string(),
        leetcoderustapi::ProgrammingLanguage::C => "main.c".to_string(),
        leetcoderustapi::ProgrammingLanguage::CSharp => "Main.cs".to_string(),
        leetcoderustapi::ProgrammingLanguage::JavaScript => {
            "main.js".to_string()
        },
        leetcoderustapi::ProgrammingLanguage::TypeScript => {
            "main.ts".to_string()
        },
        leetcoderustapi::ProgrammingLanguage::Ruby => "main.rb".to_string(),
        leetcoderustapi::ProgrammingLanguage::Swift => "main.swift".to_string(),
        leetcoderustapi::ProgrammingLanguage::Go => "main.go".to_string(),
        leetcoderustapi::ProgrammingLanguage::Bash => "main.sh".to_string(),
        leetcoderustapi::ProgrammingLanguage::Scala => "main.scala".to_string(),
        leetcoderustapi::ProgrammingLanguage::Kotlin => "main.kt".to_string(),
        leetcoderustapi::ProgrammingLanguage::Rust => "main.rs".to_string(),
        leetcoderustapi::ProgrammingLanguage::PHP => "main.php".to_string(),
        leetcoderustapi::ProgrammingLanguage::Racket => "main.rkt".to_string(),
        leetcoderustapi::ProgrammingLanguage::Erlang => "main.erl".to_string(),
        leetcoderustapi::ProgrammingLanguage::Elixir => "main.ex".to_string(),
        leetcoderustapi::ProgrammingLanguage::Dart => "main.dart".to_string(),
        leetcoderustapi::ProgrammingLanguage::Pandas => "main.py".to_string(),
        leetcoderustapi::ProgrammingLanguage::React => "main.jsx".to_string(),
        _ => panic!("Unsupported language"),
    }
}

pub fn language_to_string(
    lang: &leetcoderustapi::ProgrammingLanguage,
) -> String {
    match lang {
        leetcoderustapi::ProgrammingLanguage::CPP => "cpp".to_string(),
        leetcoderustapi::ProgrammingLanguage::Java => "java".to_string(),
        leetcoderustapi::ProgrammingLanguage::Python => "python".to_string(),
        leetcoderustapi::ProgrammingLanguage::Python3 => "python3".to_string(),
        leetcoderustapi::ProgrammingLanguage::C => "c".to_string(),
        leetcoderustapi::ProgrammingLanguage::CSharp => "csharp".to_string(),
        leetcoderustapi::ProgrammingLanguage::JavaScript => {
            "javascript".to_string()
        },
        leetcoderustapi::ProgrammingLanguage::TypeScript => {
            "typescript".to_string()
        },
        leetcoderustapi::ProgrammingLanguage::Ruby => "ruby".to_string(),
        leetcoderustapi::ProgrammingLanguage::Swift => "swift".to_string(),
        leetcoderustapi::ProgrammingLanguage::Go => "go".to_string(),
        leetcoderustapi::ProgrammingLanguage::Bash => "bash".to_string(),
        leetcoderustapi::ProgrammingLanguage::Scala => "scala".to_string(),
        leetcoderustapi::ProgrammingLanguage::Kotlin => "kotlin".to_string(),
        leetcoderustapi::ProgrammingLanguage::Rust => "rust".to_string(),
        leetcoderustapi::ProgrammingLanguage::PHP => "php".to_string(),
        _ => panic!("Unsupported language"),
    }
}

pub fn extension_programming_language(
    file_content: &str,
) -> leetcoderustapi::ProgrammingLanguage {
    let extension = file_content
        .lines()
        .next()
        .unwrap_or("")
        .trim_start_matches("//")
        .trim();
    match extension {
        "cpp" => leetcoderustapi::ProgrammingLanguage::CPP,
        "java" => leetcoderustapi::ProgrammingLanguage::Java,
        "py" => leetcoderustapi::ProgrammingLanguage::Python3,
        "python3" | "py3" => leetcoderustapi::ProgrammingLanguage::Python3,
        "c" => leetcoderustapi::ProgrammingLanguage::C,
        "cs" => leetcoderustapi::ProgrammingLanguage::CSharp,
        "js" => leetcoderustapi::ProgrammingLanguage::JavaScript,
        "ts" => leetcoderustapi::ProgrammingLanguage::TypeScript,
        "rb" => leetcoderustapi::ProgrammingLanguage::Ruby,
        "swift" => leetcoderustapi::ProgrammingLanguage::Swift,
        "go" => leetcoderustapi::ProgrammingLanguage::Go,
        "sh" => leetcoderustapi::ProgrammingLanguage::Bash,
        "scala" => leetcoderustapi::ProgrammingLanguage::Scala,
        "kt" => leetcoderustapi::ProgrammingLanguage::Kotlin,
        "rs" => leetcoderustapi::ProgrammingLanguage::Rust,
        "php" => leetcoderustapi::ProgrammingLanguage::PHP,
        "rkt" => leetcoderustapi::ProgrammingLanguage::Racket,
        "erl" => leetcoderustapi::ProgrammingLanguage::Erlang,
        "ex" | "exs" => leetcoderustapi::ProgrammingLanguage::Elixir,
        "dart" => leetcoderustapi::ProgrammingLanguage::Dart,
        "jsx" => leetcoderustapi::ProgrammingLanguage::React,
        _ => panic!("Unsupported language: {}", extension),
    }
}
