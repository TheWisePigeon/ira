struct Action {
    name: String,
    os: String,
    requires: Vec<String>,
    commands: Vec<String>
}

struct Step {
    name: String,
    uses: String,
    args: Vec<String>,

}

struct ParsedConfig {
    os_match: bool,
    steps: Vec<Step>
}

pub fn parse_action(){

}

pub fn parse_config_file( path: String){

}

pub fn run(  ){

}
