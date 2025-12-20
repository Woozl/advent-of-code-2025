use std::{fs, path::Path};

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Splitter,
    Start,
    Value(usize),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-07/input.txt"))?;

    let mut grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => Cell::Start,
                    '^' => Cell::Splitter,
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect();

    grid.last_mut().unwrap().fill(Cell::Value(1));

    'row_loop: for row_idx in (1..grid.len()).rev() {
        let (before, after) = grid.split_at_mut(row_idx);
        let last_row: &Vec<Cell> = &after[0];
        let curr_row: &mut Vec<Cell> = &mut before[before.len() - 1];

        for (col_idx, cell) in curr_row.iter_mut().enumerate() {
            let next_cell: Cell = match cell {
                Cell::Start => {
                    if let Cell::Value(val) = &last_row[col_idx] {
                        println!("{}", val);
                    } else {
                        panic!("Cell below start didn't have a value");
                    }
                    break 'row_loop;
                }
                Cell::Splitter => {
                    let left = &last_row[col_idx - 1];
                    let right = &last_row[col_idx + 1];
                    if let (Cell::Value(left), Cell::Value(right)) = (left, right) {
                        Cell::Value(left + right)
                    } else {
                        panic!("Cells on rows below splitter were not values");
                    }
                }
                _ => last_row[col_idx].clone(),
            };
            *cell = next_cell;
        }
    }

    Ok(())
}
