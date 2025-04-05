#![allow(unused)]
use colored::Colorize;
use leetcoderustapi::{
    resources::Description,
    UserApi,
};
use nanohtml2text::html2text;

use crate::config::Config;

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

    pub async fn get_problem_info(&self, id: u32)
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

        println!("{} {}: {}", id, difficulty, title);
        println!("{}", description);
    }

    pub async fn submit_solution(&self, id: u32, path_to_file: String)
    {
        unimplemented!();
        // let subm_response = problem_info
        // .send_subm(ProgrammingLanguage::Rust, "impl Solution { fn two_sum()
        // {}}") .await
        // .unwrap();
    }
}
