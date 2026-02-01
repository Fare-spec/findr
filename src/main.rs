/// TODO: Implement config (priority: low)
/// TODO: Implement recursive search thread by thread using rayon (priority: high) (current)
/// TODO: Implement history once the search is complete (priority: medium)
/// TODO: Change the way to handle Exception to avoid crash (priority: medium)
/// TODO: Stop using Existence enum, at least rename it "State" or don't use it.(priority: low)
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self, File},
    io::{BufWriter, Error, Read},
    path::{Path, PathBuf},
};

use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

enum Existence {
    FolderExist,
    AllExist,
    WasCreated,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct SearchHistory {
    names: HashMap<String, u32>,
    most_recent: bool,
}

const DIR_PATH: &str = ".findr";

const FILE_NAME: &str = "findr.bin";
const CONFIG_FILE: &str = "findr.toml";
const CONFIG_PATH: &str = ".config/findr";

const MAX_DEPTH: u16 = 20;
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
    let mut whole_fs: HashSet<PathBuf> = HashSet::new();
    let mut subdir = lookup_dir(&home).expect("Couldn't read home dir");

    //create_threads(&mut subdir, 0, &mut whole_fs);

    println!("{:?}", whole_fs);
    save_history(&file_path, &history).expect("Couldn't save the history file...");
}

fn explore(whole_fs: HashSet<PathBuf>,)


/*
fn create_threads(
    paths: &mut HashSet<PathBuf>,
    current_depth: u16,
    files_new: &mut HashSet<PathBuf>,
) -> () {
    println!("{:?}", files_new);
    let mut p2 = paths.clone();
    p2.retain(|f| f.is_file());
    files_new.extend(p2);
    paths.par_iter().for_each(|x| explore(x, current_depth + 1));
}

/*

    let mut files_old = which_files(paths);
    files_old.extend(files_new.iter().cloned());
    let files: HashSet<PathBuf> = which_files(paths);

    paths.retain(|paf| paf.is_dir() && !paf.starts_with("/home/spectre/."));

    paths.par_iter().for_each(|x| explore(x, current_depth));

    Ok(files_old)
* */

fn explore(path: &Path, depth: u16) -> () {
    if depth >= MAX_DEPTH {
    } else {
        let mut files = lookup_dir(path).unwrap_or(HashSet::with_capacity(0));
        let mut ff = files.clone();
        create_threads(&mut files, depth + 1, &mut ff);
    }
}

*/
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

fn save_history(file_path: &Path, history: &HashMap<PathBuf, SearchHistory>) -> Result<()> {
    let file = File::create(file_path)?;
    let mut buffer = BufWriter::new(file);
    postcard::to_io(&history, &mut buffer)?;
    Ok(())
}

fn load_history(file_path: &Path) -> Result<HashMap<PathBuf, SearchHistory>> {
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let history = postcard::from_bytes(&buf)?;
    Ok(history)
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
