use regex::Regex;
use thiserror;

pub struct LeetcodeReadmeParser {
    pub raw: String,
}

pub struct ProblemTestData {
    pub example_count: usize,
    pub inputs:        Vec<String>,
    pub outputs:       Vec<String>,
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeetcodeReadmeParserError {
    #[error("can't parse empty readme")]
    EmptyReadme,
}

impl From<LeetcodeReadmeParserError> for std::io::Error {
    fn from(e: LeetcodeReadmeParserError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    }
}

impl LeetcodeReadmeParser {
    pub fn new(readme: &str) -> Self {
        LeetcodeReadmeParser { raw: readme.to_string() }
    }

    pub fn parse(&self) -> Result<ProblemTestData, LeetcodeReadmeParserError> {
        if self.raw.is_empty() {
            return Err(LeetcodeReadmeParserError::EmptyReadme);
        }
        Ok(ProblemTestData {
            example_count: self.count_examples(),
            inputs:        self.extract_inputs(),
            outputs:       self.extract_outputs(),
        })
    }

    /// True when the statement lets the answer be returned in any order.
    pub fn any_order(&self) -> bool {
        Regex::new(r"(?i)in any order|order[^.]{0,60}does not matter")
            .expect("valid regex")
            .is_match(&self.raw)
    }

    /// True when any of several valid answers is accepted.
    pub fn many_answers(&self) -> bool {
        Regex::new(
            r"(?i)return any of them|any valid answer|multiple (valid )?(answers|solutions)|return any (one|such)",
        )
        .expect("valid regex")
        .is_match(&self.raw)
    }

    fn count_examples(&self) -> usize {
        self.raw.lines().filter(|line| line.starts_with("**Example")).count()
    }

    fn extract_inputs(&self) -> Vec<String> {
        // multi-line inputs (eg matrices) go on until the Output line, design
        // problems write `Input` / `Output` alone on their line
        self.extract_from_pattern(
            r"(?ms)^\s*\*?\*?Input(?:\*?\*?:\*?\*?|\*?\*?[ \t]*$)\s*(.*?)\s*^\s*\*?\*?Output(?:\*?\*?:|\*?\*?[ \t]*$)",
        )
    }

    fn extract_outputs(&self) -> Vec<String> {
        // an output ends with a blank line, a `Explanation:` like line (typos
        // included) or the code block
        self.extract_from_pattern(
            r"(?ms)^\s*\*?\*?Output(?:\*?\*?:\*?\*?|\*?\*?[ \t]*$)\s*(.*?)\s*(?:\n\s*\n|^\s*\*?\*?[A-Z][A-Za-z ]{0,30}:|^\s*\*?\*?Expla|^\s*```|\z)",
        )
    }

    /// `nums = [1,2], s = "a,b=c"` -> `[1,2],"a,b=c"`: removes the
    /// parameter names, splitting on top level commas only so commas and `=`
    /// inside strings or arrays are kept. Values without names are returned
    /// as is.
    fn strip_parameter_names(input: &str) -> String {
        let named = Regex::new(r"^\s*[A-Za-z_]\w*\s*=").unwrap();
        if !named.is_match(input) {
            return input.to_string();
        }

        let mut values: Vec<String> = Vec::new();
        for part in Self::split_top_level(input) {
            match part.split_once('=') {
                Some((_, value)) if named.is_match(&part) => {
                    values.push(value.trim().to_string())
                },
                // not a `name = value` pair, continuation of the previous
                // value
                _ => match values.last_mut() {
                    Some(last) => {
                        last.push(',');
                        last.push_str(part.trim());
                    },
                    None => values.push(part.trim().to_string()),
                },
            }
        }
        values.join(",")
    }

    fn split_top_level(input: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut in_quotes = false;
        let mut escaped = false;

        for ch in input.chars() {
            match ch {
                _ if escaped => escaped = false,
                '\\' if in_quotes => escaped = true,
                '"' => in_quotes = !in_quotes,
                '[' | '{' | '(' if !in_quotes => depth += 1,
                ']' | '}' | ')' if !in_quotes => depth -= 1,
                ',' if !in_quotes && depth == 0 => {
                    parts.push(std::mem::take(&mut current));
                    continue;
                },
                _ => {},
            }
            current.push(ch);
        }
        parts.push(current);
        parts
    }

    fn extract_from_pattern(&self, pattern: &str) -> Vec<String> {
        let re = Regex::new(pattern).unwrap();

        let mut result = Vec::new();
        for capture in re.captures_iter(&self.raw) {
            if let Some(matched) = capture.get(1) {
                let input = matched
                    .as_str()
                    .replace(['\n', '\t', '\u{a0}'], " ")
                    // leetcode html sometimes contains zero width spaces
                    .replace(['\u{200b}', '\u{feff}'], "")
                    .trim()
                    .to_string();

                let trimmed = Self::strip_parameter_names(&input);

                result.push(trimmed);
            }
        }
        result
    }
}
