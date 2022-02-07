use anyhow::Result;
use std::sync::Arc;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tokio::sync::Notify;

use crate::commhandler::CommHandler;

/// The User Interface
pub struct UI {
    quit_notify: Arc<Notify>,
    commhandler: Arc<CommHandler>,
}

impl UI {
    /// Create a new User Interface
    pub async fn new(quit_notify: Arc<Notify>, commhandler: Arc<CommHandler>) -> Result<Self> {
        Ok(UI {
            quit_notify,
            commhandler,
        })
    }

    /// Handles input
    ///
    /// Takes a given line of input, handles and slash commands or
    /// sends the line as a message to the user
    async fn handle_input(&self, input: &str) -> Result<bool> {
        let input = input.replace('\n', "");
        let tokens: Vec<&str> = input.split(' ').collect();
        match tokens[0] {
            "/q" | "/quit" => {
                println!("Quitting!");
                self.quit_notify.notify_one();
                return Ok(true);
            }
            "/c" | "/conn" => {
                if tokens.len() < 2 {
                    println!("Usage: /conn [ip]:[port]")
                } else {
                    println!("Try connecting to {}", tokens[1]);
                    self.commhandler.connect(tokens[1]).await?;
                }
            }
            _ => self.commhandler.send_message(&input).await?,
        }

        Ok(false)
    }

    /// Run the user interface until user quits
    pub async fn run(&self) -> Result<()> {
        let stdin_reader = BufReader::new(stdin());
        let mut input_lines = stdin_reader.lines();
        let mut quit = false;
        while !quit {
            match input_lines.next_line().await {
                Ok(val) => {
                    quit = match self.handle_input(&val.unwrap_or_default()).await {
                        Ok(res) => res,
                        Err(msg) => {
                            eprintln!("Input Error: {}", msg);
                            false
                        }
                    };
                }
                Err(err) => {
                    eprintln!("ERR: {}", err);
                }
            }
        }

        Ok(())
    }
}
