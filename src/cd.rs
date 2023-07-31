// read cd - change the working directory spesification here https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cd.html

use std::{
    env::{self},
    io::{self},
    path::Path,
    str::SplitWhitespace,
};

pub fn change_directory(args: SplitWhitespace) -> Result<(), io::Error> {
    // default to '~' as new directory if one was not provided
    let new_dir = args.peekable().peek().map_or("~", |x| *x);

    if new_dir != "-" {
        env::set_var("OLDPWD", env::current_dir().unwrap());
    }
    match new_dir {
        "~" => {
            let home_dir = home::home_dir().unwrap();
            let home_path = home_dir.as_os_str();
            let root = Path::new(home_path);
            env::set_current_dir(&root)
        }
        "-" => {
            let a = env::var("OLDPWD").unwrap();
            env::set_current_dir(a)
        }
        x => {
            let root = Path::new(x);
            env::set_current_dir(&root)
        }
    }
}
