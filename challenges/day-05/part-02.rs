use std::{fs, path::Path};

use regex::RegexBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-05/input.txt"))?;

    let re = RegexBuilder::new(r"^(?<min>\d+)-(?<max>\d+)$")
        .multi_line(true)
        .build()?;
    let ranges = re.captures_iter(&input).map(|caps| {
        let min = caps.name("min").unwrap().as_str().parse::<usize>().unwrap();
        let max = caps.name("max").unwrap().as_str().parse::<usize>().unwrap();
        (min, max)
    });

    let mut rc = RangeCollection::new();

    for (min, max) in ranges {
        rc.add((min, max));
    }

    println!("{}", rc.sum_of_ranges());

    Ok(())
}

struct RangeCollection {
    ranges: Vec<(usize, usize)>,
}

impl RangeCollection {
    fn new() -> Self {
        RangeCollection { ranges: vec![] }
    }

    fn sum_of_ranges(&self) -> usize {
        self.ranges.iter().fold(0, |sum, r| sum + (r.1 - r.0 + 1))
    }

    fn add(&mut self, (insert_min, insert_max): (usize, usize)) {
        if self.ranges.len() == 0 {
            self.ranges.push((insert_min, insert_max));
            return;
        }

        let mut lower_pos: Option<usize> = None;
        let mut upper_pos: Option<usize> = None;
        for (i, (existing_min, existing_max)) in self.ranges.iter().enumerate() {
            if lower_pos.is_some() && upper_pos.is_some() {
                break;
            }

            // check minimum if not already found
            if let None = lower_pos {
                // insert minimum is below existing segment
                if insert_min < *existing_min {
                    lower_pos = Some(2 * i);
                }

                // insert minimum is IN existing segment
                if *existing_min <= insert_min && insert_min <= *existing_max {
                    lower_pos = Some(2 * i + 1);
                }

                // insert min is after the existing segment
                // we need to continue looking, unless this is the last segment
                if insert_min > *existing_max && i == self.ranges.len() - 1 {
                    lower_pos = Some(2 * i + 2);
                }
            }

            // check maximum if not already found
            if let None = upper_pos {
                // insert maximum is below existing segment
                if insert_max < *existing_min {
                    upper_pos = Some(2 * i);
                }

                // insert maximum is IN existing segment
                if *existing_min <= insert_max && insert_max <= *existing_max {
                    upper_pos = Some(2 * i + 1);
                }

                // insert maximum is after the existing segment
                // we need to continue looking, unless this is the last segment
                if insert_max > *existing_max && i == self.ranges.len() - 1 {
                    upper_pos = Some(2 * i + 2);
                }
            }
        }

        // println!("{}, {}", lower_pos.unwrap(), upper_pos.unwrap());

        let lower_pos = lower_pos.unwrap();
        let upper_pos = upper_pos.unwrap();

        let new_min = if lower_pos % 2 == 1 {
            self.ranges[(lower_pos - 1) / 2].0
        } else {
            insert_min
        };

        let new_max = if upper_pos % 2 == 1 {
            self.ranges[(upper_pos - 1) / 2].1
        } else {
            insert_max
        };

        // where to start removing existing ranges
        let lower_remove_idx = if lower_pos % 2 == 1 {
            (lower_pos - 1) / 2
        } else {
            lower_pos / 2
        };

        // count the number of odd #s in the range [lower_pos, upper_pos]
        let num_ranges_to_remove = if lower_pos % 2 == 0 {
            (upper_pos - lower_pos + 1) / 2
        } else {
            (upper_pos - lower_pos + 1).div_ceil(2)
        };

        // if we're not removing any ranges, we're just inserting a range at the index
        if num_ranges_to_remove == 0 {
            self.ranges.insert(lower_remove_idx, (new_min, new_max));
        } else {
            // otherwise, remove all required ranges and insert the new one
            self.ranges.splice(
                lower_remove_idx..=(lower_remove_idx + num_ranges_to_remove - 1),
                vec![(new_min, new_max)].into_iter(),
            );
        }
    }
}
