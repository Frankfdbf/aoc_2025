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
    compute_max_voltage(&file_content, 2)
}

fn solve_part_two(file_content: &String) -> usize {
    compute_max_voltage(&file_content, 12)
}

fn compute_max_voltage(battery_banks: &str, slots_per_bank: usize) -> usize {
    battery_banks
        .trim()
        .split('\n')
        .map(|line| {
            let mut voltage = String::new();

            let mut bank = line;

            for i in (0..slots_per_bank).rev() {
                let (position, battery) = find_max_battery(&bank[..(bank.len() - i)]);
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
