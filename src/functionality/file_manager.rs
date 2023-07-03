use std::{collections::HashMap, fs, path::PathBuf};

use native_dialog::FileDialog;

use super::*;

const ENGINE_FOLDER: &str = "./engines/";

pub fn init_directories() -> Result<(), String> {
    // Create engines folder if it doesn't exist
    if !fs::metadata(ENGINE_FOLDER).is_ok() {
        fs::create_dir(ENGINE_FOLDER).unwrap();
    }
    Ok(())
}

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

// Serialize and store engine with id as filename
pub fn store_engine(engine: &Engine) -> Result<(), String> {
    let serialized = serde_json::to_string_pretty(&engine).unwrap();
    fs::write(format!("{}{}.json", ENGINE_FOLDER, engine.id), serialized).unwrap();
    println!("Stored '{}'", engine.alias);
    Ok(())
}

pub fn remove_engine(id: u32) -> Result<(), String> {
    fs::remove_file(format!("{}{}.json", ENGINE_FOLDER, id)).unwrap();
    Ok(())
}

pub fn open_file_dialog() -> Result<PathBuf, String> {
    open_file_dialog_with_path(PathBuf::new())
}

pub fn open_file_dialog_with_path(path: PathBuf) -> Result<PathBuf, String> {
    let path = match FileDialog::new()
        .set_location(path.parent().unwrap_or("".as_ref()))
        .show_open_single_file() {
            Ok(p) => match p {
                Some(p) => p,
                None => return Err("No file selected!".to_string()),
            },
            Err(err) => return Err(format!("Could not open file dialog! '{err}'")),
    };

    Ok(path)
}

pub fn install_engine(new_id: u32) -> Result<Engine, String> {
    let path = open_file_dialog()?;

    let mut engine = EngineWrapper::get_info(path)?;
    engine.id = new_id;

    store_engine(&engine).unwrap();

    Ok(engine)
}