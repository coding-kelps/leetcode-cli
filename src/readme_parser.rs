use regex::Regex;

pub struct LeetcodeReadmeParser {
    pub raw: String,
}

enum RegexTarget {
    Input,
    Output,
}

pub struct ProblemTestData {
    pub example_count: usize,
    pub inputs:        Vec<String>,
    pub outputs:       Vec<String>,
}

impl LeetcodeReadmeParser {
    pub fn new(readme: &str) -> Self {
        LeetcodeReadmeParser {
            raw: readme.to_string(),
        }
    }
    pub fn parse(&self) -> Result<ProblemTestData, Box<dyn std::error::Error>> {
        if self.raw.is_empty() {
            return Err("Failed to parse empty readme".into());
        }
        Ok(ProblemTestData {
            example_count: self.count_examples(),
            inputs:        self.extract_inputs_outputs(RegexTarget::Input),
            outputs:       self.extract_inputs_outputs(RegexTarget::Output),
        })
    }
    fn count_examples(&self) -> usize {
        self.raw
            .lines()
            .filter(|line| line.starts_with("**Example"))
            .count()
    }
    fn extract_inputs_outputs(&self, target: RegexTarget) -> Vec<String> {
        let pattern = match target {
            RegexTarget::Input => r"(?m)^\s*\*?\*?Input:\*?\*?\s*(.*)$",
            RegexTarget::Output => r"(?m)^\s*\*?\*?Output:\*?\*?\s*(.*)$",
        };
        let re = Regex::new(pattern).unwrap();

        let mut result = Vec::new();
        for capture in re.captures_iter(&self.raw) {
            if let Some(matched) = capture.get(1) {
                let res = matched
                    .as_str()
                    .replace('\n', " ")
                    .replace('\t', " ")
                    .trim()
                    .to_string();
                result.push(res);
            }
        }
        result
    }
}
