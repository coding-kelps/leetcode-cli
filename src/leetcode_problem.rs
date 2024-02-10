use regex::Regex;

#[derive(Debug, Clone)]
pub struct LeetCodeProblem
{
    pub id: Option<i32>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub code: Option<String>,
    pub fn_name: Option<String>,
    pub fn_nb_args: Option<usize>,
    pub raw_fn_args: Option<String>,
    pub fn_args: Option<Vec<String>>,
    pub_fn_args_type: Option<Vec<String>>,
    pub fn_args_name: Option<Vec<String>>,
    pub fn_rtype: Option<String>,
    pub total_examples: Option<u32>,
    pub expected_result: Option<Vec<String>>,
    pub init_var: Option<Vec<(String, String, String)>>,
}

impl LeetCodeProblem
{
    pub fn new() -> Self
    {
        Self {
            id: None,
            name: None,
            desc: None,
            code: None,
            fn_name: None,
            fn_nb_args: None,
            raw_fn_args: None,
            fn_args: None,
            pub_fn_args_type: None,
            fn_args_name: None,
            fn_rtype: None,
            total_examples: None,
            expected_result: None,
            init_var: None,
        }
    }

    fn get_fn_name(&mut self) -> Result<(), ()>
    {
        let code = self.code.as_ref().unwrap().clone();
        let pattern = Regex::new(r"pub fn (.*)\(").unwrap();
        let function_name = pattern.captures(code.as_str()).unwrap();
        self.fn_name =
            Some(function_name.get(1).unwrap().as_str().to_string());
        Ok(())
    }

    fn get_raw_fn_args(&mut self) -> Result<(), ()>
    {
        let code = self.code.as_ref().unwrap().clone();
        let pattern = Regex::new(r"pub fn .*?\((.*?)\)").unwrap();
        let function_args = pattern.captures(code.as_str()).unwrap();
        self.raw_fn_args =
            Some(function_args.get(1).unwrap().as_str().to_string());
        Ok(())
    }

    fn get_fn_rtype(&mut self) -> Result<(), ()>
    {
        let code = self.code.as_ref().unwrap().clone();
        let pattern = Regex::new(r"pub fn .*?\(.*?\) -> (.*?)\s*\{").unwrap();
        let fn_rtype = match pattern.captures(code.as_str()) {
            Some(fn_rtype) => fn_rtype,
            None => {
                self.fn_rtype = Some("()".to_string());
                return Ok(());
            }
        };
        self.fn_rtype = Some(fn_rtype.get(1).unwrap().as_str().to_string());
        Ok(())
    }

    fn get_fn_args_data(&mut self) -> Result<(), ()>
    {
        let raw_fn_args = self.raw_fn_args.as_ref().unwrap().clone();
        let types: Vec<String> = raw_fn_args
            .split(",")
            .map(|x| {
                x.trim().split(":").collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string()
            })
            .collect();
        let names: Vec<String> = raw_fn_args
            .split(",")
            .map(|x| {
                x.trim().split(":").collect::<Vec<&str>>()[0]
                    .trim()
                    .to_string()
            })
            .collect();
        self.pub_fn_args_type = Some(types);
        self.fn_args_name = Some(names.clone());
        self.fn_nb_args = Some(names.len());
        Ok(())
    }

    fn is_first_arg_array(
        &mut self,
        input: &mut String,
    ) -> bool
    {
        let split = input.split(",").collect::<Vec<&str>>();
        if split[0].contains("[") {
            return true;
        }
        return false;
    }

    fn get_example_value(
        &mut self,
        input: &mut String,
    ) -> String
    {
        let mut r = String::new();
        let arr = self.is_first_arg_array(input);
        if arr {
            r.push_str("vec!");
            let mut open_square_bracket = 0;
            let mut close_square_bracket = 0;
            for c in input.chars() {
                if c == '[' {
                    open_square_bracket += 1;
                } else if c == ']' {
                    close_square_bracket += 1;
                }
                if open_square_bracket != close_square_bracket {
                    r.push(c);
                } else {
                    r.push(']');
                    break;
                }
            }
            input.drain(0..r.len() - 4);
        } else {
            r = input.split(",").collect::<Vec<&str>>()[0]
                .trim()
                .to_string();
            input.drain(0..r.len());
        }
        r
    }

    fn collect_expected_result(
        &mut self,
        captures: &Vec<String>,
    ) -> Result<(), ()>
    {
        let mut expected_result: Vec<String> = Vec::new();
        let output_pattern = Regex::new(r"(?s)Output: (.*?)\n").unwrap();
        for (_, code) in captures.iter().enumerate() {
            let capture_output = output_pattern
                .captures_iter(&code)
                .map(|cap| cap[1].to_string())
                .collect::<Vec<String>>();
            let output = capture_output[0].clone();
            let mut output = output.split("= ").collect::<Vec<&str>>()[0]
                .trim()
                .to_owned();
            expected_result.push(self.get_example_value(&mut output));
        }
        self.expected_result = Some(expected_result);
        Ok(())
    }

    fn get_variables_from_desc(&mut self) -> Result<(), ()>
    {
        let desc = self.desc.as_ref().unwrap().clone();
        let pattern = Regex::new(r"(?s)```(.*?)```").unwrap();
        let captures = pattern
            .captures_iter(&desc)
            .map(|cap| cap[1].to_string())
            .collect::<Vec<String>>();
        self.total_examples = Some(captures.len() as u32);
        self.collect_expected_result(&captures)?;
        let mut init_var: Vec<(String, String, String)> = Vec::new();
        let input_pattern = Regex::new(r"(?s)Input: (.*?)\n").unwrap();
        for (_, code) in captures.iter().enumerate() {
            let capture_input = input_pattern
                .captures_iter(&code)
                .map(|cap| cap[1].to_string())
                .collect::<Vec<String>>();
            let input = capture_input[0].clone();
            let mut input = input.split("= ").collect::<Vec<&str>>()[1]
                .trim()
                .to_owned();

            for arg_index in 0..self.fn_nb_args.unwrap() {
                let arg_type =
                    self.pub_fn_args_type.as_ref().unwrap()[arg_index].clone();
                let arg_name =
                    self.fn_args_name.as_ref().unwrap()[arg_index].clone();
                let arg_val = self.get_example_value(&mut input);
                init_var.push((arg_name, arg_type, arg_val));
            }
        }
        self.init_var = Some(init_var);
        Ok(())
    }

    pub fn atomization(&mut self) -> Result<(), ()>
    {
        self.get_raw_fn_args()?;
        self.get_fn_name()?;
        self.get_fn_rtype()?;
        self.get_fn_args_data()?;
        self.get_variables_from_desc()?;
        Ok(())
    }
}
