use std::{env, path::PathBuf};

pub struct CLI {
    /// The String to replace
    pub from: String,
    /// The String to replace with
    pub to: String,
    /// Directory to apply run the program in
    pub dir: PathBuf,
}

impl CLI {
    pub fn get_args() -> Self {
        let mut args = env::args();

        let from = args.nth(1).expect("Can't read value to replace");
        let to = args.next().expect("Can't read value to replace with");
        let dir = PathBuf::from("./");

        CLI { from, to, dir }
    }
}
