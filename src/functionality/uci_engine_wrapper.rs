use super::*;
use std::{process::{Child, Command, Stdio}, io::{BufReader, Write, BufRead}, thread, sync::mpsc::{Receiver, Sender, channel}};

pub enum Message {
    Output(String),
    Crash(String),
}

use Message::*;

pub struct EngineWrapper {
    child: Child,
    sender: Sender<String>,
    receiver: Receiver<Message>,
    _engine: Engine,
}

impl EngineWrapper {
    pub fn get_info(path: String) -> Result<Engine, String> {
        println!("Getting info from engine: {}", path);
        let mut engine = EngineWrapper::launch(Engine { path: path.clone(), ..Default::default() })?;
        engine.send("uci");
        let mut name = "Unknown".to_string();
        let mut author = "Unknown".to_string();
        let mut options: Vec<UCIOption> = Vec::new();

        loop {
            match engine.receive() {
                Output(output) => {
                    if output.starts_with("id name") {
                        name = output.split("id name ").nth(1).unwrap().to_string();
                    } else if output.starts_with("id author") {
                        author = output.split("id author ").nth(1).unwrap().to_string();
                    } else if output.starts_with("option name Clear Hash type button") {
                        let mut option = UCIOption::default();
                        option.name = "Clear Hash".to_string();
                        option.opt_type = "button".to_string();
                        options.push(option);
                    } else if output.starts_with("option") {
                        let mut option = UCIOption::default();
                        let mut parts = output.split(" ");
                        option.name = parts.nth(2).unwrap().to_string();
                        option.opt_type = parts.nth(1).unwrap().to_string();
                        option.default = parts.nth(1).unwrap().to_string();
                        if option.opt_type == "spin" {
                            option.min = parts.nth(1).unwrap().to_string();
                            option.max = parts.nth(1).unwrap().to_string();
                        } else if option.opt_type == "combo" {
                            let mut vars = Vec::new();
                            for var in parts.step_by(2) {
                                vars.push(var.to_string());
                            }
                            option.vars = vars;
                        }
                        options.push(option);
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
            let mut reader = BufReader::new(stdout);
            loop {
                let mut buf = String::new();
                match reader.read_line(&mut buf) {
                    Ok(_) => {
                        let _ = out_tx.send(Output(buf));
                        continue;
                    }
                    Err(_) => {
                        let _ = out_tx.send(Crash(format!("Engine with pid {id} crashed!")));
                        break;
                    }
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
                        let _ =out_tx1.send(Crash(format!("Engine with pid {id} crashed!")));
                    }
                }
            }
        });

        Ok(Self {
            child,
            sender: in_tx,
            receiver: out_rx,
            _engine: engine,
        })
    }

    pub fn send(&self, command: &str) {
        self.sender.send(format!("{}\n", command.to_string())).unwrap();
    }

    pub fn quit(&mut self) {
        self.send("quit");
        let before = std::time::Instant::now();
        loop {
            if let Ok(Some(status)) = self.child.try_wait() {
                println!("Exited correctly with {status}");
                break;
            }
            if before.elapsed().as_millis() > 500 {
                self.child.kill().unwrap();
                println!("Engine did not quit gracefully in less than 500 ms, and was force killed.");
                break;
            }
        }
    }

    pub fn receive(&self) -> Message {
        self.receiver.recv().unwrap()
    }
}