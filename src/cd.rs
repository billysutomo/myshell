// read cd - change the working directory spesification here https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cd.html

use std::{
    env::{self},
    io::{self},
    path::Path,
    str::SplitWhitespace,
};

pub fn change_directory(args: SplitWhitespace) -> Result<(), io::Error> {
    let (command, param) = extract_command(args);

    if command != "-" {
        env::set_var("OLDPWD", env::current_dir().unwrap());
    }

    match command.as_str() {
        "~" => {
            if param != "" {
                let full_path = home::home_dir().unwrap().join(param);
                let root = Path::new(&full_path);
                return env::set_current_dir(&root);
            }
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

/**
 * supported
 * 1. cd ~ || current home path
 * 2. cd ~/directory || relative path
 * 3. cd . || current path
 * 4. cd .. || step back path
 * 5. cd - || back to last path
 */
fn extract_command(args: SplitWhitespace) -> (String, String) {
    let mut original_string: String = args.collect();
    let two_char: String = original_string.clone().chars().take(2).collect();

    match two_char.as_str() {
        ".." => {
            return ("..".to_string(), "".to_string());
        }
        "~/" => {
            original_string.remove(0);
            original_string.remove(0);
            return ("~".to_string(), original_string);
        }
        x => {
            let b: String = x.chars().take(1).collect();
            match b.as_str() {
                "." => {
                    return (".".to_string(), "".to_string());
                }
                "~" => {
                    return ("~".to_string(), "".to_string());
                }
                "-" => {
                    return ("-".to_string(), "".to_string());
                }
                _ => {
                    return (original_string, "".to_string());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cd::extract_command;

    #[test]
    fn directory_path() {
        let result = extract_command("directory".split_whitespace());
        assert_eq!(result, ("directory".to_string(), "".to_string()));
    }

    #[test]
    fn relative_path() {
        let result = extract_command("~/directory".split_whitespace());
        assert_eq!(result, ("~".to_string(), "directory".to_string()));
    }

    #[test]
    fn current_path() {
        let result = extract_command(".".split_whitespace());
        assert_eq!(result, (".".to_string(), "".to_string()));
    }

    #[test]
    fn last_path() {
        let result = extract_command("-".split_whitespace());
        assert_eq!(result, ("-".to_string(), "".to_string()));
    }

    #[test]
    fn back_path() {
        let result = extract_command("..".split_whitespace());
        assert_eq!(result, ("..".to_string(), "".to_string()));
    }
}
