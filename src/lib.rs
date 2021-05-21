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
        let case_sensitive = match env::var("CASE_INSENSITIVE") {
            Ok(val) => {
                let val:u8 = val.parse().unwrap();
                if val == 1 {true}
                else if val == 0 {false}
                else {false}
            },
            Err(_) => false,
        };

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
    contents
        .lines()
        .filter(|line| {
            if case_sensitive.unwrap() {
                line.to_uppercase().contains(&query.to_uppercase())
            } else {
                line.contains(query)
            }
        })
        .collect()
}
