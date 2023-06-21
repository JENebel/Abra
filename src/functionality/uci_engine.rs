use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use super::*;

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