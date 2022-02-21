const RETRIES_DEFAULT_VALUE: i32 = 4;

pub struct Config {
    pub debug: bool,
    pub retries: i32,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let debug = args.contains(&String::from("--debug"));
        let retries = get_retries(args);
        Config { debug, retries }
    }
}

fn get_retries(args: Vec<String>) -> i32 {
    let range = args.iter().filter(|&v| v.starts_with("--retries="));
    match range.last() {
        Some(value) => {
            let split: Vec<&str> = value.split(&"--retries=".to_string()).collect();
            split[1].parse::<i32>().unwrap_or(RETRIES_DEFAULT_VALUE)
        }
        _ => RETRIES_DEFAULT_VALUE,
    }
}
