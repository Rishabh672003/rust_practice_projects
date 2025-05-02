use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::process;

const API: &str = "https://api.groq.com/openai/v1/chat/completions";

#[derive(Serialize)]
pub struct GroqMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
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

#[derive(Deserialize, Serialize)]
struct Entry {
    prompt: String,
    response: String,
}

#[derive(Deserialize, Serialize)]
struct JsonData {
    chatlog: Vec<Entry>,
}

fn save_to_file(res: Entry, config: &Config) -> Result<()> {
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

fn get_history(config: &Config) -> Result<JsonData> {
    let file = File::open(&config.history_filepath)
        .with_context(|| "Failed to open config file".to_string())?;
    let reader = BufReader::new(&file);
    let data: JsonData =
        serde_json::from_reader(reader).with_context(|| "Couldn't Deserialize data".to_string())?;
    Ok(data)
}

pub fn show_history(config: &Config, count: usize) -> Result<()> {
    let mut data = get_history(config).with_context(|| {
        "Couldnt retrieve history from the config file for some reason".to_string()
    })?;

    if data.chatlog.is_empty() {
        eprintln!("Chat log is empty");
        return Ok(());
    }

    let output = data
        .chatlog
        .iter_mut()
        .rev()
        .take(count)
        .fold(String::new(), |mut out, b| {
            let _ = write!(
                out,
                "_Prompt_: {}\n_Response_: {}\n",
                b.prompt, b.response
            );
            out
        });
    println!("{}", output);
    Ok(())
}

pub async fn run(config: &Config<'_>, api_key: &String) -> Result<()> {
    if api_key.is_empty() {
        eprintln!("Error: Api key is not set properly");
        process::exit(1)
    }

    let client = Client::new();
    let mut content_str = String::new();

    match get_history(config) {
        Ok(history) => {
            for entry in history.chatlog.iter().rev().take(config.context) {
                content_str.push_str(&entry.prompt);
                content_str.push('\n');
                content_str.push_str(&entry.response);
                content_str.push('\n');
            }
        }
        Err(err) => {
            if config.context > 0 {
                eprintln!("Error: {err}");
                process::exit(1)
            }
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
    println!("_Response_: {}\n", llm_response);

    let entry = Entry {
        prompt: content_str,
        response: llm_response.to_string(),
    };
    if !config.dont_save {
        save_to_file(entry, config)?;
    }
    Ok(())
}
