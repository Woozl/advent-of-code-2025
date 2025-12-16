use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-06/input.txt"))?;

    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (operands_chars, number_lines) = lines.split_last().unwrap();

    let grid = number_lines.to_vec();

    // Vec<(operand, starting_position, width)>
    let operands: Vec<(char, usize, usize)> = {
        let mut list: Vec<(char, usize, usize)> = vec![];
        let mut current_char = operands_chars[0];
        let mut starting_idx: usize = 0;
        let mut width: usize = 1;
        
        for (i, c) in (&operands_chars[1..]).iter().enumerate() {
            if *c != ' ' {
                list.push((current_char, starting_idx, width - 1));

                current_char = *c;
                starting_idx = i + 1;
                width = 1;
            } else {
                width += 1;
            }
        }
        list.push((current_char, starting_idx, width));
        list
    };

    let mut total: usize = 0;
    for (operand, starting_position, width) in operands {
        let mut result: usize = if operand == '*' { 1 } else { 0 };
        for col_idx in starting_position..(starting_position + width) {
            let mut num_str = String::from("");
            for row_idx in 0..grid.len() {
                let c = grid[row_idx][col_idx];
                if  c != ' ' {
                    num_str.push(c);
                }
            }
            
            let num = num_str.parse::<usize>().unwrap();
            if operand == '*' {
                result *= num;
            } else {
                result += num;
            }
        }
        total += result;
    }

    println!("{total}");

    Ok(())
}
