use std::{
    fs,
    io,
    path::PathBuf,
};

use colored::Colorize;
use leetcoderustapi::{
    resources::Description,
    UserApi,
};
use nanohtml2text::html2text;

use crate::{
    config::Config,
    utils::ensure_directory_exists,
};

pub struct LeetcodeApiRunner
{
    config: Config,
    api:    UserApi,
}

impl LeetcodeApiRunner
{
    pub async fn new(mut config: Config) -> Self
    {
        let token = config.leetcode_token.take().unwrap();
        let api = UserApi::new(&token).await.unwrap();
        LeetcodeApiRunner {
            config,
            api,
        }
    }

    pub async fn get_problem_info(&self, id: u32) -> io::Result<String>
    {
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

    pub async fn submit_solution(&self, id: u32, path_to_file: String)
    {
        unimplemented!();
        // let subm_response = problem_info
        // .send_subm(ProgrammingLanguage::Rust, "impl Solution { fn two_sum()
        // {}}") .await
        // .unwrap();
    }

    pub async fn start_problem(&self, id: u32) -> io::Result<PathBuf>
    {
        // Ensure the main leetcode directory exists
        let leetcode_dir = self.config.resolve_leetcode_dir()?;

        let pb = self.api.set_problem_by_id(id).await.unwrap();
        let problem_name = pb.description().unwrap().name;
        let problem_name = problem_name.replace(" ", "_");
        let html_description = pb.description().unwrap().content;
        let md_description = html2md::parse_html(&html_description);

        // Ensure the problem-specific directory exists
        let problem_dir = leetcode_dir.join(format!("{}_{}", id, problem_name));
        ensure_directory_exists(&problem_dir)?;

        let readme_path = problem_dir.join("README.md");
        fs::write(
            readme_path,
            format!("# Problem {}:{}\n\n{}", id, problem_name, md_description),
        )
        .expect("Unable to write README file");
        Ok(problem_dir)
    }
}
