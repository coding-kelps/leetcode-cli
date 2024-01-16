use regex::Regex;

#[derive(Debug, Clone)]
pub struct LeetCodeProlem
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
    pub init_var: Option<Vec<(String, String, String)>>,
}

impl LeetCodeProlem
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
            init_var: None,
        }
    }

    fn get_fn_name(&mut self) -> Result<(), ()>
    {
        let code = self.code.as_ref().unwrap().clone();
        let pattern = Regex::new(r"pub fn (.*)\(").unwrap();
        let function_name = pattern.captures(code.as_str()).unwrap();
        println!("{}", function_name.get(1).unwrap().as_str());
        self.fn_name =
            Some(function_name.get(1).unwrap().as_str().to_string());
        Ok(())
    }

    fn get_raw_fn_args(&mut self) -> Result<(), ()>
    {
        let code = self.code.as_ref().unwrap().clone();
        let pattern = Regex::new(r"pub fn .*?\((.*?)\)").unwrap();
        let function_args = pattern.captures(code.as_str()).unwrap();
        println!("{}", function_args.get(1).unwrap().as_str());
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
        println!("{}", fn_rtype.get(1).unwrap().as_str());
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

    pub fn atomization(&mut self) -> Result<(), ()>
    {
        self.get_raw_fn_args()?;
        self.get_fn_name()?;
        self.get_fn_rtype()?;
        self.get_fn_args_data()?;
        // self.get_var_val_from_desc()?;
        Ok(())
    }
}
