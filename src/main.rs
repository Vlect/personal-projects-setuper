use anyhow::{/*Context,*/ Result};
use clap::Parser;
use std::fs;
//use std::io;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let path = &args.path;

    // We can use ? to return an error if it occurs
    // which is the shotcut for the match statement below
    let result = fs::read_to_string(&args.path);

    // Since read_to_string returns a Resulte, and Result enum
    // we can use the 'match' to check which variant it is
    let content = match result {
        Ok(content) => content,
        // We could panic!, but this would exit our program
        // If we panic! or .unwrap() we don't need to return anything and
        // we can change the return type
        // Err(error) => { panic!("Can't deal with {}, just exit here", error) }

        // Or we could just return an Err
        Err(error) => {
            return Err(anyhow::Error::new(error)
                       .context(format!("Could not read file `{}`", &path.display())))
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())

    //let pattern = std::env::args().nth(1).expect("no pattern given");
    //let path = std::env::args().nth(2).expect("no pattern given");
    //let args = Cli {
    //    pattern,
    //    path: std::path::PathBuf::from(path),
    //};

    //let mut directory_name = String::new();

    //println!("Enter your name: ");

    //io::stdin()
    //    .read_line(&mut directory_name)
    //    .expect("Failed to read line");

    //fs::create_dir(directory_name.trim()).expect("Failed to create directory");
}
