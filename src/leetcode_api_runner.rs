use std::{
    io,
    path::{
        Path,
        PathBuf,
    },
};

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
                |e| {
                    io::Error::new(
                        io::ErrorKind::NotConnected,
                        format!(
                            "An error occurred while creating the API client. \
                             Check your token in your configuration file: {}: \
                             {}",
                            rcs.config_file.display(),
                            e,
                        ),
                    )
                },
            )?,
        })
    }

    pub async fn get_problem_info(&self, id: u32) -> io::Result<String> {
        let pb = self.api.set_problem_by_id(id).await?;

        let title = pb.description()?.name.bold().cyan();
        let difficulty = difficulty_color(&pb.difficulty());
        let description = html2text(&pb.description()?.content);

        Ok(format!(
            "\n#{id}  -  {difficulty}  -  {title}\n\n{description}"
        ))
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
        &self, id: u32, lang: ProgrammingLanguage,
    ) -> io::Result<(String, PathBuf, Option<String>)> {
        let pb = self.api.set_problem_by_id(id).await?;
        let pb_desc = pb.description()?;
        let pb_name = pb_desc.name.replace(" ", "_");
        let md_desc = html2md::parse_html(&pb_desc.content);
        let (pb_dir, src_dir, warning) =
            self.prepare_problem_dir(id, &pb_name, &lang)?;

        let mut starter_code = self.get_starter_code(&lang, &pb)?;
        starter_code = inject_default_return_value(&starter_code, &lang);

        let test_data = LeetcodeReadmeParser::new(&md_desc).parse()?;
        let tests = TestGenerator::new(&starter_code, test_data).run(&lang)?;

        let mut file_content = format!("{starter_code}\n\n{tests}");
        file_content = prefix_code(&file_content, &lang);
        file_content = postfix_code(&file_content, &lang);
        write_readme(&pb_dir, id, &pb_name, &md_desc)?;
        write_to_file(&src_dir, &get_file_name(&lang), &file_content)?;

        let success_message = format!(
            "{}: {} created at \n{}\nin {}.",
            id,
            pb_name.green().bold(),
            pb_dir.display(),
            language_to_string(&lang)
        );

        Ok((success_message, pb_dir, warning))
    }

    /// Prepares the problem directory.
    fn prepare_problem_dir(
        &self, id: u32, pb_name: &str, language: &ProgrammingLanguage,
    ) -> io::Result<(PathBuf, PathBuf, Option<String>)> {
        let leetcode_dir = self.rcs.resolve_leetcode_dir()?;
        let problem_dir = leetcode_dir.join(format!("{id}_{pb_name}"));
        let src_dir = problem_dir.join("src");

        ensure_directory_exists(&problem_dir)?;
        ensure_directory_exists(&src_dir)?;

        let warning =
            self.initialize_language_project(&problem_dir, pb_name, language)?;
        Ok((problem_dir, src_dir, warning))
    }

    /// Generates starter code for the specified programming language.
    fn get_starter_code(
        &self, language: &ProgrammingLanguage, pb: &Problem,
    ) -> io::Result<String> {
        let str_language = language_to_string(language);

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
                        "No starter code found for language: {str_language}"
                    ),
                )
            })?;

        Ok(starter_code)
    }

    pub async fn test_response(
        &self, id: u32, path_to_file: &String,
    ) -> io::Result<()> {
        let problem_info = self.api.set_problem_by_id(id).await?;
        let file_content = std::fs::read_to_string(path_to_file)
            .expect("Unable to read the file");
        let language = get_language_from_extension(path_to_file);

        let test_res = problem_info.send_test(language, &file_content).await?;
        println!("Test response for problem {id}: {test_res:?}");
        Ok(())
    }

    pub async fn submit_response(
        &self, id: u32, path_to_file: &String,
    ) -> io::Result<()> {
        let pb = self.api.set_problem_by_id(id).await?;
        let file_content = std::fs::read_to_string(path_to_file)
            .expect("Unable to read the file");
        let language = get_language_from_extension(path_to_file);

        let sub_res = pb.send_subm(language, &file_content).await?;
        println!("{id}: submit result {sub_res:?}");
        Ok(())
    }

    /// Initializes language-specific project structure.
    fn initialize_language_project(
        &self, problem_dir: &Path, pb_name: &str,
        language: &ProgrammingLanguage,
    ) -> io::Result<Option<String>> {
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
            _ => return Ok(None),
        };

        match result {
            Ok(output) if !output.status.success() => {
                Ok(Some(String::from_utf8(output.stderr).unwrap()))
            },
            Err(e) => Ok(Some(e.to_string())),
            _ => Ok(None),
        }
    }
}
