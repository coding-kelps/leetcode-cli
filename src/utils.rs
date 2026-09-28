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

use crate::{
    readme_parser::LeetcodeReadmeParser,
    test_generator::TestGenerator,
};

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
    write_to_file(problem_dir, &format!("{pb_name}.md"), &content)
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
        "pandas" | "pythondata" => {
            Ok(leetcoderustapi::ProgrammingLanguage::Pandas)
        },
        "react" => Ok(leetcoderustapi::ProgrammingLanguage::React),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Unsupported language: {lang}"),
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
        leetcoderustapi::ProgrammingLanguage::Racket => "racket".to_string(),
        leetcoderustapi::ProgrammingLanguage::Erlang => "erlang".to_string(),
        leetcoderustapi::ProgrammingLanguage::Elixir => "elixir".to_string(),
        leetcoderustapi::ProgrammingLanguage::Dart => "dart".to_string(),
        // leetcode code snippets use "pythondata" as slug for pandas
        leetcoderustapi::ProgrammingLanguage::Pandas => {
            "pythondata".to_string()
        },
        leetcoderustapi::ProgrammingLanguage::React => "react".to_string(),
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
        _ => panic!("Unsupported language: {extension}"),
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
        leetcoderustapi::ProgrammingLanguage::Racket => "rkt".to_string(),
        leetcoderustapi::ProgrammingLanguage::Erlang => "erl".to_string(),
        leetcoderustapi::ProgrammingLanguage::Elixir => "ex".to_string(),
        leetcoderustapi::ProgrammingLanguage::Dart => "dart".to_string(),
        leetcoderustapi::ProgrammingLanguage::Pandas => "py".to_string(),
        leetcoderustapi::ProgrammingLanguage::React => "jsx".to_string(),
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

pub fn prefix_code(file_content: &str, lang: &ProgrammingLanguage) -> String {
    let prefix = match lang {
        // design problems can define their own `struct Solution` (384)
        ProgrammingLanguage::Rust
            if !file_content.contains("struct Solution") =>
        {
            "pub struct Solution;\n\n".to_string()
        },
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

/// Fills empty function bodies so the starter code compiles: rust starter
/// code comes with `{ }` bodies, which do not type check for functions
/// returning a value. `todo!()` keeps the file compiling and makes the
/// generated tests fail until the function is implemented.
pub fn inject_default_return_value(
    starter_code: &str, lang: &ProgrammingLanguage,
) -> String {
    match lang {
        ProgrammingLanguage::Rust => {
            let empty_fn = regex::Regex::new(r"(fn\s+\w+[^{};]*\{)(\s*)\}")
                .expect("valid regex");
            empty_fn
                .replace_all(starter_code, |caps: &regex::Captures| {
                    let indent =
                        caps[2].rsplit('\n').next().unwrap_or("").to_string();
                    format!("{}\n{indent}    todo!()\n{indent}}}", &caps[1])
                })
                .into_owned()
        },
        _ => starter_code.to_string(),
    }
}

const LOCAL_DEFINITIONS_START: &str =
    "// leetcode-cli: local definitions, removed before test / submit";
const LOCAL_DEFINITIONS_END: &str = "// leetcode-cli: end of local definitions";

/// Leetcode rust starter code ships the `ListNode` / `TreeNode` definitions
/// commented out, the file does not compile locally without them. Uncomments
/// them between markers, `preprocess_code` removes them again before the code
/// is sent to leetcode, which defines them itself.
pub fn uncomment_local_definitions(
    starter_code: &str, lang: &ProgrammingLanguage,
) -> String {
    if !matches!(lang, ProgrammingLanguage::Rust) {
        return starter_code.to_string();
    }
    let lines: Vec<&str> = starter_code.lines().collect();
    // `// Definition for ...` titles, or a bare commented definition (385)
    let is_definition = |line: &&str| {
        ["// Definition for", "// #[derive", "// pub struct", "// pub enum"]
            .iter()
            .any(|prefix| line.starts_with(prefix))
    };
    let Some(start) = lines.iter().position(is_definition) else {
        return starter_code.to_string();
    };
    let end = lines[start..]
        .iter()
        .position(|line| !line.starts_with("//"))
        .map_or(lines.len(), |offset| start + offset);

    let mut result: Vec<String> =
        lines[..start].iter().map(|line| line.to_string()).collect();
    result.push(LOCAL_DEFINITIONS_START.to_string());
    for line in &lines[start..end] {
        let code = line.trim_start_matches("//");
        // every block (list and tree) has its own `Definition for` title
        if code.trim_start().starts_with("Definition for") {
            continue;
        }
        result.push(code.strip_prefix(' ').unwrap_or(code).to_string());
    }
    // design problems use `Rc` without importing it, the judge does
    if result.iter().any(|line| line.contains("Rc<"))
        && !starter_code.contains("use std::rc::Rc")
    {
        result.push("use std::rc::Rc;".to_string());
        result.push("use std::cell::RefCell;".to_string());
    }
    result.push(LOCAL_DEFINITIONS_END.to_string());
    result.extend(lines[end..].iter().map(|line| line.to_string()));
    result.join("\n")
}

/// Content of the solution file of a problem: the starter code, the
/// generated tests and the language boilerplate. The error is a warning
/// telling why no tests were generated.
pub fn build_problem_file(
    starter_code: &str, md_desc: &str, lang: &ProgrammingLanguage,
) -> (String, Option<String>) {
    let starter_code = inject_default_return_value(starter_code, lang);
    let starter_code = uncomment_local_definitions(&starter_code, lang);

    // Tests are a bonus, the problem is still set up without them.
    let readme = LeetcodeReadmeParser::new(md_desc);
    let tests = readme.parse().map_err(io::Error::from).and_then(|test_data| {
        TestGenerator::new(&starter_code, test_data)
            .any_order(readme.any_order())
            .many_answers(readme.many_answers())
            .run(lang)
            .map_err(io::Error::from)
    });
    let (file_content, warning) = match tests {
        Ok(tests) => (format!("{starter_code}\n\n{tests}"), None),
        Err(e) => (
            starter_code,
            Some(format!(
                "No tests generated for {}: {e}",
                language_to_string(lang)
            )),
        ),
    };
    let file_content = prefix_code(&file_content, lang);
    (postfix_code(&file_content, lang), warning)
}

fn remove_local_definitions(content: &str) -> String {
    let mut inside = false;
    content
        .lines()
        .filter(|line| match line.trim() {
            LOCAL_DEFINITIONS_START => {
                inside = true;
                false
            },
            LOCAL_DEFINITIONS_END => {
                inside = false;
                false
            },
            _ => !inside,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn difficulty_color(difficulty: &str) -> colored::ColoredString {
    match difficulty {
        "Easy" => "Easy".green(),
        "Medium" => "Medium".yellow(),
        "Hard" => "Hard".red(),
        _ => "Unknown".normal(),
    }
}

/// Preprocesses file content before sending to LeetCode by removing local
/// compilation helpers
pub fn preprocess_code(
    content: &str, language: &ProgrammingLanguage,
) -> String {
    match language {
        ProgrammingLanguage::Rust => preprocess_rust_content(content),
        // For other languages, return as-is for now
        _ => content.to_string(),
    }
}

/// Removes pub struct Solution; from the top of the file, the local
/// ListNode / TreeNode definitions and the main function
fn preprocess_rust_content(content: &str) -> String {
    let n = delete_line_content(content, "pub struct Solution;");
    let n = remove_local_definitions(&n);
    remove_main(&n)
}
fn remove_main(content: &str) -> String {
    let mut c = vec![];

    for line in content.lines() {
        if line.contains("fn main() {") {
            break;
        }
        c.push(line);
    }
    c.join("\n")
}

fn delete_line_content(content: &str, target: &str) -> String {
    content
        .lines()
        .filter(|line| line.trim() != target)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Find the nearest Cargo project root (directory containing Cargo.toml)
/// starting from `start_dir` and walking up.
fn find_manifest_dir(start_dir: &Path) -> Option<PathBuf> {
    for dir in start_dir.ancestors() {
        let candidate = dir.join("Cargo.toml");
        if candidate.is_file() {
            return Some(dir.to_path_buf());
        }
    }
    None
}

/// Runs local compilation check before sending to LeetCode, errors out if
/// the code does not compile
pub async fn run_local_check(
    path_to_file: &str, language: &ProgrammingLanguage,
) -> io::Result<String> {
    use std::process::Command;

    match language {
        ProgrammingLanguage::Rust => {
            let file_path = Path::new(path_to_file);

            // If within a Cargo project, run `cargo check` at the project root
            if let Some(parent) = file_path.parent() {
                if let Some(manifest_dir) = find_manifest_dir(parent) {
                    let output = Command::new("cargo")
                        .args(["check", "--quiet"])
                        .current_dir(&manifest_dir)
                        .output()?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(io::Error::other(format!(
                            "❌ Local check failed:\n{stderr}"
                        )));
                    }

                    return Ok("✅ Local compilation passed!".to_string());
                }
            }

            // Fallback: compile the single file directly with rustc
            let output = Command::new("rustc")
                .args([
                    "--edition=2021",
                    "--emit=metadata",
                    "--crate-type=bin",
                    path_to_file,
                ])
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(io::Error::other(format!(
                    "❌ Compilation failed:\n{stderr}"
                )));
            }

            Ok("✅ Local compilation passed!".to_string())
        },
        _ => Ok(format!("⚠️ Local check not implemented for {language:?}",)),
    }
}
