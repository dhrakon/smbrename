use clap::Parser;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io::{self};
use regex::Regex;

const INVALID_CHARS: &[&str] = &[r"\\", r"/", ":", "|", "<", ">", "*", "?", "\""];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="./")]
    path: String,

    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long)]
    no_action: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // compile the regex
    let regex = match build_regex() {
        Ok(regex) => regex,
        Err(error) => {
            eprintln!("Error compiling regex: {}", error);
            return Err(error.into());
        }
    };

    // match traverse(&args, regex) {
    //     Ok(contents) => println!("Paths {}", contents),
    //     Err(error) => println!("Error: {}", error),
    // }

    // Ok(())
    let path = Path::new(&args.path);
    
    traverse(&args, path, &regex)?;

    Ok(())

}

fn traverse(args: &Args, path: &Path, regex: &Regex) -> io::Result<()> {

    // loop through entries in the path
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        // let path = entry.path();

        let entry_path = entry.path();

        // println!("{}", entry_path.display());

        if entry_path.is_dir() && args.recursive {
            traverse(&args, &entry_path, &regex)?;
        }

        // if the name of the entry matches one of our illegal characters
        match entry_path.file_name() {
            Some(file_name) => {
                let file_name = &file_name.to_string_lossy().to_string();

                if regex.is_match(&file_name) {
        
                    rename_file_for_smb(&args, &entry_path, &file_name, &regex)?;
        
                } else { }
            }
            None => {
                //why would there be no file name??
                println!("no file name??");
            }
        }
    }

    Ok(())
}

fn rename_file_for_smb(args: &Args, path: &Path, file_name: &str, regex: &Regex) -> io::Result<()> {
    // create a new name by replacing all illegal characters with an empty string
    let new_file_name = regex.replace_all(&file_name, "").to_string();
    let new_path: PathBuf;
    match path.parent() {
        Some(parent) => {
            new_path = parent.join(new_file_name);
            println!("{}", new_path.display());
        }
        None => {
            return Err(io::Error::new(io::ErrorKind::Other, format!("No parent path for {}", path.display())));
        }
    }

    println!("{} -> {}", path.display(), new_path.display());

    if !args.no_action {
        fs::rename(path, new_path)?;
    }

    Ok(())
}

fn build_regex() -> Result<Regex, regex::Error> {
    let regex_string = format!("[{}]", INVALID_CHARS.join(""));

    Regex::new(&regex_string)
}
