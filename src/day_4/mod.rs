use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::grid::{Grid, Coordinates};

pub fn output_single_star(path: &Path) {
    let file_content = fs::read_to_string(path).unwrap();
    let result = solve_part_one(&file_content);
    println!("Day three, from input {path:?}, resulted in {result}");
}

pub fn output_double_star(path: &Path) {
    let file_content = fs::read_to_string(path).unwrap();
    let result = solve_part_two(&file_content);
    println!("Day three, from input {path:?}, resulted in {result}");
}

fn solve_part_one(file_content: &str) -> usize {
    let grid: Grid<char> = Grid::from_str(file_content).unwrap();

    let mut movable_rolls = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = Coordinates::new(x, y);
            if grid[pos] != '@' {
                continue;
            }

            let count_adjacent_rolls = grid.neighbors(pos).filter(|&pos| grid[pos] == '@').count();
            if count_adjacent_rolls < 4 {
                movable_rolls += 1;
            }
        }
    }

    movable_rolls
}

fn solve_part_two(file_content: &String) -> usize {
    let mut grid: Grid<char> = Grid::from_str(file_content).unwrap();
    let mut removed_rolls_count = 0;

    loop {
        // let mut removed_rolls_in_current_loop = 0;
        let mut removed_rolls_current_loop = Vec::new();

        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let pos = Coordinates::new(x, y);
                if grid[pos] != '@' {
                    continue;
                }

                let count_adjacent_rolls =
                    grid.neighbors(pos).filter(|&pos| grid[pos] == '@').count();

                if count_adjacent_rolls < 4 {
                    removed_rolls_count += 1;
                    removed_rolls_current_loop.push(pos);
                }
            }
        }

        for pos in removed_rolls_current_loop.iter() {
            grid[*pos] = '.';
        }

        if removed_rolls_current_loop.len() == 0 {
            break;
        }
    }

    removed_rolls_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            &"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .to_string(),
        );

        assert_eq!(result, 13);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            &"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
                .to_string(),
        );

        assert_eq!(result, 43);
    }
}
