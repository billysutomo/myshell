use std::{
    env,
    io::{self, stdout, Write},
    process::{Child, Command, Stdio},
};

mod cd;

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap();
        print!("{}$ ", current_dir.display());

        match stdout().flush() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("got error when flushing {}", e)
            }
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // must be peekable so we know when we are on the last command
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
            match command {
                "cd" => {
                    if let Err(e) = cd::change_directory(args) {
                        eprintln!("{}", e)
                    }
                    previous_command = None;
                }
                "exit" => return,
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    // gracefully handle malformed user input
                    match output {
                        Ok(output) => previous_command = Some(output),
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e)
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            match final_command.wait() {
                Ok(status) => {
                    if !status.success() {
                        println!("execute command failed: {status}");
                    }
                }
                Err(e) => {
                    eprintln!(" error here {}", e)
                }
            }
        }
    }
}
