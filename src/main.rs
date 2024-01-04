pub mod template;

use std::{
    env,
    io::{Error, Write},
    rc::Rc,
};

// use template::TEMPLATE;
use url::Url;
use regex::Regex;
use leetcoderustapi::UserApi;
use dotenv::dotenv;
use html2md::parse_html;

#[derive(Debug, Clone)]
struct Core
{
    obsidian: bool,
    vscode: bool,
    url: Option<Url>,
    problem_number: Option<i32>,
    problem_name: Option<String>,
    problem_description: Option<String>,
    problem_code: Option<String>,
    api: Rc<UserApi>,
    leetcode_folder_path: &'static str,
    _obsidian_vault_path: &'static str,
    folder_name_path: String,
    dot_vscode_folder_name_path: String,
    src_folder_name_path: String,
}

static HELP: &str = "Usage: ./leetcode_init [options] [<url>|<N>\n";

impl Core
{
    async fn new() -> Result<Self, Error>
    {
        dotenv().ok();
        let token =
            env::var("LEETCODE_TOKEN").expect("LEETCODE_TOKEN not found");
        let core = Self {
            obsidian: false,
            vscode: false,
            url: None,
            problem_number: None,
            problem_name: None,
            problem_description: None,
            problem_code: None,
            api: Rc::new(UserApi::new(&token).await?),
            leetcode_folder_path: "/home/louis/Work/leetcode/",
            _obsidian_vault_path: "/home/louis/Foundation/",
            folder_name_path: "".to_string(),
            dot_vscode_folder_name_path: "".to_string(),
            src_folder_name_path: "".to_string(),
        };
        Ok(core)
    }

    fn parse_flag(
        &mut self,
        flag: &str,
    ) -> Result<(), Error>
    {
        match flag {
            "-o" => {
                self.obsidian = true;
                Ok(())
            }
            "-v" => {
                self.vscode = true;
                Ok(())
            }
            "-ov" | "-vo" => {
                self.obsidian = true;
                self.vscode = true;
                Ok(())
            }
            "-h" => {
                println!("{}", HELP);
                Ok(())
            }
            "-" => {
                Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid flag passed: {}", flag),
                ))
            }
            _ => {
                Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid flag passed: {}", flag),
                ))
            }
        }
    }

    fn validate(
        &mut self,
        args: Vec<String>,
        arg_count: usize,
    ) -> Result<(), Error>
    {
        if arg_count == 1 {
            let arg = &args[1];
            if arg.parse::<i32>().is_ok() {
                self.problem_number = Some(arg.parse::<i32>().unwrap());
            } else if Url::parse(arg).is_ok() {
                self.url = Some(Url::parse(arg).unwrap());
            } else {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid argument passed: {}", arg),
                ));
            }
        } else if arg_count == 2 {
            let arg1 = &args[1];
            let arg2 = &args[2];
            self.parse_flag(arg1)?;
            if arg2.parse::<i32>().is_ok() {
                self.problem_number = Some(arg2.parse::<i32>().unwrap());
            } else if Url::parse(arg2).is_ok() {
                self.url = Some(Url::parse(arg2).unwrap());
            } else {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid argument passed: {}", arg2),
                ));
            }
        } else {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid number of arguments passed",
            ));
        }

        Ok(())
    }

    fn args_handler(
        &mut self,
        args: Vec<String>,
    ) -> Result<(), Error>
    {
        match args.len() {
            2 => self.validate(args, 1),
            3 => self.validate(args, 2),
            _ => {
                Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Too many arguments passed",
                ))
            }
        }
    }

    async fn get_problem_number(&mut self) -> Result<(), Error>
    {
        let problem_name = self.problem_name.as_ref().unwrap().as_str();
        let problem_info = self.api.set_problem(problem_name).await;

        if let Ok(problem_info) = problem_info {
            self.problem_number = Some(
                problem_info
                    .full_data
                    .data
                    .question
                    .questionId
                    .parse::<i32>()
                    .expect("Error parsing problem number"),
            );
            return Ok(());
        }

        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Problem not found",
        ))
    }

    fn remove_html_sup_balise(&mut self) -> Result<(), Error>
    {
        let mut problem_description =
            self.problem_description.as_ref().unwrap().clone();
        let pattern = Regex::new(r"<sup>(.*?)</sup>").unwrap();
        problem_description = pattern
            .replace_all(problem_description.as_str(), "$1")
            .to_string();
        self.problem_description = Some(problem_description);
        Ok(())
    }

    async fn get_problem_description(&mut self) -> Result<(), Error>
    {
        let problem_name = self.problem_name.as_ref().unwrap().as_str();
        let problem_info = self.api.set_problem(problem_name).await;

        if let Ok(problem_info) = problem_info {
            let problem_description = problem_info.description().unwrap();
            self.problem_description =
                Some(parse_html(problem_description.content.as_str()));
            self.remove_html_sup_balise()?;
            return Ok(());
        }

        Err(Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "Problem not found",
        ))
    }

    async fn get_problem_code(&mut self) -> Result<(), Error>
    {
        let problem_name = self.problem_name.as_ref().unwrap().as_str();
        let problem_info = self.api.set_problem(problem_name).await;

        if let Ok(problem_info) = problem_info {
            let snippet = problem_info.full_data.data.question.codeSnippets;
            let code_snippets = &snippet.unwrap();
            for code_snippet in code_snippets {
                if code_snippet.lang == "Rust" {
                    self.problem_code = Some(code_snippet.code.clone());
                    return Ok(());
                }
            }
            return Err(Error::new(
                std::io::ErrorKind::Unsupported,
                "Programming language not found.",
            ));
        }

        Err(Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "Problem not found",
        ))
    }

    async fn parse_leetcode_url(
        &mut self,
        url: &Url,
    ) -> Result<(), Error>
    {
        // Define the pattern for LeetCode URLs
        let pattern =
            Regex::new(r"https://leetcode.com/problems/([^/]+)").unwrap();

        let url_str = url.as_str();
        // Use regex to capture the problem name
        if let Some(captures) = pattern.captures(url_str) {
            // Extract name from URL
            self.problem_name =
                Some(captures.get(1).unwrap().as_str().replace("-", " "));
            return Ok(());
        }

        Err(Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "Invalid URL passed",
        ))
    }

    fn validate_url(
        &self,
        url: &Url,
    ) -> Result<(), Error>
    {
        if url.host_str().unwrap() != "leetcode.com" {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid URL passed",
            ));
        }

        Ok(())
    }

    fn directory_structure_creation(&mut self) -> Result<(), Error>
    {
        self.folder_name_path = format!(
            "{}{}_{}",
            self.leetcode_folder_path,
            self.problem_number.unwrap(),
            self.problem_name.as_ref().unwrap().replace(" ", "_")
        );
        self.dot_vscode_folder_name_path =
            format!("{}/.vscode", self.folder_name_path);
        self.src_folder_name_path = format!("{}/src", self.folder_name_path);
        self.create_directory(self.folder_name_path.clone().as_str())?;
        self.create_directory(
            self.dot_vscode_folder_name_path.clone().as_str(),
        )?;
        self.create_directory(self.src_folder_name_path.clone().as_str())?;

        Ok(())
    }

    async fn collect_api_data(&mut self) -> Result<(), Error>
    {
        self.get_problem_number().await?;
        self.get_problem_description().await?;
        self.get_problem_code().await?;
        Ok(())
    }

    fn create_base_files(&mut self) -> Result<(), Error>
    {
        self.create_file(
            format!(
                "{}/{}.md",
                self.folder_name_path,
                self.problem_name.as_ref().unwrap().replace(" ", "_")
            ),
            Some(self.problem_description.as_ref().unwrap().clone()),
        )?;
        self.create_file(
            format!("{}/main.rs", self.src_folder_name_path),
            Some(self.problem_code.as_ref().unwrap().clone()),
        )?;
        self.create_file(
            format!("{}/Cargo.toml", self.folder_name_path),
            Some(
                template::cargo_template(
                    format!(
                        "{}",
                        self.problem_name
                            .as_ref()
                            .expect("Problem name not found")
                            .replace(" ", "_")
                    )
                    .as_str(),
                )
                .to_string(),
            ),
        )?;
        if self.obsidian {
            let package_name = format!(
                "{}. {}",
                self.problem_number
                    .as_ref()
                    .expect("Problem number not found"),
                self.problem_name.as_ref().expect("Problem name not found")
            );
            self.create_file(
                format!(
                    "{}/Project/Leetcode/{}.md",
                    self._obsidian_vault_path, package_name
                ),
                self.problem_description.clone(),
            )?;
        }
        if self.vscode {
            self.create_file(
                format!(
                    "{}/launch.json",
                    self.dot_vscode_folder_name_path.clone()
                ),
                Some(
                    template::launch_json_template(
                        format!(
                            "{}. {}",
                            self.problem_number
                                .as_ref()
                                .expect("Problem number not found"),
                            self.problem_name
                                .as_ref()
                                .expect("Problem name not found")
                        )
                        .as_str(),
                    )
                    .to_string(),
                ),
            )?;
        }
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Error>
    {
        let url = self.url.clone();
        let problem_number = self.problem_number.clone();

        if let Some(url) = url {
            if let Err(e) = self.validate_url(&url) {
                eprintln!("Error validating URL: {}", e);
                return Err(e);
            }
            if let Err(e) = self.parse_leetcode_url(&url).await {
                eprintln!("Error parsing LeetCode URL: {}", e);
                return Err(e);
            }
        } else if problem_number.is_some() {
            println!(
                "Getting a problem from LeetCode by the number is still in \
                 development. Please use the URL instead."
            );
        } else {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Arguments passed to leetcode_init.",
            ));
        }
        let _ = self.collect_api_data().await?;
        let _ = self.directory_structure_creation();
        let _ = self.create_base_files();
        Ok(())
    }

    fn create_file(
        &mut self,
        path: String,
        content: Option<String>,
    ) -> Result<(), Error>
    {
        let mut command = std::process::Command::new("touch");
        command.arg(&path);
        let output = command.output()?;
        if !output.status.success() {
            return Err(Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Error creating file",
            ));
        }
        println!("File {} created successfully.", &path);

        if let Some(content) = content {
            let mut file =
                std::fs::OpenOptions::new().write(true).open(&path)?;
            file.write_all(content.as_bytes())?;
        }
        println!("File {} written successfully.", &path);
        Ok(())
    }

    fn create_directory(
        &mut self,
        path: &str,
    ) -> Result<(), Error>
    {
        let mut command = std::process::Command::new("mkdir");
        command.arg(path);
        let output = command.output()?;
        if !output.status.success() {
            return Err(Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Error creating directory",
            ));
        }
        println!("Directory {} created successfully.", path);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error>
{
    let args: Vec<String> = env::args().collect();
    let mut core = Core::new().await?;

    match core.args_handler(args) {
        Ok(_) => core.run().await,
        Err(e) => Err(e),
    }
}
