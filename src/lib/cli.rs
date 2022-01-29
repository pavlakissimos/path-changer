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
        let args = env::args().into_iter().collect::<Vec<String>>();
        let args = Arguments::parse(&args);

        let from = args.0;
        let to = args.1;
        let dir = PathBuf::from("./");

        CLI { from, to, dir }
    }
}

struct Arguments(String, String);

impl Arguments {
    pub fn parse(args: &Vec<String>) -> Self {
        let args: Vec<&String> = args
            .into_iter()
            .enumerate()
            .filter_map(|(idx, arg)| {
                if idx == 0 {
                    return None;
                }
                if arg.starts_with("-") {
                    return None;
                }

                Some(arg)
            })
            .collect();

        if args.len() != 2 {
            panic!("Handle args error");
        }

        let first = args.get(0).unwrap().to_string();
        let second = args.get(1).unwrap().to_string();

        Arguments(first, second)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_args() {
        let args = vec![
            String::from("program name"),
            String::from("Arg 1"),
            String::from("Arg 2"),
        ];

        let parsed_args = Arguments::parse(&args);

        assert_eq!(parsed_args.0, String::from("Arg 1"));
        assert_eq!(parsed_args.1, String::from("Arg 2"));
    }

    #[test]
    #[should_panic]
    fn parse_one_args_and_panic() {
        let args = vec![String::from("program name"), String::from("Arg 1")];

        Arguments::parse(&args);
    }
}
