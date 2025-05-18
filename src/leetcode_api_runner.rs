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
        ensure_directory_exists,
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
        // Ensure the main leetcode directory exists
        let leetcode_dir = self.config.resolve_leetcode_dir()?;
        let pb = self.api.set_problem_by_id(id).await.unwrap();
        let pb_desc = pb.description().unwrap();
        let pb_name = pb_desc.name.replace(" ", "_");
        let md_desc = html2md::parse_html(&pb_desc.content);

        // Ensure the problem-specific directory exists
        let problem_dir = leetcode_dir.join(format!("{}_{}", id, pb_name));
        ensure_directory_exists(&problem_dir)?;

        let readme = format!("# Problem {}: {}\n\n{}", id, pb_name, md_desc);
        write_to_file(&problem_dir, "README.md", &readme);
        Ok(format!(
            "Problem {}: {} has been created at {}",
            id,
            pb_name,
            problem_dir.display()
        ))
    }
}
