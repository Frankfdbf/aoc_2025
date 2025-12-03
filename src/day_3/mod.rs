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
    file_content
        .trim()
        .split('\n')
        .map(|line| {
            let first_max = line[..(line.len() - 1)]
                .chars()
                .max_by_key(|battery| *battery)
                .unwrap();

            let pos_first_max = line
                .chars()
                .position(|battery| battery == first_max)
                .unwrap();

            dbg!(line);
            dbg!(pos_first_max);
            dbg!(first_max);

            let second_max = line[(pos_first_max + 1)..]
                .chars()
                .max_by_key(|battery| *battery)
                .unwrap();

            dbg!(second_max);

            let power = format!("{first_max}{second_max}");

            dbg!(power).parse::<usize>().unwrap()
        })
        .sum()
}

fn solve_part_two(file_content: &String) -> usize {
    const ITERATIONS: usize = 12;
    file_content
        .trim()
        .split('\n')
        .map(|line| {
            let mut voltage = String::new();

            let mut bank = line;

            for i in (0..ITERATIONS).rev() {
                let battery;
                let position;

                // if i == ITERATIONS {
                //    (_, battery) = find_max_battery(&bank);
                // } else {
                //    (position, battery) = find_max_battery(&bank[..(bank.len() - 1)]);
                //     bank = &bank[(position + 1)..];
                // }
                (position, battery) = find_max_battery(&bank[..(bank.len() - i)]);
                bank = &bank[(position + 1)..];

                voltage.push(battery);
            }
            voltage.parse::<usize>().unwrap()
        })
        .sum()
}

fn find_max_battery(bank: &str) -> (usize, char) {
    let max = bank.chars().max_by_key(|battery| *battery).unwrap();

    let pos_max = bank.chars().position(|battery| battery == max).unwrap();

    (pos_max, max)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            &"987654321111111
811111111111119
234234234234278
818181911112111"
                .to_string(),
        );

        assert_eq!(result, 357);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            &"987654321111111
811111111111119
234234234234278
818181911112111"
                .to_string(),
        );

        assert_eq!(result, 3121910778619);
    }
}
