use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use super::*;
use InnerUCIOption::{Check, Spin, Combo, Button, };

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Engine {
    pub id: u32,
    pub name: String,
    pub author: String,
    pub alias: String,
    pub elo: u32,
    pub path: PathBuf,
    pub options: Vec<UCIOption>,
    pub go_commands: String,
    pub launch_commands: String,
}

impl Engine {
    pub fn reload(&mut self) -> Result<(), String> {
        let updated = EngineWrapper::get_info(self.path.clone())?;
        
        self.name = updated.name;
        self.author = updated.author;

        let old_options = self.options.clone();
        let mut updated_options: Vec<UCIOption> = Vec::new();

        // Add new options, and remove old options
        for option in updated.options {
            let mut option = option.clone();
            if let Some(old_option) = old_options.iter().find(|old_option| old_option.name == option.name) {
                let inner = match (option.inner.clone(), old_option.inner.clone()) {
                    (Check(_, a), Check(old_value, _)) => Check(old_value, a),
                    (Spin(_, a, b, c), Spin(old_value, _, _, _)) => Spin(old_value, a, b, c),
                    (Combo(_, a, b), Combo(old_value, _, _)) => Combo(old_value, a, b),
                    (Button, Button) => Button,
                    (InnerUCIOption::String(_, a), InnerUCIOption::String(old_value, _)) => InnerUCIOption::String(old_value, a),
                    (new, _) => {
                        println!("Option {} has changed type, and its value has been forgotten", option.name);
                        new
                    },
                };
                option.inner = inner;
            }

            updated_options.push(option)
        }

        self.options = updated_options;

        Ok(())
    }

    pub fn set_option(&mut self, name: &str, value: &str) -> Result<(), String> {
        let option = self.options.iter_mut().find(|option| option.name == name).ok_or(format!("Option {} could not be set as it does not exist", name))?;
        let inner = match option.inner.clone() {
            Check(_, def) => Check(value.parse().expect("Could not parse {value} as a boolean"), def),
            Spin(_, def, b, c) => Spin(value.parse().expect("Could not parse {value} as an integer"), def, b, c),
            Combo(_, def, vars) => {
                if !vars.contains(&value.to_string()) {
                    return Err(format!("Option {} could not be set to {} as it is not a valid option", name, value));
                }
                Combo(value.to_string(), def, vars)
            },
            Button => todo!(),
            InnerUCIOption::String(_, def) => InnerUCIOption::String(value.to_string(), def),
        };
        option.inner = inner;
        Ok(())
    }

    pub fn restore_defaults(&mut self) {
        for option in self.options.iter_mut() {
            let inner = match option.inner.clone() {
                Check(_, def) => Check(def, def),
                Spin(_, def, b, c) => Spin(def, def, b, c),
                Combo(_, def, vars) => Combo(def.clone(), def.clone(), vars),
                Button => Button,
                InnerUCIOption::String(_, def) => InnerUCIOption::String(def.clone(), def.clone()),
            };
            option.inner = inner;
        }
    }
}