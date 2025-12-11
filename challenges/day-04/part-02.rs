use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-04/input.txt"))?;

    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();

    let mut removed_rolls: usize = 0;

    loop {
        let removed = remove_rolls(&mut grid);

        if removed == 0 {
            break;
        }

        removed_rolls += removed;
    }

    println!("{removed_rolls:?}");

    Ok(())
}

fn remove_rolls(grid: &mut Vec<Vec<bool>>) -> usize {
    let mut sum = 0;

    #[rustfmt::skip]
    let dirs: &[(isize, isize); 8] = &[
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let mut next_grid: Vec<Vec<bool>> = grid.iter().cloned().collect();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if !*col {
                continue;
            }

            let mut num_rolls_surrounding: usize = 0;

            for dir in dirs {
                let x: isize = col_idx as isize + dir.0;
                let y: isize = row_idx as isize + dir.1;

                if x < 0 || x >= row.len() as isize || y < 0 || y >= grid.len() as isize {
                    continue;
                }

                if *col && grid[y as usize][x as usize] {
                    num_rolls_surrounding += 1;
                }
            }

            if num_rolls_surrounding < 4 {
                next_grid[row_idx][col_idx] = false;
                sum += 1;
            }
        }
    }

    *grid = next_grid;
    sum
}
