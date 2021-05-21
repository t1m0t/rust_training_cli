use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].as_str();
        let filename = args[2].as_str();
        let case_sensitive = !env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run_cmd(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(config.query, &contents, Some(config.case_sensitive)) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: Option<bool>) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if case_sensitive.unwrap() {
            if line.to_uppercase().contains(&query.to_uppercase()) {
                results.push(line);
            }
        } else {
            if line.contains(query) {
                results.push(line);
            }
        }
    }

    if results.len() == 0 {
        results.push("Nothing found");
    }
    results
}
