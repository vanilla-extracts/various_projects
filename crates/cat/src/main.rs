use std::env;
use std::env::Args;
use std::error::Error;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let mut args: Args = env::args();
    let mut reversed = false;
    if args.len() < 2 {
        println!("Usage: cat <file>");
        exit(1);
    }
    if args.len() > 2 {
        reversed = !reversed;
    }
    let file = args.nth(1);
    match file {
        Some(name) => {
            let s = cat(name,reversed);
            if s.is_err() {
                println!("Error while reading file");
            }
        }
        None => println!("Error while reading file")
    };
}

fn cat(file: String,reversed: bool) -> Result<(), Box<dyn Error>> {
    let content = read_to_string(file)?;
    println!("{}",
        if reversed {content.lines().rev().collect::<Vec<&str>>().join("\n")} else {content}
    );
    Ok(())
}
