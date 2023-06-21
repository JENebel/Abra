use std::{io::stdin, process, thread, sync::mpsc::{channel, Receiver}, str::FromStr, path::PathBuf};

use super::*;


pub fn start_cli() {
    println!("{} {} by {}", PKG_NAME, PKG_VERSION, PKG_AUTHORS);

    interface_loop();
}

pub fn interface_loop() {
    // Spawn listening thread that reads input without blocking main thread
    let ui_receiver = spawn_ui_listener_thread();

    loop {
        let line = wait_for_input(&ui_receiver);
        let mut command = line.as_str().trim();

        let cmd_name = match take_next(&mut command) {
            Some(name) => name,
            None => continue, // Empty command
        };

        match cmd_name {
            "q" | "x" | "quit" | "exit" => quit(),
            "launch" => {
                match EngineWrapper::launch(Engine { path: PathBuf::default(), ..Default::default() }) {
                    Ok(mut engine) => {
                        println!("Engine launched");
                        engine.send("go movetime 2000");
                        loop {
                            match engine.receive() {
                                Message::Output(output) => {
                                    print!("{}", output);
                                    if output.contains("bestmove") {
                                        break;
                                    }
                                },
                                Message::Crash(error) => {
                                    println!("Crash: {}", error);
                                    break;
                                }
                            }
                        }
                        engine.quit();
                    },
                    Err(e) => println!("{}", e),
                }
            }
            /*"install" => {
                let engine_name = match take_next(&mut command) {
                    Some(name) => name,
                    None => {
                        println!("Missing engine name!");
                        continue
                    },
                };

                let engine_version = match take_next(&mut command) {
                    Some(version) => version,
                    None => {
                        println!("Missing engine version!");
                        continue
                    },
                };

                println!("Engine installed!");
            },*/
            _ => println!("Unknown command: {}", cmd_name)
        }
    }
}

fn quit() {
    process::exit(0)
}

fn wait_for_input(ui_receiver: &Receiver<String>) -> String {
    ui_receiver.recv().expect("Error receiving ui command!")
}

pub fn spawn_ui_listener_thread() -> Receiver<String> {
    let (sender, ui_receiver) = channel::<String>();

    // Spawn listening thread that reads input without blocking main thread
    thread::spawn(move || {
        loop {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            sender.send(buf.trim().to_string()).unwrap()
        }
    });

    ui_receiver
}

pub fn take_next<'a>(command: &'a mut &str) -> Option<&'a str> {
    if command.is_empty() {
        return None
    }

    let (next, rest) = command.split_once(' ').unwrap_or((command, ""));

    let rest = rest.trim();

    *command = rest;

    Some(next)
}

pub fn take_next_num<T: FromStr>(command: &mut &str) -> Option<T> {
    let depth_str = match take_next(command) {
        None => {
            return None
        },
        Some(depth) => {
            depth
        },
    };

    match depth_str.parse::<T>() {
        Ok(depth) => Some(depth),
        Err(_) => {
            None
        },
    }
}