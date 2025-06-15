use std::io;

use colored::Colorize;
use leetcoderustapi::{
    problem_actions::Problem,
    ProgrammingLanguage,
    UserApi,
};
use nanohtml2text::html2text;

use crate::{
    config::RuntimeConfigSetup,
    readme_parser::LeetcodeReadmeParser,
    test_generator::TestGenerator,
    utils::*,
};

pub struct LeetcodeApiRunner {
    rcs: RuntimeConfigSetup,
    api: UserApi,
}

impl LeetcodeApiRunner {
    pub async fn new(rcs: &RuntimeConfigSetup) -> Result<Self, io::Error> {
        Ok(LeetcodeApiRunner {
            rcs: rcs.clone(),
            api: UserApi::new(&rcs.config.leetcode_token).await.map_err(
                |_| {
                    io::Error::new(
                        io::ErrorKind::NotConnected,
                        format!(
                            "An error occurred while creating the API client. \
                             Check your token in your configuration file: {}",
                            rcs.config_file.display()
                        ),
                    )
                },
            )?,
        })
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

    /// Fetches the problem name by its ID.
    pub async fn get_problem_name(&self, id: u32) -> io::Result<String> {
        let pb = self.api.set_problem_by_id(id).await.unwrap();
        Ok(pb.description().unwrap().name.clone())
    }

    /// Fetches the available languages for a given problem ID.
    pub async fn get_available_languages(
        &self, id: &u32,
    ) -> io::Result<Vec<String>> {
        let problem = self.api.set_problem_by_id(*id).await?;

        Ok(problem
            .code_snippets()
            .expect("No code snippets found.")
            .iter()
            .map(|snippet| snippet.langSlug.clone())
            .collect::<Vec<_>>())
    }

    pub async fn start_problem(
        &self, id: u32, language: ProgrammingLanguage,
    ) -> io::Result<String> {
        let pb = self.api.set_problem_by_id(id).await?;
        let pb_desc = pb.description().unwrap();
        let pb_name = pb_desc.name.replace(" ", "_");
        let md_desc = html2md::parse_html(&pb_desc.content);
        let problem_dir =
            self.prepare_problem_directory(id, &pb_name, &language)?;

        let starter_code = self.get_starter_code(&language, &pb)?;

        let test_data =
            LeetcodeReadmeParser::new(&md_desc).parse().map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, e.to_string())
            })?;
        let tests = TestGenerator::new(&starter_code, test_data)
            .run(&language)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let file_content = format!("{}\n\n{}", starter_code, tests);
        write_readme(&problem_dir, id, &pb_name, &md_desc)?;
        let src_dir = problem_dir.join("src");
        ensure_directory_exists(&src_dir)?;
        write_to_file(&src_dir, &get_file_name(&language), &file_content)?;

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

    /// Generates starter code for the specified programming language.
    fn get_starter_code(
        &self, language: &ProgrammingLanguage, pb: &Problem,
    ) -> io::Result<String> {
        let str_language = language_to_string(&language);

        let code_snippets = pb.code_snippets().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "No code snippets found")
        })?;

        let starter_code = code_snippets
            .iter()
            .find(|snippet| snippet.langSlug == str_language)
            .map(|snippet| snippet.code.clone())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "No starter code found for language: {}",
                        str_language
                    ),
                )
            })?;

        Ok(starter_code)
    }

    pub async fn test_response(
        &self, id: u32, path_to_file: String,
    ) -> io::Result<String> {
        let problem_info = self.api.set_problem_by_id(id).await.unwrap();
        let file_content = std::fs::read_to_string(&path_to_file)
            .expect("Unable to read the file");
        let language = get_language_from_extension(&path_to_file);

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
        let language = get_language_from_extension(&path_to_file);

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
                .args(["init", "--name", pb_name, "--vcs", "none", "--bin"])
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
