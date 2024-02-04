mod leetcode_problem;
pub mod template;
use std::{
    env,
    io::{Error, Write},
    rc::Rc,
};

use leetcode_problem::LeetCodeProblem;
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
    pb: LeetCodeProblem,
    api: Rc<UserApi>,
    leetcode_dir_path: &'static str,
    obsidian_vault_path: &'static str,
    folder_name_path: String,
    vscode_dir_path: String,
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
            pb: LeetCodeProblem::new(),
            api: Rc::new(UserApi::new(&token).await?),
            leetcode_dir_path: "/home/louis/Work/leetcode/",
            obsidian_vault_path: "/home/louis/Foundation/",
            folder_name_path: "".to_string(),
            vscode_dir_path: "".to_string(),
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
                self.pb.id = Some(arg.parse::<i32>().unwrap());
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
                self.pb.id = Some(arg2.parse::<i32>().unwrap());
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

    async fn get_number(&mut self) -> Result<(), Error>
    {
        let name = self.pb.name.as_ref().unwrap().as_str();
        let info = self.api.set_problem(name).await;

        if let Ok(info) = info {
            self.pb.id = Some(
                info.full_data
                    .data
                    .question
                    .questionId
                    .parse::<i32>()
                    .expect("Error parsing pb number"),
            );
            return Ok(());
        }

        Err(Error::new(std::io::ErrorKind::InvalidData, "pb not found"))
    }

    fn remove_html_sup_balise(&mut self) -> Result<(), Error>
    {
        let mut desc = self.pb.desc.as_ref().unwrap().clone();
        let pattern = Regex::new(r"<sup>(.*?)</sup>").unwrap();
        desc = pattern.replace_all(desc.as_str(), "$1").to_string();
        self.pb.desc = Some(desc);
        Ok(())
    }

    async fn get_desc(&mut self) -> Result<(), Error>
    {
        let name = self.pb.name.as_ref().unwrap().as_str();
        let info = self.api.set_problem(name).await;

        if let Ok(info) = info {
            let desc = info.description().unwrap();
            self.pb.desc = Some(parse_html(desc.content.as_str()));
            self.remove_html_sup_balise()?;
            return Ok(());
        }

        Err(Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "pb not found",
        ))
    }

    async fn get_code(&mut self) -> Result<(), Error>
    {
        let name = self.pb.name.as_ref().unwrap().as_str();
        let info = self.api.set_problem(name).await;

        if let Ok(info) = info {
            let snippet = info.full_data.data.question.codeSnippets;
            let code_snippets = &snippet.unwrap();
            for code_snippet in code_snippets {
                if code_snippet.lang == "Rust" {
                    self.pb.code = Some(code_snippet.code.clone());
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
            "pb not found",
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
        // Use regex to capture the pb name
        if let Some(captures) = pattern.captures(url_str) {
            // get name from URL
            self.pb.name =
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
            self.leetcode_dir_path,
            self.pb.id.unwrap(),
            self.pb.name.as_ref().unwrap().replace(" ", "_")
        );
        self.vscode_dir_path = format!("{}/.vscode", self.folder_name_path);
        self.src_folder_name_path = format!("{}/src", self.folder_name_path);
        self.create_directory(self.folder_name_path.clone().as_str())?;
        self.create_directory(self.vscode_dir_path.clone().as_str())?;
        self.create_directory(self.src_folder_name_path.clone().as_str())?;

        Ok(())
    }

    async fn collect_api_data(&mut self) -> Result<(), Error>
    {
        self.get_number().await?;
        self.get_desc().await?;
        self.get_code().await?;
        self.pb.atomization().expect("Could not atomize pb");
        Ok(())
    }

    fn create_base_files(&mut self) -> Result<(), Error>
    {
        self.create_file(
            format!(
                "{}/{}.md",
                self.folder_name_path,
                self.pb.name.as_ref().unwrap().replace(" ", "_")
            ),
            Some(self.pb.desc.as_ref().unwrap().clone()),
        )?;
        let code = self.pb.code.as_ref().unwrap().clone();
        let mut main_rs_content = template::main_rs_base_template();
        main_rs_content = main_rs_content.replace("{code}", &code);
        let main_function_template =
            template::main_function_template(self.pb.clone());
        main_rs_content = main_rs_content
            .replace("{main_function}", main_function_template.as_str());
        main_rs_content = main_rs_content.replace(
            "{tests}",
            template::tests_template(self.pb.clone()).as_str(),
        );
        self.create_file(
            format!("{}/main.rs", self.src_folder_name_path),
            Some(main_rs_content),
        )?;
        self.create_file(
            format!("{}/Cargo.toml", self.folder_name_path),
            Some(
                template::cargo_template(
                    format!(
                        "{}",
                        self.pb
                            .name
                            .as_ref()
                            .expect("pb name not found")
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
                self.pb.id.as_ref().expect("pb number not found"),
                self.pb.name.as_ref().expect("pb name not found")
            );
            self.create_file(
                format!(
                    "{}/Project/Leetcode/{}.md",
                    self.obsidian_vault_path, package_name
                ),
                self.pb.desc.clone(),
            )?;
        }
        if self.vscode {
            self.create_file(
                format!("{}/launch.json", self.vscode_dir_path.clone()),
                Some(
                    template::launch_json_template(
                        format!(
                            "{}. {}",
                            self.pb.id.as_ref().expect("pb number not found"),
                            self.pb.name.as_ref().expect("pb name not found")
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
        let number = self.pb.id.clone();

        if let Some(url) = url {
            if let Err(e) = self.validate_url(&url) {
                eprintln!("Error validating URL: {}", e);
                return Err(e);
            }
            if let Err(e) = self.parse_leetcode_url(&url).await {
                eprintln!("Error parsing LeetCode URL: {}", e);
                return Err(e);
            }
        } else if number.is_some() {
            println!(
                "Getting a pb from LeetCode by the number is still in \
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
