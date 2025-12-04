use std::fs;
use std::path::Path;

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
    let diagram: Matrix<char> = file_content
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width = diagram[0].len();
    let height = diagram.len();

    let mut result = 0;

    for x in 0..width {
        for y in 0..height {
            // check if adjcent position are less than 3 rolls
            if is_roll(x, y, &diagram)
                && forklift_can_access((x as i64, y as i64), &diagram, width, height)
            {
                dbg!((x, y));
                result += 1;
            }
        }
    }
    result
}

fn solve_part_two(file_content: &String) -> usize {
    let mut diagram: Matrix<char> = file_content
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width = diagram[0].len();
    let height = diagram.len();

    let mut removed_rolls = 0;

    loop {
        let mut removed_rolls_in_current_loop = 0;

        for x in 0..width {
            for y in 0..height {
                // check if adjcent position are less than 3 rolls
                if is_roll(x, y, &diagram)
                    && forklift_can_access((x as i64, y as i64), &diagram, width, height)
                {
                    dbg!((x, y));
                    removed_rolls += 1;
                    removed_rolls_in_current_loop += 1;
                    diagram[x][y] = '.';
                }
            }
        }
        if removed_rolls_in_current_loop == 0 {
            break;
        }
    }

    removed_rolls
}

fn forklift_can_access(
    pos: (i64, i64),
    diagram: &Matrix<char>,
    width: usize,
    height: usize,
) -> bool {
    let mut rolls_count = 0;
    let (x, y) = pos;

    let positions_to_check = vec![
        (x - 1, y),
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
    ];

    let positions_to_check: Vec<(i64, i64)> = positions_to_check
        .into_iter()
        .filter(|(x, y)| x >= &0 && x < &(width as i64) && y >= &0 && y < &(height as i64))
        .collect();

    for (x, y) in positions_to_check {
        if is_roll(x as usize, y as usize, diagram) {
            // dbg!((x, y));
            rolls_count += 1;
        }
    }

    rolls_count < 4
}

fn is_roll(x: usize, y: usize, diagram: &Matrix<char>) -> bool {
    if diagram[x][y] == '@' { true } else { false }
}

type Matrix<T> = Vec<Vec<T>>;

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
