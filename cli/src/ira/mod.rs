pub struct Action {
    pub name: String,
    pub os: String,
    pub requires: Vec<String>,
    pub commands: Vec<String>
}

pub struct Step {
    pub name: String,
    pub uses: String,
    pub args: Vec<String>,

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
        Ok(ParsedConfig { os_match: true, steps: vec![] })
    }
}

pub fn run(  ){

}
