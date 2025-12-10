use std::collections::HashSet;
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

fn solve_part_one(file_content: &str) -> usize {
    let edges: Vec<(i64, i64)> = file_content
        .lines()
        .map(|line| {
            let mut coordinates = line.split(",");
            (
                coordinates.next().unwrap().parse().unwrap(),
                coordinates.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut max_area = 0;

    for (idx, (i, j)) in edges.iter().enumerate() {
        for (x, y) in edges.iter() {
            let area = (x - i + 1) * (y - j + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area as usize
}

fn solve_part_two(file_content: &String) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            &"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            .to_string(),
        );

        assert_eq!(result, 50);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            &"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            .to_string(),
        );

        assert_eq!(result, 24);
    }
}
