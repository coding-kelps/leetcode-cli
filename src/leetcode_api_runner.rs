#![allow(unused)]
use crate::config::Config;
use leetcoderustapi::UserApi;

pub struct LeetcodeApiRunner
{
    config: Config,
    api: UserApi,
}

impl LeetcodeApiRunner
{
    pub async fn new(mut config: Config) -> Self {
        let token = config.leetcode_token.take().unwrap();
        let api = UserApi::new(&token).await.unwrap();
        LeetcodeApiRunner { config, api }
    }

    pub async fn get_problem_info(&self, id: u32)
    {
        unimplemented!("asking for problem info");
        // Implement the logic to get problem info using self.api
    }

    pub async fn submit_solution(&self, id: u32, path_to_file: String)
    {
        unimplemented!();
        // Implement the logic to submit solution using self.api
    }
}