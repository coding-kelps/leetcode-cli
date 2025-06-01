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
