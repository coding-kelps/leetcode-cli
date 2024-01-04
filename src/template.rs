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
