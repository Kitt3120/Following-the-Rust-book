use std::path::Path;

#[derive(Debug)]
pub struct Arguments {
    pub pattern: String,
    pub path: String, // We don't use Path here, as Paths are OsStr, whose size is not known at compile time
}

impl Arguments {
    fn new(pattern: String, path: String) -> Arguments {
        Arguments { pattern, path }
    }
}

pub fn parse_arguments(args: &[String]) -> Result<Arguments, String> {
    if args.len() < 3 {
        return Err("Not enough arguments provided.\nUsage: grep <pattern> <file>".to_string());
    }

    Ok(Arguments::new(args[1].clone(), args[2].clone()))
}

pub fn read_file(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("{} does not exist", path.display()));
    } else if !path.is_file() {
        return Err(format!("{} is not a file", path.display()));
    }

    let content = std::fs::read_to_string(path);
    if content.is_err() {
        return Err(format!(
            "Error while reading file: {}",
            content.unwrap_err()
        ));
    }

    Ok(content.unwrap())
}

pub fn filter<'a, 'b>(content: &'a str, pattern: &'b str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}
