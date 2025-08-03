use std::{
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
};

use colored::Colorize;
use leetcoderustapi::ProgrammingLanguage;

/// Ensures that a directory exists, creating it if necessary.
pub fn ensure_directory_exists(path: &Path) -> io::Result<PathBuf> {
    let path = Path::new(&path);
    if !path.exists() {
        std::fs::create_dir_all(path).expect("Unable to create directory");
    }
    Ok(path.to_path_buf())
}

/// Writes content to a file in the specified directory.
pub fn write_to_file(
    dir: &Path, file_name: &str, content: &str,
) -> io::Result<()> {
    let file_path = dir.join(file_name);
    fs::write(file_path, content)
}

/// Writes the README file for the given problem.
pub(crate) fn write_readme(
    problem_dir: &Path, id: u32, pb_name: &str, md_desc: &str,
) -> io::Result<()> {
    let content = format!("# Problem {id}: {pb_name}\n\n{md_desc}");
    write_to_file(problem_dir, &format!("{}.md", pb_name), &content)
}

pub fn parse_programming_language(
    lang: &str,
) -> Result<leetcoderustapi::ProgrammingLanguage, std::io::Error> {
    match lang.to_ascii_lowercase().as_str() {
        "cpp" | "c++" => Ok(leetcoderustapi::ProgrammingLanguage::CPP),
        "java" => Ok(leetcoderustapi::ProgrammingLanguage::Java),
        "python" | "py" => Ok(leetcoderustapi::ProgrammingLanguage::Python),
        "python3" | "py3" => Ok(leetcoderustapi::ProgrammingLanguage::Python3),
        "c" => Ok(leetcoderustapi::ProgrammingLanguage::C),
        "csharp" | "c#" => Ok(leetcoderustapi::ProgrammingLanguage::CSharp),
        "javascript" | "js" => {
            Ok(leetcoderustapi::ProgrammingLanguage::JavaScript)
        },
        "typescript" | "ts" => {
            Ok(leetcoderustapi::ProgrammingLanguage::TypeScript)
        },
        "ruby" => Ok(leetcoderustapi::ProgrammingLanguage::Ruby),
        "swift" => Ok(leetcoderustapi::ProgrammingLanguage::Swift),
        "go" | "golang" => Ok(leetcoderustapi::ProgrammingLanguage::Go),
        "bash" | "shell" => Ok(leetcoderustapi::ProgrammingLanguage::Bash),
        "scala" => Ok(leetcoderustapi::ProgrammingLanguage::Scala),
        "kotlin" | "kt" => Ok(leetcoderustapi::ProgrammingLanguage::Kotlin),
        "rust" | "rs" => Ok(leetcoderustapi::ProgrammingLanguage::Rust),
        "php" => Ok(leetcoderustapi::ProgrammingLanguage::PHP),
        "racket" => Ok(leetcoderustapi::ProgrammingLanguage::Racket),
        "erlang" => Ok(leetcoderustapi::ProgrammingLanguage::Erlang),
        "elixir" => Ok(leetcoderustapi::ProgrammingLanguage::Elixir),
        "dart" => Ok(leetcoderustapi::ProgrammingLanguage::Dart),
        "pandas" => Ok(leetcoderustapi::ProgrammingLanguage::Pandas),
        "react" => Ok(leetcoderustapi::ProgrammingLanguage::React),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Unsupported language: {}", lang),
        )),
    }
}

pub fn get_file_name(lang: &leetcoderustapi::ProgrammingLanguage) -> String {
    format!("main.{}", get_extension_from_language(lang))
}

/// Converts a programming language enum to its string representation.
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

pub fn get_language_from_extension(
    file_name: &str,
) -> leetcoderustapi::ProgrammingLanguage {
    let extension = file_name.rsplit('.').next().unwrap_or("").to_lowercase();
    match extension.as_str() {
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

pub fn spin_the_spinner(message: &str) -> spinners::Spinner {
    spinners::Spinner::new(spinners::Spinners::Dots12, message.to_string())
}

pub fn stop_and_clear_spinner(mut spinner: spinners::Spinner) {
    use std::io::{
        self,
        Write,
    };

    spinner.stop();
    print!("\r\x1b[2K"); // Clear the line
    io::stdout().flush().unwrap_or(());
}

pub fn prompt_for_language(
    id: &u32, problem_name: &str, available_languages: &[String],
) -> Result<String, io::Error> {
    println!(
        "\nPlease enter a valid Leetcode programming language.\nHere is a \
         list of available languages for the problem {} - {}\n{}",
        id,
        problem_name,
        available_languages
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "No language entered"))
    } else {
        Ok(trimmed)
    }
}

pub fn get_extension_from_language(
    lang: &leetcoderustapi::ProgrammingLanguage,
) -> String {
    match lang {
        leetcoderustapi::ProgrammingLanguage::CPP => "cpp".to_string(),
        leetcoderustapi::ProgrammingLanguage::Java => "java".to_string(),
        leetcoderustapi::ProgrammingLanguage::Python => "py".to_string(),
        leetcoderustapi::ProgrammingLanguage::Python3 => "py".to_string(),
        leetcoderustapi::ProgrammingLanguage::C => "c".to_string(),
        leetcoderustapi::ProgrammingLanguage::CSharp => "cs".to_string(),
        leetcoderustapi::ProgrammingLanguage::JavaScript => "js".to_string(),
        leetcoderustapi::ProgrammingLanguage::TypeScript => "ts".to_string(),
        leetcoderustapi::ProgrammingLanguage::Ruby => "rb".to_string(),
        leetcoderustapi::ProgrammingLanguage::Swift => "swift".to_string(),
        leetcoderustapi::ProgrammingLanguage::Go => "go".to_string(),
        leetcoderustapi::ProgrammingLanguage::Bash => "sh".to_string(),
        leetcoderustapi::ProgrammingLanguage::Scala => "scala".to_string(),
        leetcoderustapi::ProgrammingLanguage::Kotlin => "kt".to_string(),
        leetcoderustapi::ProgrammingLanguage::Rust => "rs".to_string(),
        leetcoderustapi::ProgrammingLanguage::PHP => "php".to_string(),
        _ => panic!("Unsupported language: {lang:?}"),
    }
}

pub fn prefix_code(file_content: &str, lang: &ProgrammingLanguage) -> String {
    let prefix = match lang {
        ProgrammingLanguage::Rust => "pub struct Solution;\n\n".to_string(),
        _ => "".to_string(),
    };
    format!("{prefix}\n{file_content}")
}

pub fn postfix_code(file_content: &str, lang: &ProgrammingLanguage) -> String {
    let postfix = match lang {
        ProgrammingLanguage::Rust => "\n\nfn main() {}\n".to_string(),
        _ => "".to_string(),
    };
    format!("{file_content}\n{postfix}")
}

fn read_rust_ast(starter_code: &str) -> Result<String, io::Error> {
    Ok(starter_code.to_string())
}

pub fn inject_default_return_value(
    starter_code: &str, lang: &ProgrammingLanguage,
) -> String {
    match lang {
        ProgrammingLanguage::Rust => {
            let _ast = read_rust_ast(starter_code).unwrap_or_else(|_| {
                panic!("Failed to read Rust AST from starter code")
            });

            starter_code.to_string()
        },
        _ => starter_code.to_string(),
    }
}

pub fn difficulty_color(difficulty: &str) -> colored::ColoredString {
    match difficulty {
        "Easy" => "Easy".green(),
        "Medium" => "Medium".yellow(),
        "Hard" => "Hard".red(),
        _ => "Unknown".normal(),
    }
}
