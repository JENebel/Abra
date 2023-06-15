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
}

impl EngineWrapper {
    pub fn launch(/*path: &str*/) -> Result<Self, String> {
        let path = "C:/Users/Joachim/VSCode Projects/Cadabra/target/release/cadabra.exe";

        let mut child = match Command::new(path).stdout(Stdio::piped()).stdin(Stdio::piped()).spawn() {
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