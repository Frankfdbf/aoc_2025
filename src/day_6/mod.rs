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
    let mut lines = file_content.trim().lines().rev();

    let operators: Vec<char> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|op| op.trim().parse().unwrap())
        .collect();

    let operands: Vec<Vec<usize>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.trim().parse().unwrap())
                .collect()
        })
        .collect();

    operators
        .iter()
        .enumerate()
        .map(|(idx, operator)| match &operator {
            '*' => operands.iter().map(|line| line[idx]).product::<usize>(),
            '+' => operands.iter().map(|line| line[idx]).sum::<usize>(),
            _ => todo!("missing operators"),
        })
        .sum()
}

fn solve_part_two(file_content: &String) -> usize {
    let mut lines = file_content.trim().lines().rev();

    let operators: Vec<(usize, &str)> = lines
        .next()
        .unwrap()
        .rmatch_indices(|a| a == '*' || a == '+')
        .collect();

    let len_line = lines.clone().next().unwrap().len();

    let mut last_visited_idx = len_line;

    operators
        .iter()
        .map(|(idx, operator)| {
            let operands: Vec<&str> = lines
                .clone()
                .map(|line| &line[*idx..last_visited_idx])
                .collect();

            last_visited_idx = idx.saturating_sub(1);

            let result = apply_operator_on_operand(&operands, operator);
            result
        })
        .sum()
}

fn apply_operator_on_operand(operands: &Vec<&str>, operator: &str) -> usize {
    let max_length = operands[0].len();

    let computations = (0..max_length).map(|idx| {
        let mut num_str = String::with_capacity(max_length);
        for operand in operands.clone().iter().rev() {
            num_str.push_str(&operand[idx..idx + 1]);
        }

        num_str.trim().parse::<usize>().unwrap()
    });
    match operator {
        "+" => computations.sum(),
        "*" => computations.product(),
        _ => todo!("missing operators, implement them"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            &"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
                .to_string(),
        );

        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            &"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
                .to_string(),
        );

        assert_eq!(result, 3263827);
    }
}
