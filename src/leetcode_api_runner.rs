use std::io;

use colored::Colorize;
use leetcoderustapi::{
    ProgrammingLanguage,
    UserApi,
};
use nanohtml2text::html2text;

use crate::{
    config::Config,
    utils::{
        self,
        ensure_directory_exists,
        get_file_name,
        write_to_file,
    },
};

pub struct LeetcodeApiRunner {
    config: Config,
    api:    UserApi,
}

impl LeetcodeApiRunner {
    pub async fn new(mut config: Config) -> Self {
        let token = config.leetcode_token.take().unwrap();
        let api = UserApi::new(&token).await.unwrap();
        LeetcodeApiRunner {
            config,
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

    /// automatically detect if leetcode_dir exists and use that directory
    /// automatically detect the language from the file extension
    #[allow(dead_code)]
    #[allow(unused)]
    pub async fn submit_solution(&self, id: u32, path_to_file: String) {
        unimplemented!();
        // let subm_response = problem_info
        // .send_subm(ProgrammingLanguage::Rust, "impl Solution { fn two_sum()
        // {}}") .await
        // .unwrap();
    }

    pub async fn start_problem(
        &self, id: u32, language: ProgrammingLanguage,
    ) -> io::Result<String> {
        let pb = self.api.set_problem_by_id(id).await.unwrap();
        let pb_desc = pb.description().unwrap();
        let pb_name = pb_desc.name.replace(" ", "_");
        let md_desc = html2md::parse_html(&pb_desc.content);

        let problem_dir = self.prepare_problem_directory(id, &pb_name)?;

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
        &self, id: u32, pb_name: &str,
    ) -> io::Result<std::path::PathBuf> {
        let leetcode_dir = self.config.resolve_leetcode_dir()?;
        let problem_dir = leetcode_dir.join(format!("{}_{}", id, pb_name));
        ensure_directory_exists(&problem_dir)?;
        Ok(problem_dir)
    }

    /// Writes the README file for the given problem.
    fn write_readme(
        &self, problem_dir: &std::path::Path, id: u32, pb_name: &str,
        md_desc: &str,
    ) -> io::Result<()> {
        let readme_content =
            format!("# Problem {}: {}\n\n{}", id, pb_name, md_desc);
        write_to_file(problem_dir, "README.md", &readme_content);
        Ok(())
    }

    /// Generates starter code for the specified programming language.
    fn generate_starter_code(
        &self, problem_dir: &std::path::Path, language: ProgrammingLanguage,
        pb: &leetcoderustapi::problem_actions::Problem,
    ) -> io::Result<()> {
        let file_name = get_file_name(&language);
        let str_language = utils::language_to_string(&language);

        let starter_code = pb
            .code_snippets()
            .expect("No code snippets found.")
            .iter()
            .find(|snippet| snippet.langSlug == str_language)
            .map(|snippet| snippet.code.clone())
            .unwrap_or_else(|| {
                panic!("No starter code found for the specified language.")
            });
        write_to_file(problem_dir, &file_name, &starter_code);
        Ok(())
    }

    pub async fn test_response(
        &self, id: u32, path_to_file: String,
    ) -> io::Result<String> {
        let problem_info = self.api.set_problem_by_id(id).await.unwrap();
        // read the file content
        let file_content = std::fs::read_to_string(path_to_file)
            .expect("Unable to read the file");
        let language = utils::extension_programming_language(&file_content);
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
        // read the file content
        let file_content = std::fs::read_to_string(path_to_file)
            .expect("Unable to read the file");
        let language = utils::extension_programming_language(&file_content);
        let test_response = problem_info
            .send_test(language, &file_content)
            .await
            .unwrap();
        Ok(format!(
            "Here's your submit response for problem {}: {:#?}",
            id, test_response
        ))
    }
}
