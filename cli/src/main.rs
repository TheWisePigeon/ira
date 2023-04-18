pub mod ira;

use ira::{ parse_action };

fn main() {
    let arg1 = std::env::args().nth(1);
    match arg1 {
        Some(arg1)=>{
            match arg1.as_str() {
                "help" =>{
                    println!("Help for ira cli");
                },
                _ =>{

                }
            }
        },
        None=>{
            println!("IRA CLI is a tool that helps you replicate your dev environment accross devices easily using just a config gile");
            println!("Run `ira help` to learn more or visit ira website https://github.com/TheWisePigeon/ira");
        }
    }
}
