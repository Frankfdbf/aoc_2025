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

fn solve_part_one(file_content: &str) -> usize {
    let width = file_content.trim().lines().next().unwrap().len();

    let mut beams: Vec<bool> = vec![false; width];

    file_content
        .trim()
        .lines()
        .map(|line| {
            // dbg!(&line);
            // dbg!(&beams);
            let mut new_beams = beams.clone();
            let splits = line
                .chars()
                .enumerate()
                .map(|(idx, element)| {
                    let mut splits = 0;
                    if element == 'S' {
                        new_beams[idx] = true;
                    } else if element == '^' && beams[idx] == true {
                        splits += 1;
                        new_beams[idx - 1] = true;
                        new_beams[idx + 1] = true;
                        new_beams[idx] = false;
                    }
                    splits
                })
                .sum::<usize>();
            beams = new_beams;
            // dbg!(splits);
            splits
        })
        .sum()
}

fn solve_part_two(file_content: &String) -> usize {
    let width = file_content.trim().lines().next().unwrap().len();

    let mut splits: Vec<usize> = vec![1; width];

    let lines = file_content.trim().lines().rev();

    for line in lines {
        // dbg!(&line);
        // dbg!(&splits);

        let mut new_splits = splits.clone();

        for (idx, element) in line.chars().enumerate() {
            if element == '^' {
                new_splits[idx] = splits[idx - 1] + splits[idx + 1];
            }
        }
        // dbg!(&new_splits);
        splits = new_splits;
    }
    splits[width / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            &".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
                .to_string(),
        );

        assert_eq!(result, 21);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            &".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
                .to_string(),
        );

        assert_eq!(result, 40);
    }
}
