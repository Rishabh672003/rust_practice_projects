use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::process;

const API: &str = "https://api.groq.com/openai/v1/chat/completions";

#[derive(Serialize, Deserialize)]
pub struct GroqMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroqRequest {
    model: String,
    messages: Vec<GroqMessage>,
}

#[derive(Deserialize)]
pub struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    message: Message,
}

#[derive(Deserialize)]
pub struct Message {
    content: String,
}

pub struct Config<'a> {
    pub model: String,
    pub arguments: &'a Vec<String>,
    pub history_filepath: String,
    pub context: usize,
    pub dont_save: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct Entry {
    prompt: String,
    response: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct JsonData {
    chatlog: Vec<Entry>,
}

fn save_to_file(res: Entry, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let history_path = Path::new(&config.history_filepath);

    if !history_path.exists() {
        let initial_data = JsonData { chatlog: vec![] };
        let file = File::create(history_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &initial_data)?;
    }

    let file = File::open(history_path)?;
    let reader = BufReader::new(&file);
    let mut data: JsonData = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(_) => JsonData { chatlog: vec![] },
    };
    data.chatlog.push(res);

    let file = File::create(history_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &data)?;

    Ok(())
}

fn get_history(config: &Config) -> JsonData {
    let file = match File::open(&config.history_filepath) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Couldnt open file");
            process::exit(1)
        }
    };
    let reader = BufReader::new(&file);
    let data: JsonData = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    };
    data
}

pub fn show_history(config: &Config, count: usize) {
    let data = get_history(config);
    for entry in data.chatlog.iter().rev().take(count) {
        println!("Prompt: {}", entry.prompt);
        println!("Response: {}\n", entry.response);
    }
}

pub async fn run(config: &Config<'_>, api_key: &String) -> Result<(), Box<dyn std::error::Error>> {
    if api_key.is_empty() {
        eprintln!("Error: Api key is not set properly");
        process::exit(1)
    }

    let client = reqwest::Client::new();
    let mut content_str = String::new();

    if config.context > 0 {
        let history = get_history(config);
        for entry in history.chatlog.iter().rev().take(config.context) {
            content_str.push_str(&entry.prompt);
            content_str.push('\n');
            content_str.push_str(&entry.response);
            content_str.push('\n');
        }
    }
    content_str.push_str(&config.arguments.join(" "));

    let req = GroqRequest {
        model: config.model.clone(),
        messages: vec![GroqMessage {
            role: "user".to_string(),
            content: content_str.clone(),
        }],
    };

    let response = client
        .post(API)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&req)
        .send()
        .await?;

    let response_text: GroqResponse = response.json().await?;
    let llm_response = &response_text.choices[0].message.content;
    println!("Response: {}", llm_response);

    let entry = Entry {
        prompt: content_str,
        response: llm_response.to_string(),
    };
    if !config.dont_save {
        save_to_file(entry, config)?;
    }
    Ok(())
}
