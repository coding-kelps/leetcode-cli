use core::fmt::Write as _;
use std::{
    fs,
    path::Path,
    process::Command,
};

use leetcode_cli::utils::build_problem_file;
use leetcoderustapi::ProgrammingLanguage;

/// Problems whose tests cannot be generated: judge apis (278, 374, 1095) and
/// random answers (384).
const WITHOUT_TESTS: [&str; 4] = ["1095", "278", "374", "384"];

/// Every fixture of `tests/data/problems` (`<id>.md` readme converted by
/// html2md and `<id>.rs` rust starter code from leetcode) is turned into a
/// solution file, and the ones with tests must type check.
#[test]
fn test_rust_fixtures_generate_compiling_tests() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/problems");
    let mut ids: Vec<String> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            (path.extension()? == "rs")
                .then(|| path.file_stem()?.to_str().map(str::to_string))
                .flatten()
        })
        .collect();
    ids.sort();
    assert!(!ids.is_empty());

    let out = std::env::temp_dir()
        .join(format!("leetcode-cli-fixtures-{}", std::process::id()));
    fs::create_dir_all(&out).unwrap();
    let mut modules = String::new();
    for id in &ids {
        let starter = fs::read_to_string(dir.join(format!("{id}.rs"))).unwrap();
        let readme = fs::read_to_string(dir.join(format!("{id}.md"))).unwrap();
        let (content, warning) =
            build_problem_file(&starter, &readme, &ProgrammingLanguage::Rust);
        assert_eq!(
            warning.is_some(),
            WITHOUT_TESTS.contains(&id.as_str()),
            "{id}: {warning:?}"
        );
        // without tests there is nothing generated to check, and judge
        // interfaces (1095 MountainArray) are not defined anyway
        if warning.is_some() {
            continue;
        }
        assert!(content.contains("fn test_case_0()"), "{id}: no test");
        fs::write(out.join(format!("p{id}.rs")), content).unwrap();
        let _ = writeln!(modules, "mod p{id};");
    }
    fs::write(out.join("main.rs"), format!("{modules}\nfn main() {{}}\n"))
        .unwrap();

    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let output = Command::new(rustc)
        .args(["--edition", "2021", "--test", "--emit=metadata", "-A"])
        .args(["warnings", "--crate-name", "fixtures", "--out-dir"])
        .arg(&out)
        .arg(out.join("main.rs"))
        .output()
        .unwrap();
    fs::remove_dir_all(&out).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
