use std::{collections::HashMap, fs};

use super::*;

const ENGINE_FOLDER: &str = "./engines/";

pub fn load_all_engines() -> Result<HashMap<u32, Engine>, String> {
    let mut engines: HashMap<u32, Engine> = HashMap::new();

    // Load all engines in the engines folder
    let paths = fs::read_dir(ENGINE_FOLDER).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        // Deserialize and add to hashmap
        let engine: Engine = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        engines.insert(engine.id, engine);
    }

    Ok(engines)
}

pub fn load_engine(id: u32) -> Result<Engine, String> {
    let path = format!("{}{}.json", ENGINE_FOLDER, id);
    let engine: Engine = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    Ok(engine)
}

pub fn store_engines(engines: &HashMap<u32, Engine>) -> Result<(), String> {
    for engine in engines.values() {
        store_engine(engine.clone())?;
    }
    Ok(())
}

// Serialize and store engine with id as filename
pub fn store_engine(engine: Engine) -> Result<(), String> {
    let serialized = serde_json::to_string(&engine).unwrap();
    fs::write(format!("{}{}.json", ENGINE_FOLDER, engine.id), serialized).unwrap();
    Ok(())
}