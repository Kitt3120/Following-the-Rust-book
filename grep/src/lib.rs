use std::{env, path::Path};

#[derive(Debug)]
pub struct Arguments {
    pub pattern: String,
    pub path: String, // We don't use Path here, as Paths are OsStr, whose size is not known at compile time
    pub case_insensitive: bool,
}

impl Arguments {
    fn new(pattern: String, path: String, case_insensitive: bool) -> Arguments {
        Arguments {
            pattern,
            path,
            case_insensitive,
        }
    }
}

pub fn run(arguments: Arguments) -> Result<(), String> {
    let content = read_file(&Path::new(&arguments.path))?;

    let lines = match arguments.case_insensitive {
        true => filter_case_insensitive(&content, &arguments.pattern),
        false => filter_case_sensitive(&content, &arguments.pattern),
    };

    println!("{}", lines.join("\n"));
    Ok(())
}

pub fn parse_arguments(args: &[String]) -> Result<Arguments, String> {
    if args.len() < 3 {
        return Err("Not enough arguments provided.\nUsage: grep <pattern> <file>".to_string());
    }

    let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

    Ok(Arguments::new(
        args[1].clone(),
        args[2].clone(),
        case_insensitive,
    ))
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

pub fn filter_case_sensitive<'a, 'b>(content: &'a str, pattern: &'b str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn filter_case_insensitive<'a, 'b>(content: &'a str, pattern: &'b str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{filter_case_insensitive, filter_case_sensitive};

    #[test]
    fn test_filter_case_sensitive() {
        let content = "Line with an X\nLine with a Y\nLine with an X and a Y\n";
        let pattern = "X";
        let expected = vec!["Line with an X", "Line with an X and a Y"];
        let result = filter_case_sensitive(content, pattern);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_case_insensitive() {
        let content = "Line with an X\nLine with a Y\nLine with an X and a Y\n";
        let pattern = "x";
        let expected = vec!["Line with an X", "Line with an X and a Y"];
        let result = filter_case_insensitive(content, pattern);
        assert_eq!(result, expected);
    }
}
