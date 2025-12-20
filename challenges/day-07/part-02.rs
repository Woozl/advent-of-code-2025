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

    // work from the bottom of the grid up, consider all bottom cells as starting positions
    grid.last_mut().unwrap().fill(Cell::Value(1));

    'o: for row_idx in (1..grid.len()).rev() {
        // get a mutable reference to the current row (curr_row) to update the grid's values
        // last_row is a immutable ref to the row directly below, used to generate values for the current row
        let (before, after) = grid.split_at_mut(row_idx);
        let last_row: &Vec<Cell> = &after[0];
        let curr_row: &mut Vec<Cell> = &mut before[before.len() - 1];

        for (col_idx, cell) in curr_row.iter_mut().enumerate() {
            // update the curr_row based on the last_row
            *cell = match cell {
                // if the cell is the start cell, we've found the solution.
                // just use the value from the cell right below
                //   . S .
                //   1 2 1  -> solution: 2
                //   1 1 1
                Cell::Start => {
                    let Cell::Value(val) = &last_row[col_idx] else {
                        panic!("Cell below start didn't have a value");
                    };
                    println!("{}", val);
                    break 'o;
                }

                // if the cell is a splitter, it changes into a Value that is
                // the sum of the bottom left and bottom right cell values
                //   . ^ .   becomes   . 3 .
                //   1 1 2      ->     1 1 2
                Cell::Splitter => {
                    let left = &last_row[col_idx - 1];
                    let right = &last_row[col_idx + 1];
                    if let (Cell::Value(left), Cell::Value(right)) = (left, right) {
                        Cell::Value(left + right)
                    } else {
                        panic!("Cells on rows below splitter were not values");
                    }
                }

                // this is wildcard to satisfy Rust's type checker, but this
                // branch should only ever handle `Empty`s.
                //
                // if a cell is empty, copy the values from the previous row up
                //   . . .   becomes   1 2 1
                //   1 2 1      ->     1 2 1
                _ => last_row[col_idx].clone(),
            };
        }
    }

    Ok(())
}
