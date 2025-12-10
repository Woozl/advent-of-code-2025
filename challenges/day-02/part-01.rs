use std::{fs, path::Path};

use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-02/input.txt"))?;

    let ids = input.split(',').map(|range| range.split('-'));

    let mut invalids: Vec<String> = vec![];

    for mut range in ids {
        let start = range.next().unwrap().parse::<usize>()?;
        let end = range.next().unwrap().parse::<usize>()?;

        for id in start..=end {
            let str_id = id.to_string();
            if is_invalid(&str_id)? {
                invalids.push(str_id);
            }
        }
    }

    println!("{invalids:?}");
    println!(
        "{}",
        invalids
            .iter()
            .map(|i| i.parse::<usize>().unwrap())
            .sum::<usize>()
    );

    Ok(())
}

fn is_invalid(id: &String) -> Result<bool, Box<dyn std::error::Error>> {
    if id.len() % 2 != 0 {
        return Ok(false);
    }

    for substring_length in 0..=id.len() / 2 {
        let subs: String = id.chars().take(substring_length).collect();
        let re = Regex::new(&subs)?;

        let sequences_removed = re.replacen(&id, 2, "").to_string();

        if sequences_removed == "" {
            return Ok(true);
        }
    }

    Ok(false)
}
