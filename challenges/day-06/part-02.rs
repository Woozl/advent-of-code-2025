use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-06/input.txt"))?;

    println!("{input}");

    Ok(())
}
