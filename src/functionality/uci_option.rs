use std::str::FromStr;

use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum InnerUCIOption {
    /// (value, default)
    Check(bool, bool),
    /// (value, default, min, max)
    Spin(i64, i64, i64, i64),
    /// (value, default, vars)
    Combo(String, String, Vec<String>),
    #[default]
    Button,
    /// (value, default) with "<empty>" when empty
    String(String, String),
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UCIOption {
    pub name: String,
    pub is_uci_option: bool,
    pub inner: InnerUCIOption,
}

impl UCIOption {
    pub fn parse(command: &mut &str) -> Result<Self, String> {
        let original = command.clone();

        assert!(matches!(take_next(command), Ok("option")));

        let name: &str;
        let is_uci_option: bool;

        match take_next(command) {
            Ok("name") => {
                (name, *command) = command.split_once(" type ").unwrap();
                *command = command.trim();

                is_uci_option = 
                    name.starts_with("UCI")
                    || name == "Hash" 
                    || name == "NalimovPath"
                    || name == "NalimovCache"
                    || name == "Ponder" 
                    || name == "OwnBook" 
                    || name == "MultiPV";
            },
            _ => return Err(format!("Expected 'type' keyword in '{original}'"))
        }
;
        let inner = match take_next(command) {
            Ok("check") => {
                let default = match take_next(command) {
                    Ok("default") => parse_next(command)?,
                    _ => false
                };
                InnerUCIOption::Check(default, default)
            },
            Ok("spin") => {
                let mut min = None;
                let mut max = None;
                let mut default = None;
                loop {
                    match take_next(command) {
                        Ok("default") => default = Some(parse_next::<i64>(command)?),
                        Ok("min") => min = Some(parse_next::<i64>(command)?),
                        Ok("max") => max = Some(parse_next::<i64>(command)?),
                        Ok(unexpected) => return Err(format!("Unexpected token '{unexpected}' in '{original}'")),
                        _ => break
                    }
                }

                let min = match min {
                    Some(n) => n,
                    None => return Err(format!("No 'min' provided for spin option '{original}'")),
                };

                let max = match max {
                    Some(n) => n,
                    None => return Err(format!("No 'max' provided for spin option '{original}'")),
                };

                let default = match default {
                    Some(n) => n,
                    None => return Err(format!("No 'default' provided for spin option '{original}'")),
                };

                InnerUCIOption::Spin(default, default, min, max)
            },
            Ok("combo") => {
                let mut vars: Vec<String> = Vec::new();
                let default;
                match take_next(command) {
                    Ok("default") => {
                        (default, *command) = match command.split_once(" var ") {
                            Some(pair) => pair,
                            None => return Err("Expected at least one var for combo option '{original}'".to_string()),
                        }
                    },
                    _ => return Err(format!("No 'default' provided for combo option '{original}'")),
                };

                loop {
                    match command.split_once(" var ") {
                        Some((var, cmd)) => {
                            *command = cmd.trim();
                            vars.push(var.to_string())
                        },
                        None => {
                            vars.push(command.to_string());
                            break;
                        },
                    }
                }

                InnerUCIOption::Combo(default.to_owned(), default.to_owned(), vars)
            },
            Ok("button") => {
                InnerUCIOption::Button
            },
            Ok("string") => {
                let default = match take_next(command) {
                    Ok("default") => command,
                    Ok(unexpected) => return Err(format!("Unexpected token '{unexpected}' in '{original}'")),
                    _ => "<empty>"
                };

                let default = match default {
                    "<empty>" => "".to_string(),
                    any => any.to_string()
                };

                InnerUCIOption::String(default.clone(), default)
            },
            _ => return Err(format!("Expected 'check', 'spin', 'combo', 'button', or 'string' keyword in '{original}'"))
        };

        Ok(UCIOption { name: name.trim().to_string(), is_uci_option, inner })
    }
}

pub fn take_next<'a>(command: &'a mut &str) -> Result<&'a str, String> {
    if command.is_empty() {
        return Err("String was empty.".to_string())
    }

    let (next, rest) = command.split_once(' ').unwrap_or((command, ""));

    let rest = rest.trim();

    *command = rest;

    Ok(next)
}

pub fn parse_next<'a, T: FromStr>(command: &'a mut &str) -> Result<T, String> {
    let next = take_next(command)?;

    return match next.parse::<T>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Could not parse {} as {}", next, std::any::type_name::<T>())),
    }
}