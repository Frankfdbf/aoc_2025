use std::fs;
use std::path::Path;
use std::time::Instant;

pub fn output_single_star(path: &Path) {
    let start = Instant::now();
    let file_content = fs::read_to_string(path).unwrap();
    let result = solve_part_one(&file_content);
    let duration = start.elapsed();
    println!("Part 1, from input {path:?}, resulted in {result}, took {duration:?}");
}

pub fn output_double_star(path: &Path) {
    let start = Instant::now();
    let file_content = fs::read_to_string(path).unwrap();
    let result = solve_part_two(&file_content);
    let duration = start.elapsed();
    println!("Part 2, from input {path:?}, resulted in {result}, took {duration:?}");
}

const STARTING_POSITION: i64 = 50;

fn solve_part_one(file_content: &str) -> usize {
    let mut counter = 0;
    let mut current_position = STARTING_POSITION;
    for line in file_content.lines() {
        let direction = &line[..1].to_string();
        let increment: i64 = line[1..].parse().unwrap();

        if direction == &String::from("R") {
            current_position += increment;
        } else {
            current_position -= increment;
        }
        current_position = current_position.rem_euclid(100);
        if current_position == 0 {
            counter += 1;
        }
    }
    counter
}

fn solve_part_two(file_content: &str) -> usize {
    let mut counter = 0;
    let mut current_position = STARTING_POSITION;
    for line in file_content.lines() {
        let direction = &line[..1].to_string();
        let mut increment: i64 = line[1..].parse().unwrap();
        let initial_position = current_position;

        counter += increment / 100;
        increment %= 100;

        if direction == &String::from("R") {
            current_position += increment;
        } else {
            current_position -= increment;
        }

        if initial_position != 0 && (current_position <= 0 || current_position >= 100) {
            counter += 1;
        }
        current_position = current_position.rem_euclid(100);
    }
    counter as usize
}

#[cfg(test)]
mod test {
    use crate::day_1::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );

        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        );

        assert_eq!(result, 6);
    }
}
