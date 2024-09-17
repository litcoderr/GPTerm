use std::env;

const ENVVAR: &str = "OPENAI_API_KEY";

fn print_usage() {
    const USAGE: &str = "[USAGE]\n\
DISCLAIMER: Set `OPENAI_API_KEY` environment variable before using gpterm!!\n\
gpterm [question]\t\t: constructs a linux command based on question";
    println!("{}", USAGE);
}

fn get_api_key() -> Option<String> {
    match env::var(ENVVAR) {
        Ok(value) => Some(value),
        Err(env::VarError::NotPresent) => {
            println!("Environment variable {} is not set.", ENVVAR);
            None
        }
        Err(env::VarError::NotUnicode(_)) => {
            println!(
                "Environment variable {} contains invalid Unicode characters.",
                ENVVAR
            );
            None
        }
    }
}

fn main() {
    // read arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    } else {
        match get_api_key() {
            Some(key) => {
                let query = args[1..].join(" ");
                // TODO answer to query using api key
            }
            None => {
                print_usage();
                return
            }
        }
    }
}
