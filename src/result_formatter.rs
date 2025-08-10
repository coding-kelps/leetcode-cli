use colored::Colorize;
use leetcoderustapi::resources::test_send::TestExecutionResult;

pub fn format_test_result(
    id: u32, name: &str, result: &TestExecutionResult,
) -> String {
    let mut output = String::new();

    output.push_str(&format!("🧪 Test Results for Problem {id}: {name}\n"));
    output.push_str(&"=".repeat(50));
    output.push('\n');
    output.push_str(&format_status_message(result.status_msg.as_deref()));
    output.push('\n');

    // Language
    if let Some(ref lang) = result.pretty_lang {
        output.push_str(&format!("🔧 Language: {}\n", lang.cyan()));
    }

    // // Execution success
    // if let Some(run_success) = result.run_success {
    //     let success_text = if run_success { "Yes".green() } else { "No".red()
    // };     output.push_str(&format!("> Execution Success: {}\n",
    // success_text)); }

    // Test case results
    if let (Some(correct), Some(total)) =
        (result.total_correct, result.total_testcases)
    {
        let ratio_color = if correct == total {
            |s: String| s.green()
        } else if correct > 0 {
            |s: String| s.yellow()
        } else {
            |s: String| s.red()
        };
        output.push_str(&format!(
            "📊 Test Cases: {}\n",
            ratio_color(format!("{}/{}", correct, total))
        ));
    }

    // Runtime and memory (only if successful)
    if result.run_success == Some(true) {
        if let Some(ref runtime) = result.status_runtime {
            if runtime != "N/A" {
                output.push_str(&format!("⏱️ Runtime: {}\n", runtime.blue()));
            }
        }

        if let Some(ref memory) = result.status_memory {
            if memory != "N/A" {
                output.push_str(&format!("💾 Memory: {}\n", memory.blue()));
            }
        }

        // Percentiles if available
        if let Some(Some(runtime_perc)) = result.runtime_percentile {
            output.push_str(&format!(
                "📈 Runtime Percentile: {:.1}%\n",
                runtime_perc
            ));
        }

        if let Some(Some(memory_perc)) = result.memory_percentile {
            output.push_str(&format!(
                "📈 Memory Percentile: {:.1}%\n",
                memory_perc
            ));
        }
    }

    // Compilation errors
    if let Some(ref compile_error) = result.compile_error {
        if !compile_error.is_empty() {
            output.push_str(&format!(
                "\n🔴 {}\n",
                "Compilation Error:".red().bold()
            ));
            output.push_str(&format!("{}\n", compile_error.red()));
        }
    }

    // Detailed compilation errors
    if let Some(ref full_error) = result.full_compile_error {
        if !full_error.is_empty() && result.compile_error.is_none() {
            output.push_str(&format!(
                "\n📋 {}\n",
                "Detailed Error:".red().bold()
            ));
            output.push_str(&format!("{}\n", full_error));
        }
    }

    // Wrong answer details
    if let Some(ref code_output) = result.code_output {
        if !code_output.is_empty() {
            output.push_str(&format!("\n❌ {}\n", "Your Output:".red().bold()));
            for (i, out) in code_output.iter().enumerate() {
                output.push_str(&format!("Test {}: {}\n", i + 1, out));
            }
        }
    }

    if let Some(ref expected_output) = result.expected_code_output {
        if !expected_output.is_empty() {
            output.push_str(&format!(
                "\n✅ {}\n",
                "Expected Output:".green().bold()
            ));
            for (i, out) in expected_output.iter().enumerate() {
                output.push_str(&format!("Test {}: {}\n", i + 1, out));
            }
        }
    }

    // Standard output (if any)
    if let Some(ref std_output) = result.std_output_list {
        if !std_output.is_empty()
            && std_output.iter().any(|s| !s.trim().is_empty())
        {
            output.push_str(&format!(
                "\n📤 {}\n",
                "Standard Output:".blue().bold()
            ));
            for (i, out) in std_output.iter().enumerate() {
                if !out.trim().is_empty() {
                    output.push_str(&format!("Test {}: {}\n", i + 1, out));
                }
            }
        }
    }

    output.push('\n');
    output
}

fn format_status_message(status: Option<&str>) -> String {
    match status {
        Some("Accepted") => "✅ Accepted".green().to_string(),
        Some("Wrong Answer") => "❌ Wrong Answer".red().to_string(),
        Some("Compile Error") => "🔴 Compile Error".red().to_string(),
        Some("Runtime Error") => "⚠️ Runtime Error".red().to_string(),
        Some("Time Limit Exceeded") => {
            "⏰ Time Limit Exceeded".yellow().to_string()
        },
        Some("Memory Limit Exceeded") => {
            "💾 Memory Limit Exceeded".yellow().to_string()
        },
        _ => "Unknown Status".yellow().to_string(),
    }
}
// Status with icon
