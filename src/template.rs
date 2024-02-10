use crate::leetcode_problem::LeetCodeProblem;

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

{{tests}}

{{main_function}}

"
    );
}

fn _generate_vardecla(pb: LeetCodeProblem) -> String
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

pub fn tests_template(pb: LeetCodeProblem) -> String
{
    let mut r = format!(
        r#"
#[cfg(test)]
mod tests {{
    use super::*;
"#,
    );
    for i in 0..pb.total_examples.unwrap() {
        r += &format!(
            r#"
    #[test]
    fn test_example_{}() {{"#,
            i + 1
        );

        {
            r += &format!(
                r#"
        let {}:{} = {};"#,
                pb.init_var.as_ref().unwrap()[i as usize].0,
                pb.init_var.as_ref().unwrap()[i as usize].1,
                pb.init_var.as_ref().unwrap()[i as usize].2
            );
        }
        r += &format!(
            r#"
        let result = Solution::{}({});
        let expected: {} = {};
        assert_eq!(result, expected);
    }}
"#,
            pb.fn_name.as_ref().unwrap(),
            pb.init_var.as_ref().unwrap()[i as usize].0,
            pb.fn_rtype.as_ref().unwrap(),
            pb.expected_result.as_ref().unwrap()[i as usize]
        );
    }
    r += "}\n";
    return r;
}

pub fn main_function_template(_pb: LeetCodeProblem) -> String
{
    return format!(
        "fn main() {{}}",
    );
}
