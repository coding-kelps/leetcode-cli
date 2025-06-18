use leetcode_cli::readme_parser::{
    LeetcodeReadmeParser,
    LeetcodeReadmeParserError,
};

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

    match lrp.parse() {
        Ok(_) => {
            panic!("Unexpected Ok result when testing empty readme parsing")
        },
        Err(e) => {
            assert!(
                matches!(e, LeetcodeReadmeParserError::EmptyReadme),
                "Unexpected error when testing empty readme parsing"
            );
        },
    }
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
    let expected = "[4,3,2,1]";

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
    let expected = "\"abc\",\"ahbgdc\"";

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

#[test]
fn test_823_count_example() {
    let readme_content = std::fs::read_to_string("tests/data/823.md")
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
fn test_823_inputs_0_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/823.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);
    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    let input = &problem_data.inputs[0];
    let expected = "[2,4]";
    assert_eq!(input, expected, "first input mismatch");
}

#[test]
fn test_823_outputs_1_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/823.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);
    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    let output = &problem_data.outputs[1];
    let expected = "7";
    assert_eq!(output, expected, "second output mismatch");
}

#[test]
fn test_1768_count_example() {
    let readme_content = std::fs::read_to_string("tests/data/1768.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);

    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    assert_eq!(
        problem_data.example_count, 3,
        "Expected 3 test cases in the readme"
    );
}

#[test]
fn test_1768_inputs_0_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/1768.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);
    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    let input = &problem_data.inputs[0];
    let expected = "\"abc\",\"pqr\"";
    assert_eq!(input, expected, "first input mismatch");
}

#[test]
fn test_1768_outputs_1_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/1768.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);
    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    let output = &problem_data.outputs[1];
    let expected = "\"apbqrs\"";
    assert_eq!(output, expected, "second output mismatch");
}

#[test]
fn test_1768_outputs_2_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/1768.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);
    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    let output = &problem_data.outputs[2];
    let expected = "\"apbqcd\"";
    assert_eq!(output, expected, "3rd output mismatch");
}

#[test]
fn test_1004_count_example() {
    let readme_content = std::fs::read_to_string("tests/data/1004.md")
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
fn test_1004_inputs_0_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/1004.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);
    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    let input = &problem_data.inputs[0];
    let expected = "[1,1,1,0,0,0,1,1,1,1,0],2";
    assert_eq!(input, expected, "first input mismatch");
}

#[test]
fn test_1004_inputs_1_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/1004.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);
    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    let input = &problem_data.inputs[1];
    let expected = "[0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1],3";
    assert_eq!(input, expected, "first input mismatch");
}

#[test]
fn test_1004_outputs_0_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/1004.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);
    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    let output = &problem_data.outputs[0];
    let expected = "6";
    assert_eq!(output, expected, "second output mismatch");
}

#[test]
fn test_1004_outputs_1_parse_readme() {
    let readme_content = std::fs::read_to_string("tests/data/1004.md")
        .expect("Failed to read test readme file");
    let lrp = LeetcodeReadmeParser::new(&readme_content);
    let result = lrp.parse();
    assert!(result.is_ok(), "Failed to parse readme: {:?}", result.err());
    let problem_data = result.unwrap();
    let output = &problem_data.outputs[1];
    let expected = "10";
    assert_eq!(output, expected, "second output mismatch");
}
