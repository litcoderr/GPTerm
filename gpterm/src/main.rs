use regex::Regex;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::io;
use std::io::Write;
use std::process::Command;
use shlex;

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

async fn answer_query(api_key: &str, query: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let messages = json!([
        {
            "role": "system",
            "content": "You are a helpful linux command assistant. Answer in following format. [1] [linux command wrapped in `] : [very concise explanation] next line [2] ... Reply without additional chat. Only the format."
        },
        {
            "role": "user",
            "content": query
        }
    ]);

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-4o",  // Use the appropriate model
            "messages": messages,
            "max_completion_tokens": 100,
        }))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let answer = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response")
        .to_string();
    Ok(answer)
}

fn parse_result(s: &str) -> Vec<String> {
    // Create a regex pattern to match anything inside backticks
    let re = Regex::new(r"`([^`]*)`").unwrap();

    // Create a vector to store the matched strings
    let mut commands: Vec<String> = Vec::new();

    // Iterate over all matches and collect the strings into the vector
    for cap in re.captures_iter(s) {
        commands.push(cap[1].to_string());
    }

    commands
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        Ok(())
    } else {
        match get_api_key() {
            Some(key) => {
                let query = args[1..].join(" ");
                let result = answer_query(&key, query).await?;
                println!("{}", &result);
                println!("--");

                // parse result to a list of command strings
                let commands = parse_result(&result);

                print!("Enter Number (Ctrl-C to abort): ");
                io::stdout().flush().expect("Unable to flush stdout!");

                // get user's prefered command number
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                match input.trim().parse::<u32>() {
                    Ok(number) => {
                        if number > (commands.len() as u32) {
                            println!("Invalid input.");
                        } else {
                            let command = commands[(number - 1) as usize].clone();

                            // split command to command and arguments
                            let parts: Vec<String> = shlex::split(&command).expect("Failed to parse command string");
                            if let Some((program, args)) = parts.split_first() {
                                // Execute
                                Command::new(program)
                                    .args(args)
                                    .spawn()
                                    .expect("Failed to execute command");
                            } else {
                                println!("No command to execute!");
                            }
                        }
                    }
                    Err(_) => {
                        println!("Invalid input.");
                    }
                }
                Ok(())
            }
            None => {
                print_usage();
                Ok(())
            }
        }
    }
}
