pub mod ira;

use std::env::args;

fn main()->() {
    let arg1 = args().nth(1);
    let arg2 = args().nth(2);
    match arg1 {
        Some(arg1)=>{
            match arg1.as_str() {
                "help" =>{
                    println!("Help for ira cli");
                },
                "run"=>{
                    match arg2 {
                        Some(conf_file_path)=>{
                            println!("{conf_file_path}");
                        },
                        None=>{
                            println!("config file path missing. Run `ira help` to learn how to use ira");
                            return;
                        }
                    }
                },
                _ =>{
                    println!("Unrecognized argument. Run `ira help` to learn how to use ira");
                }
            }
        },
        None=>{
            println!("IRA CLI is a tool that helps you replicate your dev environment accross devices easily using just a config gile");
            println!("Run `ira help` to learn more or visit ira website https://github.com/TheWisePigeon/ira");
        }
    }
}
