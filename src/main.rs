use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self, File},
    io::Error,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
enum Existence {
    FolderExist,
    AllExist,
    WasCreated,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct SearchHistory {
    names: HashSet<String>,
    total_count: u32,
    most_recent: bool,
}

const DIR_PATH: &str = ".findr";

const FILE_NAME: &str = "findr.bin";

fn main() {
    let home = PathBuf::from(env::var("HOME").expect("HOME isn't defined"));
    let folder_path = home.join(PathBuf::from(DIR_PATH));
    let file_path = folder_path.join(PathBuf::from(FILE_NAME));
    let has_file = match folder_creation(&folder_path, &file_path) {
        Ok(Existence::FolderExist) => false,
        Ok(Existence::WasCreated) => false,
        Ok(Existence::AllExist) => true,
        Err(e) => panic!("{e}"),
    };
    let history: HashMap<PathBuf, SearchHistory> = if !has_file {
        match create_history(&file_path) {
            Ok(history) => history,
            Err(e) => panic!("Couldn't load the history file: {e}"),
        }
    } else {
        match load_history(&file_path) {
            Ok(history) => history,
            Err(e) => panic!("Couldn't load the history file: {e}"),
        }
    };
    println!("{:?}", history);

    let subdir = lookup_dir(&home).expect("Couldn't read home dir");

    println!("{:?}", subdir);
}

/// Function that return a HashSet of PathBuf that contain the content of starting_dir or an Error
fn lookup_dir(starting_dir: &Path) -> Result<HashSet<PathBuf>, Error> {
    let entries = fs::read_dir(starting_dir)?;

    let mut found = HashSet::new();

    for entry in entries {
        found.insert(entry?.path());
    }
    Ok(found)
}
fn create_history(file_path: &Path) -> Result<HashMap<PathBuf, SearchHistory>, Error> {
    File::create_new(file_path)?;
    Ok(HashMap::new())
}
fn save_history(file_path: &Path) -> Result<HashMap<PathBuf, SearchHistory>, Error> {
    todo!()
}
fn load_history(file_path: &Path) -> Result<HashMap<PathBuf, SearchHistory>, Error> {
    todo!()
}

fn folder_creation(folder_path: &Path, file_path: &Path) -> Result<Existence, Error> {
    if folder_path.exists() {
        if file_path.exists() {
            Ok(Existence::AllExist)
        } else {
            Ok(Existence::FolderExist)
        }
    } else {
        fs::create_dir(folder_path)?;
        Ok(Existence::WasCreated)
    }
}
