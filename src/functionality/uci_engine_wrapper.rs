use super::*;
use std::{process::{Child, Command, Stdio}, io::{BufReader, Write, BufRead}, thread, sync::mpsc::{Receiver, Sender, channel}, path::PathBuf};

pub enum Message {
    Output(String),
    Crash(String),
}

use Message::*;

pub struct EngineWrapper {
    child: Child,
    sender: Sender<String>,
    receiver: Receiver<Message>,
    engine: Engine,
}

impl EngineWrapper {
    pub fn get_info(path: PathBuf) -> Result<Engine, String> {
        println!("Getting info from engine: {}", path.as_path().display());
        let mut engine = EngineWrapper::launch(Engine { path: path.clone(), ..Default::default() })?;
        engine.send("uci");
        let mut name = "Unnamed".to_string();
        let mut author = "Unknown".to_string();
        let mut options: Vec<UCIOption> = Vec::new();

        loop {
            match engine.receive() {
                Output(output) => {
                    let mut output = output.trim();
                    if output.starts_with("id name") {
                        name = output.split("id name ").nth(1).unwrap().trim().to_string();
                    } else if output.starts_with("id author") {
                        author = output.split("id author ").nth(1).unwrap().trim().to_string();
                    }

                    else if output.starts_with("option") {
                        options.push(UCIOption::parse(&mut output)?);
                    }

                    else if output.contains("uciok") {
                        break;
                    }
                },
                Crash(error) => return Err(error),
            }
        }

        engine.quit();

        Ok(Engine {
            id: 0,
            name: name.to_string(),
            author: author.to_string(),
            alias: name.to_string(),
            elo: 0,
            options,
            path,
            go_commands: "".to_string(),
            launch_commands: "".to_string(),
        })
    }

    pub fn launch(engine: Engine) -> Result<Self, String> {
        let mut child = match Command::new(engine.path.clone()).stdout(Stdio::piped()).stdin(Stdio::piped()).spawn() {
            Ok(child) => child,
            Err(e) => return Err(format!("Failed to launch engine: {}", e))
        };

        let id = child.id();

        let mut stdin = match child.stdin.take() {
            Some(input) => input,
            None => return Err("Failed to open stdin".to_string())
        };

        let stdout = match child.stdout.take() {
            Some(output) => output,
            None => return Err("Failed to open stdout".to_string())
        };

        /*let stderr = match child.stderr {
            Some(error) => error,
            None => return Err("Failed to open stderr".to_string())
        };*/

        let (in_tx, in_rx) = channel::<String>();
        let (out_tx, out_rx) = channel::<Message>();
        let out_tx1 = out_tx.clone();

        // Reader thread
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    out_tx.send(Message::Output(line)).expect("Failed to send stdout line through channel");
                } else {
                    out_tx.send(Message::Crash(format!("Engine with pid {id} crashed!"))).expect("Failed to send crash message through channel");
                    break;
                }
            }
        });

        // Writer thread
        thread::spawn(move || {
            loop {
                match in_rx.recv() {
                    Ok(line) => {
                        if stdin.write_all(line.as_bytes()).is_err() {
                            let _ = out_tx1.send(Crash(format!("Engine with pid {id} crashed!")));
                            break;
                        }
                    }
                    Err(_) => {
                        let _ = out_tx1.send(Crash(format!("Engine with pid {id} crashed!")));
                        break;
                    }
                }
            }
        });

        Ok(Self {
            child,
            sender: in_tx,
            receiver: out_rx,
            engine: engine,
        })
    }

    pub fn send(&self, command: &str) {
        self.sender.send(format!("{}\n", command.to_string())).unwrap();
    }

    pub fn quit(&mut self) {
        self.send("quit");
        self.child.wait().unwrap();
        if let Ok(Some(status)) = self.child.try_wait() {
            println!("Exited correctly with {status}");
        } else {
            println!("Exited incorrectly");
        }
    }

    pub fn receive(&self) -> Message {
        self.receiver.recv().unwrap()
    }
}