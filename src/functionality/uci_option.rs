use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UCIOption {
    pub name: String,
    pub value: String,
    pub default: String,
    pub opt_type: String,
    pub min: String,
    pub max: String,
    pub vars: Vec<String>,
}