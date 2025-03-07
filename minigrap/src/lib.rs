use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;


    

    let result = if config.case_insensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };


    for line in result {
        println!("{}", line)
    }


    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query not set")
        };


        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("filename not set")
        };


        // if args.len() < 3 {
        //     return Err("no args found");
        // }
        // let query = &args[1];
        // let filename = &args[2];
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_insensitive})
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line.trim());
    //     }
    // }
    // results
    contents.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = &query.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(query) {
    //         results.push(line.trim());
    //     }
    // }
    // results
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
    Rust
    safe,fast,production.
    pink there.
    Duct us  
    ";

        assert_eq!(vec!["safe,fast,production."], search(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "rUst";
        let contents = "\
    Rust:
    safe,fast,production.
    pink there.
    Trust us 
    ";

        assert_eq!(
            vec!["Rust:", "Trust us"],
            search_case_insensitive(query, contents)
        );
    }
}
