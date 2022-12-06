
use std::env;
use std::fs;

fn get_file_name() -> &'static str {
    if env::var("DEMO").unwrap_or("0".to_string()) == "1" {
        "demo-input.txt"
    } else {
        "input.txt"
    }
}



pub fn get_input<F>(func: F) where F: Fn(String) {
    let file_name = get_file_name();
    match fs::read_to_string(file_name){
        Err(_) => println!("can't read file"),
        Ok(content) => {
            func(content)
        }
    }
}

pub fn get_lines<F>(func: F) 
where 
    F:Fn(Vec<&str>),
{
    get_input(|content| {
        let lines = content.split("\n").collect();
        func(lines)
    })
}