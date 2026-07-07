# Minishell

Minishell is a simple shell written in Rust as a small pet project. It handles a basic interactive shell loop and supports running simple commands from the terminal.

## Features

- Runs external commands such as `ls`, `pwd`, `echo`, and other programs available on the system path.
- Passes basic command arguments separated by whitespace.
- Supports changing directories with `cd`.
- Supports exiting the shell with `exit`.
- Supports simple output redirection with `>`.
- Prints basic error messages when a command fails, a directory cannot be changed, or a file cannot be created.

## Usage

Run the project with Cargo:

```bash
cargo run
```

Example commands:

```bash
mysh>pwd
mysh>ls -la
mysh>cd src
mysh>echo hello > output.txt
mysh>exit
```

## Notes

This shell is intentionally simple. The command parser is based on whitespace splitting, so advanced shell syntax such as quotes, escaping, pipes, environment variable expansion, or multiple command operators are not in the scope of this project.

The redirection/file handling could be improved with a proper tokenizer or parser. Right now, the shell only expects one file name after `>`, like:

```bash
echo hello > output.txt
```

Other situations, such as missing files, multiple files, multiple redirections, or more complex input/output redirection, are also not in the scope of this project.
