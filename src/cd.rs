// read cd - change the working directory spesification here https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cd.html

use std::{
    env::{self},
    io::{self},
    path::Path,
    str::SplitWhitespace,
};

pub fn change_directory(args: SplitWhitespace) -> Result<(), io::Error> {
    let (first, second) = extract_command(args);

    println!("first {first}");
    println!("second {second}");
    if second != "-" {
        env::set_var("OLDPWD", env::current_dir().unwrap());
    }

    match first.as_str() {
        "~" => {
            println!("pattern ~|{}", second);
            if second != "" {
                let full_path = home::home_dir().unwrap().join(second);
                let root = Path::new(&full_path);
                return env::set_current_dir(&root);
            }
            let home_dir = home::home_dir().unwrap();
            let home_path = home_dir.as_os_str();
            let root = Path::new(home_path);
            env::set_current_dir(&root)
        }
        "-" => {
            println!("pattern -|");
            let a = env::var("OLDPWD").unwrap();
            env::set_current_dir(a)
        }
        x => {
            println!("pattern x|{x}");
            let root = Path::new(x);
            env::set_current_dir(&root)
        }
    }
}

/**
 * supported
 * 1. cd ~
 * 2. cd ~/Engineering or directory with relative path
 * 3.
 *
 * bug
 * 1. cd .
 * 2. cd ..
 * 3. cd - / or back to last history or path
 *
 */
fn extract_command(args: SplitWhitespace) -> (String, String) {
    let mut binding = args.peekable();
    binding.peek();
    let new_dir = binding.next().map_or("~", |x| x);
    let mut a = String::default();
    let mut b = new_dir.chars();
    a.push(b.next().unwrap());
    b.next();

    return (a.to_string(), b.as_str().to_string());
}

fn extract_command_new(args: SplitWhitespace) -> (String, String) {
    let sub = args.take(2).collect();

    return (a.to_string(), b.as_str().to_string());
}

#[cfg(test)]
mod tests {
    use crate::cd::extract_command;

    #[test]
    fn relative_path() {
        let result = extract_command("~/Engineering".split_whitespace());
        assert_eq!(result, ("~".to_string(), "Engineering".to_string()));
    }

    #[test]
    fn current_path() {
        let result = extract_command(".".split_whitespace());
        assert_eq!(result, (".".to_string(), "".to_string()));
    }
}
