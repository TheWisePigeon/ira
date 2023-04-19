
pub struct Action {
    pub name: String,
    pub os: String,
    pub requires: Vec<String>,
    pub commands: Vec<String>,
    pub skip_command: String
}

impl Default for Action {
    fn default() -> Self {
        Action { name: "".to_string(), os: "linux".to_string(), requires: vec![], commands: vec![], skip_command: "".to_string() }
    }
}

pub struct Step {
    pub name: String,
    pub uses: String,
    pub args: Vec<String>,

}

impl Default for Step {
    fn default() -> Self {
        Step { name: "".to_string(), uses: "".to_string(), args: vec![] }
    }
}

pub struct ParsedConfig {
    pub os_match: bool,
    pub steps: Vec<Step>
}

pub struct ParsingError {
    pub message: String
}

pub fn parse_action(){

}



pub fn parse_config_file( path: String)-> Result<ParsedConfig, ParsingError>  {

    let path_is_valid = path.ends_with(".ira.conf");
    if ! path_is_valid {
        return Err(ParsingError { message: String::from("File is not a valid ira config file") })
    }else {
        let config_content = std::fs::read_to_string(&path);
        let mut parsing_step = false;
        let mut current_step: Step = Step::default();
        let mut steps: Vec<Step> = vec![];
        if let Ok(content) = config_content {
            for line in content.lines() {
                if line.starts_with("#") { continue };
                if line == "[STEP]" {
                    if parsing_step{
                        return Err(ParsingError{ message: "Error while parsing your config file".to_string()});
                    }
                    parsing_step = true;
                    continue;
                }
                let props: Vec<&str>= line.trim().split("=").filter(|item|{
                    *item!=""
                }).collect();
                if props.len()!=2 {
                    return Err(ParsingError { message: "Error while parsing your config file".to_string() });
                }
                println!("{:?}", props);

            }
            Ok(ParsedConfig { os_match: true, steps: vec![] })
        }else{
            Err(ParsingError{ message: String::from("Error while reading config file. ")})
        }
    }
}

pub fn run(  ){

}
