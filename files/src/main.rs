use std::fs::File;
use std::path::Path;

fn main() {
    let file_path = Path::new("/home/torben/test.txt");

    let file_create_optional: Result<File, std::io::Error> = File::create(&file_path);

    match file_create_optional {
        Err(error) => println!("Error while creating {}: {}", file_path.display(), error),
        Ok(_) => println!("Ok"),
    }
}
