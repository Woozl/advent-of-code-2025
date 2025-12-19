use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-07/input.txt"))?;

    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut num_splits = 0;

    let start: (usize, usize) = (grid[0].iter().position(|c| *c == 'S').unwrap(), 0);

    let mut lasers = vec![start];
    while lasers.len() > 0 {
        let mut next_lasers: Vec<(usize, usize)> = vec![];

        for laser in lasers.iter() {
            let next_loc = (laser.0, laser.1 + 1);

            if next_loc.1 >= grid.len() {
                continue;
            }

            let grid_char = grid[next_loc.1][next_loc.0];

            // we've already hit this splitter
            if grid_char == '*' {
                continue;
            }

            // add the 2 neighboring cells to the next lasers
            if grid_char == '^' {
                let left = (next_loc.0 - 1, next_loc.1);
                let right = (next_loc.0 + 1, next_loc.1);

                if left.0 > 0 {
                    next_lasers.push(left);
                }
                if right.0 < grid[0].len() {
                    next_lasers.push(right);
                }

                // increment the split count and put an '*' in the spot of
                // the splitter so it isn't double counted

                num_splits += 1;
                grid[next_loc.1][next_loc.0] = '*';

                continue;
            }

            // otherwise, this is an open cell, add it to the next_lasers set
            next_lasers.push(next_loc);
        }
        lasers = next_lasers;
    }

    println!("{num_splits}");

    Ok(())
}
