use regex::Regex;

pub struct LeetcodeReadmeParser {
    pub raw: String,
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
            inputs:        self.extract_inputs(),
            outputs:       self.extract_outputs(),
        })
    }
    fn count_examples(&self) -> usize {
        self.raw
            .lines()
            .filter(|line| line.starts_with("**Example"))
            .count()
    }

    fn extract_inputs(&self) -> Vec<String> {
        self.extract_from_pattern(r"(?m)^\s*\*?\*?Input:\*?\*?\s*(.*)$")
    }

    fn extract_outputs(&self) -> Vec<String> {
        self.extract_from_pattern(r"(?m)^\s*\*?\*?Output:\*?\*?\s*(.*)$")
    }

    fn extract_from_pattern(&self, pattern: &str) -> Vec<String> {
        let re = Regex::new(pattern).unwrap();

        let mut result = Vec::new();
        for capture in re.captures_iter(&self.raw) {
            if let Some(matched) = capture.get(1) {
                let input = matched
                    .as_str()
                    .replace(['\n', '\t'], " ")
                    .trim()
                    .to_string();

                let trimmed = if input.contains('=') {
                    input
                        .split(',')
                        .filter_map(|part| {
                            if let Some(eq_pos) = part.find('=') {
                                Some(part[eq_pos + 1..].trim())
                            } else {
                                // Handle continuation of previous array/value
                                let trimmed_part = part.trim();
                                (!trimmed_part.is_empty())
                                    .then_some(trimmed_part)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                } else {
                    input.to_string()
                };

                result.push(trimmed);
            }
        }
        result
    }
}
