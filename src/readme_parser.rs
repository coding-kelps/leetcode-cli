pub struct LeetcodeReadmeParser {
    pub raw: String,
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
        Err("Parsing not yet implemented".into())
    }
}

pub struct ProblemTestData {
    pub id:          u32,
    pub title:       String,
    pub difficulty:  String,
    pub description: String,
}
