use leetcode_cli::readme_parser::LeetcodeReadmeParser;

#[test]
fn new_leetcode_readme_parser() {
    let readme_content = "# a random LeetCode Problem!";
    let lrp = LeetcodeReadmeParser::new(readme_content);

    assert_eq!(lrp.raw, readme_content);
}

#[test]
fn test_parse_empty_readme() {
    let empty_readme = String::new();
    let lrp = LeetcodeReadmeParser::new(&empty_readme);

    let result = lrp.parse();
    assert!(
        result.is_err(),
        "Failed to parse empty readme: {:?}",
        result.err()
    );
}

#[test]
fn test_3467_count_example() {
    let readme_content = std::fs::read_to_string("tests/data/3467.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);

    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    assert_eq!(
        problem_data.example_count, 2,
        "Expected 2 test cases in the readme"
    );
}

#[test]
fn test_3467_inputs_0_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/3467.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);

    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();

    let input = &problem_data.inputs[0];
    let expected = "nums = [4,3,2,1]";

    assert_eq!(input, expected, "first input mismatch");
}

#[test]
fn test_3467_outputs_1_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/3467.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);

    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();

    let output = &problem_data.outputs[1];
    let expected = "[0,0,1,1,1]";

    assert_eq!(output, expected, "second output mismatch");
}

#[test]
fn test_392_count_example() {
    let readme_content = std::fs::read_to_string("tests/data/392.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);

    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    assert_eq!(
        problem_data.example_count, 2,
        "Expected 1 test case in the readme"
    );
}

#[test]
fn test_392_inputs_0_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/392.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);

    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();

    let input = &problem_data.inputs[0];
    let expected = "s = \"abc\", t = \"ahbgdc\"";

    assert_eq!(input, expected, "first input mismatch");
}

#[test]
fn test_392_outputs_1_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/392.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);

    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();

    let output = &problem_data.outputs[1];
    let expected = "false";

    assert_eq!(output, expected, "second output mismatch");
}
