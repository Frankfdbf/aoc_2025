use std::fs;
use std::ops::RangeInclusive;
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

fn solve_part_one(file_content: &str) -> usize {
    let mut db = file_content.trim().split("\n\n");
    let db_fresh_ingredients = db.next().unwrap();
    let db_available_ingredients = db.next().unwrap();

    let fresh_ingredients: Vec<RangeInclusive<usize>> = db_fresh_ingredients
        .lines()
        .map(|range| {
            let mut bounds = range.split('-');
            let lower_bound = bounds.next().unwrap().parse::<usize>().unwrap();
            let upper_bound = bounds.next().unwrap().parse::<usize>().unwrap();
            lower_bound..=upper_bound
        })
        .collect();

    db_available_ingredients
        .lines()
        .filter(|id| {
            fresh_ingredients
                .iter()
                .any(|range| range.contains(&id.parse().unwrap()))
        })
        .count()
}

fn solve_part_two(file_content: &str) -> usize {
    let mut fresh_ingredients: Vec<RangeInclusive<usize>> = file_content
        .trim()
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|range| {
            let mut bounds = range.split('-');
            let lower_bound = bounds.next().unwrap().parse::<usize>().unwrap();
            let upper_bound = bounds.next().unwrap().parse::<usize>().unwrap();
            lower_bound..=upper_bound
        })
        .collect();

    fresh_ingredients.sort_by(|a, b| a.start().cmp(b.start()));

    let mut last_high = 0;
    let mut total = 0;
    for ingredient in fresh_ingredients.iter() {
        if ingredient.end() <= &last_high {
            continue;
        }
        if ingredient.start() <= &last_high {
            total -= last_high - ingredient.start() + 1;
        }

        total += ingredient.end() - ingredient.start() + 1;
        last_high = *ingredient.end();
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );

        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );

        assert_eq!(result, 14);
    }
}
