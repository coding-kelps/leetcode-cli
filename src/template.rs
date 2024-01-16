use crate::leetcode_problem::LeetCodeProlem;

pub fn cargo_template(package_name: &str) -> String
{
    format!(
        "[package]\nname = \"{}\"\nauthors = [\"Loud_C\"]\nversion = \
         \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]",
        package_name
    )
}
pub fn launch_json_template(package_name: &str) -> String
{
    format!(
        r#"{{
    "version": "0.2.0",
    "configurations": [
        {{
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable '{}'",
            "cargo": {{
                "args": [
                    "build",
                    "--bin={}",
                    "--package={}"
                ],
                "filter": {{
                    "name": "{}",
                    "kind": "bin"
                }}
            }},
            "args": [""],
            "cwd": "${{workspaceFolder}}"
        }},
        {{
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable '{}'",
            "cargo": {{
                "args": [
                    "test",
                    "--no-run",
                    "--bin={}",
                    "--package={}"
                ],
                "filter": {{
                    "name": "{}",
                    "kind": "bin"
                }}
            }},
            "args": [""],
            "cwd": "${{workspaceFolder}}"
        }}
    ]
}}"#,
        package_name,
        package_name,
        package_name,
        package_name,
        package_name,
        package_name,
        package_name,
        package_name
    )
}

pub fn main_rs_base_template() -> String
{
    return format!(
        "pub struct Solution;
    
    {{code}}

{{main_function}}
"
    );
}

fn generate_vardecla(pb: LeetCodeProlem) -> String
{
    let result: String = pb
        .init_var
        .expect("init_var is None")
        .iter()
        .map(|(var_name, var_type, var_val)| {
            format!("let {}:{} = {};", var_name, var_type, var_val)
        })
        .collect::<Vec<String>>()
        .join("\n");
    return result;
}

pub fn main_function_template(pb: LeetCodeProlem) -> String
{
    let var_decla = "".to_string();
    return format!(
        "fn main() {{
    {}
    let result = Solution::{}();
    println!(\"{{:?}}\", result);
}}",
        var_decla,
        pb.fn_name.expect("fn_name is None")
    );
}
