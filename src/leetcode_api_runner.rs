use std::io;

use colored::Colorize;
use leetcoderustapi::{
    ProgrammingLanguage,
    UserApi,
};
use nanohtml2text::html2text;

use crate::{
    config::RuntimeConfigSetup,
    utils::{
        self,
        ensure_directory_exists,
        get_file_name,
        write_to_file,
    },
};

pub struct LeetcodeApiRunner {
    rcs: RuntimeConfigSetup,
    api: UserApi,
}

impl LeetcodeApiRunner {
    pub async fn new(rcs: &RuntimeConfigSetup) -> Self {
        let api = UserApi::new(&rcs.config.leetcode_token).await.unwrap();
        LeetcodeApiRunner {
            rcs: rcs.clone(),
            api,
        }
    }

    pub async fn get_problem_info(&self, id: u32) -> io::Result<String> {
        let pb = self.api.set_problem_by_id(id).await.unwrap();

        let title = pb.description().unwrap().name.bold().cyan();
        let difficulty = match pb.difficulty().as_str() {
            "Easy" => "Easy".green(),
            "Medium" => "Medium".yellow(),
            "Hard" => "Hard".red(),
            _ => "Unknown".normal(),
        };
        let description = html2text(&pb.description().unwrap().content);

        Ok(format!("{} {}: {}\n{}", id, difficulty, title, description))
    }

    pub async fn start_problem(
        &self, id: u32, language: ProgrammingLanguage,
    ) -> io::Result<String> {
        let pb = self.api.set_problem_by_id(id).await.unwrap();
        let pb_desc = pb.description().unwrap();
        let pb_name = pb_desc.name.replace(" ", "_");
        let md_desc = html2md::parse_html(&pb_desc.content);

        let problem_dir =
            self.prepare_problem_directory(id, &pb_name, &language)?;

        self.write_readme(&problem_dir, id, &pb_name, &md_desc)?;
        self.generate_starter_code(&problem_dir, language, &pb)?;
        Ok(format!(
            "Problem {}: {} has been created at {}.",
            id,
            pb_name,
            problem_dir.display()
        ))
    }

    /// Prepares the problem directory.
    fn prepare_problem_directory(
        &self, id: u32, pb_name: &str, language: &ProgrammingLanguage,
    ) -> io::Result<std::path::PathBuf> {
        let leetcode_dir = self.rcs.resolve_leetcode_dir()?;
        let problem_dir = leetcode_dir.join(format!("{}_{}", id, pb_name));
        ensure_directory_exists(&problem_dir)?;

        // Initialize language-specific project structure
        self.initialize_language_project(&problem_dir, pb_name, language)?;

        Ok(problem_dir)
    }

    /// Writes the README file for the given problem.
    fn write_readme(
        &self, problem_dir: &std::path::Path, id: u32, pb_name: &str,
        md_desc: &str,
    ) -> io::Result<()> {
        let readme_content =
            format!("# Problem {}: {}\n\n{}", id, pb_name, md_desc);
        write_to_file(problem_dir, "README.md", &readme_content)?;
        Ok(())
    }

    /// Generates starter code for the specified programming language.
    fn generate_starter_code(
        &self, problem_dir: &std::path::Path, language: ProgrammingLanguage,
        pb: &leetcoderustapi::problem_actions::Problem,
    ) -> io::Result<()> {
        let file_name = get_file_name(&language);
        let str_language = utils::language_to_string(&language); // If only ProgrammingLanguage could derive PartialEq

        let starter_code = pb
            .code_snippets()
            .expect("No code snippets found.")
            .iter()
            .find(|snippet| snippet.langSlug == str_language)
            .map(|snippet| snippet.code.clone())
            .unwrap_or_else(|| {
                panic!("No starter code found for the specified language.")
            });
        write_to_file(problem_dir, &file_name, &starter_code)?;
        Ok(())
    }

    pub async fn test_response(
        &self, id: u32, path_to_file: String,
    ) -> io::Result<String> {
        let problem_info = self.api.set_problem_by_id(id).await.unwrap();
        let file_content = std::fs::read_to_string(&path_to_file)
            .expect("Unable to read the file");
        let language = utils::extension_programming_language(&path_to_file);

        let test_response = problem_info
            .send_test(language, &file_content)
            .await
            .unwrap();
        Ok(format!("Test response for problem {}: {:#?}", id, test_response))
    }

    pub async fn submit_response(
        &self, id: u32, path_to_file: String,
    ) -> io::Result<String> {
        let problem_info = self.api.set_problem_by_id(id).await.unwrap();
        let file_content = std::fs::read_to_string(&path_to_file)
            .expect("Unable to read the file");
        let language = utils::extension_programming_language(&path_to_file);

        let test_response = problem_info
            .send_subm(language, &file_content)
            .await
            .unwrap();
        Ok(format!(
            "Here's your submit response for problem {}: {:#?}",
            id, test_response
        ))
    }

    /// Initializes language-specific project structure.
    fn initialize_language_project(
        &self, problem_dir: &std::path::Path, pb_name: &str,
        language: &ProgrammingLanguage,
    ) -> io::Result<()> {
        use std::process::Command;

        let result = match language {
            ProgrammingLanguage::Rust => Command::new("cargo")
                .args(["init", "--name", pb_name, "--vcs", "none"])
                .current_dir(problem_dir)
                .output(),
            ProgrammingLanguage::JavaScript
            | ProgrammingLanguage::TypeScript => Command::new("npm")
                .args(["init", "-y"])
                .current_dir(problem_dir)
                .output(),
            ProgrammingLanguage::Go => {
                let module_name =
                    format!("leetcode-{}", pb_name.replace("_", "-"));
                Command::new("go")
                    .args(["mod", "init", &module_name])
                    .current_dir(problem_dir)
                    .output()
            },
            _ => return Ok(()),
        };

        match result {
            Ok(output) if !output.status.success() => {
                eprintln!(
                    "Warning: Failed to initialize project: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            },
            Err(e) => eprintln!(
                "Warning: Failed to run initialization command: {}",
                e
            ),
            _ => {},
        }

        Ok(())
    }
}
