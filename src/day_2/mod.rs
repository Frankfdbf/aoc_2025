use std::fs;
use std::path::Path;

pub fn output_single_star(path: &Path) {
    let file_content = fs::read_to_string(path).unwrap();
    let result = solve_part_one(&file_content);
    println!("Part 1, from input {path:?}, resulted in {result}");
}

pub fn output_double_star(path: &Path) {
    let file_content = fs::read_to_string(path).unwrap();
    let result = solve_part_two(&file_content);
    println!("Part 2, from input {path:?}, resulted in {result}");
}

fn solve_part_one(file_content: &String) -> usize {
    file_content
        .split(',')
        .map(|range| {
            let mut total: usize = 0;

            let mut bounds = range.trim().split('-');
            let lower_bound: usize = bounds.next().unwrap().parse().unwrap();
            let upper_bound: usize = bounds.next().unwrap().parse().unwrap();

            for id in lower_bound..=upper_bound {
                let id_as_str = id.to_string();

                // Odd number of chars, not possible that id is made of
                // pattern repeated twice.
                if id_as_str.len() % 2 != 0 {
                    continue;
                }

                let middle = id_as_str.len() / 2;

                if id_as_str[..middle] == id_as_str[middle..] {
                    total += id;
                }
            }
            total
        })
        .sum()
}

fn solve_part_two(file_content: &String) -> usize {
    file_content
        .split(',')
        .map(|range| {
            let mut total: usize = 0;

            let mut bounds = range.trim().split('-');
            let lower_bound: usize = bounds.next().unwrap().parse().unwrap();
            let upper_bound: usize = bounds.next().unwrap().parse().unwrap();

            for id in lower_bound..=upper_bound {
                let id_as_str = id.to_string();

                // Test all possible pattern lengths, from 1 to n/2 + 1, where n
                // is the number of digits in the id.
                // Patterns longer than n/2 + 1 cannoted be repeated.
                for pattern_length in (1..=(id_as_str.len() / 2)).rev() {
                    if is_repeating_pattern(&id_as_str, pattern_length) {
                        total += id;
                        break;
                    }
                }
            }
            total
        })
        .sum()
}

fn is_repeating_pattern(id: &str, pattern_length: usize) -> bool {
    if id.len() % pattern_length != 0 {
        return false;
    }

    let pattern = &id[0..pattern_length];

    for idx in (pattern_length..id.len()).step_by(pattern_length) {
        if &id[idx..idx + pattern_length] != pattern {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            &"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            .to_string(),
        );

        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            &"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            .to_string(),
        );

        assert_eq!(result, 4174379265);
    }
}
