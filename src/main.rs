mod lib;

use lib::{cli::CLI, file_path::FilePath};
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let CLI { from, to, dir } = CLI::get_args();

    let path_list = FilePath::scan_root(&dir);

    for path in path_list {
        let new_file_contents = fs::read_to_string(&path)?.replace(&from, &to);

        fs::write(path, new_file_contents)?;
    }

    Ok(())
}
