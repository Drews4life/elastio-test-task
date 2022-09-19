use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn get_config_file_path() -> PathBuf {
    PathBuf::from("./elastiocfg.json")
}

fn does_config_exists() -> bool {
    get_config_file_path().exists()
}

fn get_config_map() -> Option<HashMap<String, String>> {
    File::open(get_config_file_path()).map_or(None, |file| serde_json::from_reader(file).ok())
}

fn get_raw_entry(entry: &str) -> Option<String> {
    get_config_map().and_then(|map| map.get(entry).cloned())
}

fn create_config_file() {
    let mut file = File::create(get_config_file_path()).expect("Unable to create config file");

    file.write_all("{}".as_bytes())
        .expect("Unable to write initial data to file");
}

fn set_entry_inner(entry: &str, value: &str) {
    let mut config_map = get_config_map().unwrap();

    config_map.insert(entry.to_owned(), value.to_owned());

    let serialized = serde_json::to_string(&config_map).unwrap();

    File::create(get_config_file_path())
        .expect("File should exist")
        .write_all(serialized.as_bytes())
        .expect("Unable to write to file");
}

pub fn set_entry(entry: &str, value: &str) {
    if !does_config_exists() {
        create_config_file();
    }

    set_entry_inner(entry, value);
}

pub fn get_entry(entry: &str) -> Option<String> {
    get_raw_entry(entry)
}
