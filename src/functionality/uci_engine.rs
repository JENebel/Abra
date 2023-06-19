use serde::{Serialize, Deserialize};

use super::*;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Engine {
    pub id: u32,
    pub name: String,
    pub author: String,
    pub alias: String,
    pub elo: u32,
    pub path: String,
    pub options: Vec<UCIOption>,
    pub go_commands: String,
    pub launch_commands: String,
}