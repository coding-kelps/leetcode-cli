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

#[test]
fn test_strings_with_commas_and_equals_are_kept() {
    let readme = "**Example 1:**\n\n```\nInput: strs = \
                  [\"eat\",\"t,a=n\",\"\"], sep = \",\"\nOutput: \
                  [[\"eat\"],[\"t,a=n\"],[\"\"]]\n```\n";
    let data = LeetcodeReadmeParser::new(readme).parse().unwrap();
    assert_eq!(data.inputs, vec!["[\"eat\",\"t,a=n\",\"\"],\",\""]);
    assert_eq!(data.outputs, vec!["[[\"eat\"],[\"t,a=n\"],[\"\"]]"]);
}

#[test]
fn test_output_string_with_equals_is_kept() {
    let readme = "**Example 1:**\n\n```\nInput: s = \"a\", t = \"b\"\nOutput: \
                  \"a=b, c\"\n```\n";
    let data = LeetcodeReadmeParser::new(readme).parse().unwrap();
    assert_eq!(data.inputs, vec!["\"a\",\"b\""]);
    assert_eq!(data.outputs, vec!["\"a=b, c\""]);
}

#[test]
fn test_multi_line_input() {
    let readme = "**Example 1:**\n\n```\nInput: grid = [\n  [\"1\",\"0\"],\n  \
                  [\"0\",\"1\"]\n]\nOutput: 2\n\n```\n";
    let data = LeetcodeReadmeParser::new(readme).parse().unwrap();
    assert_eq!(data.inputs, vec!["[   [\"1\",\"0\"],   [\"0\",\"1\"] ]"]);
    assert_eq!(data.outputs, vec!["2"]);
}

#[test]
fn test_output_stops_at_explanation() {
    let readme = "**Example 1:**\n\n```\nInput: nums = [2,7], target = \
                  9\nOutput: [0,1]\nExplanation: Because nums[0] + nums[1] == \
                  9.\n\n```\n\n**Example 2:**\n\n**Input:** nums = \
                  [4]\n\n**Output:** [0]\n\n**Explanation:**\n";
    let data = LeetcodeReadmeParser::new(readme).parse().unwrap();
    assert_eq!(data.inputs, vec!["[2,7],9", "[4]"]);
    assert_eq!(data.outputs, vec!["[0,1]", "[0]"]);
}

#[test]
fn test_zero_width_spaces_are_removed() {
    let readme = "**Example 1:**\n\n```\nInput: s = \"ab\u{200b}\"\nOutput: \
                  \"\"\u{200b}\u{200b}\n```\n";
    let data = LeetcodeReadmeParser::new(readme).parse().unwrap();
    assert_eq!(data.inputs, vec!["\"ab\""]);
    assert_eq!(data.outputs, vec!["\"\""]);
}

#[test]
fn test_any_order_is_detected() {
    assert!(LeetcodeReadmeParser::new(
        "You can return the answer in any order."
    )
    .any_order());
    assert!(LeetcodeReadmeParser::new(
        "Notice that the order of the output and the order of the triplets \
         does not matter."
    )
    .any_order());
    assert!(!LeetcodeReadmeParser::new("Return the sorted array.").any_order());
}

#[test]
fn test_output_stops_at_misspelled_explanation() {
    let readme = "**Example 1:**\n\n```\nInput: n = 4\nOutput: \
                  \"100\"\nExplantion: (-2)2 = 4\n```\n";
    let data = LeetcodeReadmeParser::new(readme).parse().unwrap();
    assert_eq!(data.outputs, vec!["\"100\""]);
}

#[test]
fn test_design_labels_without_colon() {
    let readme = "**Example 1:**\n\n```\nInput\n[\"MinStack\",\"push\",\"top\"\
                  ]\n[[],[-2],[]]\n\nOutput\n[null,null,-2]\n\nExplanation\\
                  nMinStack minStack = new MinStack();\n```\n";
    let data = LeetcodeReadmeParser::new(readme).parse().unwrap();
    assert_eq!(data.inputs, vec![
        "[\"MinStack\",\"push\",\"top\"] [[],[-2],[]]"
    ]);
    assert_eq!(data.outputs, vec!["[null,null,-2]"]);
}

#[test]
fn test_many_answers_is_detected() {
    assert!(LeetcodeReadmeParser::new(
        "If there are multiple answers, return any of them."
    )
    .many_answers());
    assert!(!LeetcodeReadmeParser::new("Return the answer.").many_answers());
}
