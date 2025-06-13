use std::{env, process};

use anyhow::Result;
use clap::CommandFactory;
use clap::{Parser, Subcommand};
use clap_complete::Shell;

/// Simple program to interact with LLM in terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the model
    #[arg(short, long, default_value_t = String::from("llama-3.3-70b-versatile"), long_help = "Possible values are:\ndistil-whisper-large-v3-en\ngemma2-9b-it\nllama-3.3-70b-versatile\nllama-3.1-8b-instant\nllama-guard-3-8b\nllama3-70b-8192\nllama3-8b-8192\nmixtral-8x7b-32768")]
    model: String,

    /// Filepath where history should be saved
    #[arg(long)]
    history_filepath: Option<String>,

    /// Groq Api key
    #[arg(short, long)]
    api_key: Option<String>,

    /// Whether to disable saving the history for current prompt
    #[arg(
        short,
        long,
        default_value_t = false,
        long_help = "Do not save the prompt and response in the history"
    )]
    dont_save_history: bool,

    /// No. of previous prompts and responses to be used as context
    #[arg(short, long, default_value_t = 0)]
    context: usize,

    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,

    /// Query for the LLM
    prompt: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show the history
    ShowHistory {
        /// Number of entries to display Newest to Oldest
        #[arg(short, long, default_value_t = 10)]
        count: usize,
    },

    /// Generate shell completion scripts
    #[clap(name = "completion")]
    Completion {
        #[clap(value_enum)]
        shell: Shell,

        #[arg(short, long, default_value_t = String::from("llm_chat"))]
        cmdname: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();

    let config = llm_chat::Config {
        model: cli.model,
        arguments: &cli.prompt,
        history_filepath: match cli.history_filepath {
            Some(value) => value,
            None => match env::var("HOME") {
                Ok(value) => value + "/.local/share/llm.json",
                Err(_) => process::exit(1),
            },
        },
        context: cli.context,
        dont_save: cli.dont_save_history,
    };

    match &cli.command {
        Some(Commands::ShowHistory { count }) => match llm_chat::show_history(&config, *count) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{} occured", err)
            }
        },
        Some(Commands::Completion { shell, cmdname }) => {
            let mut cmd = Args::command();
            clap_complete::generate(*shell, &mut cmd, cmdname, &mut std::io::stdout());
        }
        None => {
            let api_key = match cli.api_key {
                Some(value) => value,
                None => match env::var("GROQ_API_KEY") {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("Error: Please make sure your GROQ_API_KEY is in the environment or provide one with the -a flag");
                        process::exit(1)
                    }
                },
            };
            if cli.prompt.is_empty() {
                eprintln!("Error: No prompt was provided");
                process::exit(1)
            }
            llm_chat::run(&config, &api_key).await?
        }
    }

    Ok(())
}
