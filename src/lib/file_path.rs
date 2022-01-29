use super::cli::CLI;
use std::fs;
use std::path::{Path, PathBuf};

const INCLUDED_EXTENSIONS: [&str; 2] = ["ts", "tsx"];

pub struct FilePath {}

type FilePathList = Vec<String>;

impl FilePath {
    pub fn scan_root(path: &PathBuf) -> FilePathList {
        let path = Path::new(path);
        let mut path_list: Vec<PathBuf> = Vec::new();

        Self::read_dirs_and_files_to_paths(path, &mut path_list);

        path_list
            .into_iter()
            .filter(|path| Self::file_includes_phrase(&path))
            .map(|path| String::from(path.to_str().expect("Oops")))
            .collect()
    }

    fn is_file_included(file_path: &PathBuf) -> bool {
        if let Some(extension) = file_path.extension() {
            return INCLUDED_EXTENSIONS.contains(
                &extension
                    .to_str()
                    .expect("Extension can't be converted to string"),
            );
        }

        false
    }

    fn read_dirs_and_files_to_paths(path: &Path, store: &mut Vec<PathBuf>) {
        let dirs = fs::read_dir(path).expect("Can't read path");

        for entry in dirs {
            if let Ok(entry) = entry {
                if let Ok(meta) = entry.metadata() {
                    let inner_path = entry.path();

                    match meta.is_dir() {
                        true => Self::read_dirs_and_files_to_paths(inner_path.as_path(), store),
                        false => {
                            if Self::is_file_included(&inner_path) {
                                store.push(entry.path())
                            }
                        }
                    }
                }
            }
        }
    }

    fn file_includes_phrase(path: &PathBuf) -> bool {
        let error_msg = format!("Something went wrong reading {}", &path.to_str().unwrap());
        let CLI { from, .. } = CLI::get_args();

        fs::read_to_string(path)
            .expect(&error_msg)
            .lines()
            .enumerate()
            .any(|(_, line)| line.contains(&from))
    }
}
