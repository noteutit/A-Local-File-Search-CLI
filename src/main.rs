use regex::Regex;
use std::env;
use std::fs;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("not enough arguments : {}", err);
        std::process::exit(1);
    });
    run(config);
}

fn run(config: Config) {
    let re = Regex::new(&config.keyword).unwrap_or_else(|err| {
        eprintln!("Keyword isn't in the files");
        std::process::exit(1);
    });
    for entry in WalkDir::new(&config.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let path = entry.path();
        let contents = fs::read_to_string(path);
        // to check if the file contents match with the keyword
        match contents {
            Ok(text) => {
                if re.is_match(&text) {
                    println!(" match is found in {}: \n{} ", path.display(), text);
                }
            }
            Err(e) => eprintln!("there was an error in {}:{}", path.display(), e),
        }
    }
}

struct Config {
    dir: String,
    keyword: String, //possible regex pattern
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("<program> ,<directory>, <keyword>");
        }
        let dir: String = args[1].clone();
        let keyword: String = args[2].clone();
        // to check if directory exists or not
        if !fs::metadata(&dir).map(|m| m.is_dir()).unwrap_or(false) {
            return Err("Directory doesn't exist");
        }

        Ok(Config { dir, keyword })
    }
}
