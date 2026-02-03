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
    sync::Mutex,
};

use anyhow::Result;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};
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

const MAX_DEPTH: u16 = 30;
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
    let mut whole_fs: HashSet<PathBuf> = lookup_dir(&home).unwrap();
    let test1 = explore(whole_fs);
    println!("{:?}", test1);

    save_history(&file_path, &history).expect("Couldn't save the history file...");
}

fn explore(whole_fs: HashSet<PathBuf>) -> HashSet<PathBuf> {
    let pool = ThreadPoolBuilder::new().num_threads(0).build().unwrap();
    pool.install(|| {
        let mut dirs = select_dirs(&whole_fs);
        let mut discovered_files: HashSet<PathBuf> = HashSet::new();

        for _ in 0..MAX_DEPTH {
            let (new_dirs, new_files) = dirs
                .par_iter()
                .filter_map(|dir| lookup_dir(dir).ok())
                .fold(
                    || (HashSet::<PathBuf>::new(), HashSet::<PathBuf>::new()),
                    |(mut acc_dirs, mut acc_files), inside| {
                        acc_dirs.extend(select_dirs(&inside));
                        acc_files.extend(select_files(&inside));
                        (acc_dirs, acc_files)
                    },
                )
                .reduce(
                    || (HashSet::<PathBuf>::new(), HashSet::<PathBuf>::new()),
                    |(mut d1, mut f1), (d2, f2)| {
                        d1.extend(d2);
                        f1.extend(f2);
                        (d1, f1)
                    },
                );

            discovered_files.extend(new_files);
            dirs = new_dirs;
        }

        discovered_files
    })
}

fn select_dirs(source: &HashSet<PathBuf>) -> HashSet<PathBuf> {
    let mut files: HashSet<PathBuf> = HashSet::new();
    for elt in source {
        if elt.is_dir() {
            files.insert(elt.to_owned());
        }
    }
    files
}

fn select_files(source: &HashSet<PathBuf>) -> HashSet<PathBuf> {
    let mut files: HashSet<PathBuf> = HashSet::new();
    for elt in source {
        if elt.is_file() {
            files.insert(elt.to_owned());
        }
    }
    files
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
