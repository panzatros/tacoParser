use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

mod token;
mod scanner;

fn write_to_file() -> io::Result<()> {
    let path = Path::new("filesTesting/example.txt");
    let mut file = File::create(path)?;
    file.write_all(b"Hello, Rust!")?;
    Ok(())
}

fn read_file(path_string: &str) -> io::Result<()> {
    let path = Path::new(path_string);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    read_eacch_char(&contents);
    println!("{}", contents);
    Ok(())
}

fn read_file_with_buffer(path_string: &str) -> io::Result<()> {
    let path = Path::new(path_string);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // Process each line
        println!("\n{}", line);
    }
    Ok(())
}

fn read_eacch_char(content: &str) {
    for items in content.chars() {
        print!("{} ", items);
        char_matching(items);
    }
}

fn char_matching(c: char) {
    match c {
        'a' => println!("taco"),
        'b' => println!("anotherTaco"),
        _ => println!("alv"),
    }
}

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);

    let taco = "filesTesting/lorem.txt";

    //    if let Err(e) = write_to_file() {
    //        eprintln!("Error write to file: {}", e);
    //    }

    //    if let Err(e) = read_file(taco) {
    //        eprintln!("Error reading to file: {}", e);
    //    }

    if let Err(e) = read_file_with_buffer(taco) {
        eprintln!("Error reading to file: {}", e);
    }

    scanner::Scanner::test();
}
