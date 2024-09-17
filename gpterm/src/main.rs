use reqwest::Client;
use serde_json::json;
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

async fn answer_query(api_key: &str, query: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let messages = json!([
        {
            "role": "system",
            "content": "You are a helpful linux command assistant. Answer in following format. [1] [linux command] : [very concise explanation] next line [2] ... Reply without additional chat. Only the format."
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
                // TODO add functionality of selecting option and pasting it to the command prompt to run the command
                Ok(())
            }
            None => {
                print_usage();
                Ok(())
            }
        }
    }
}
