use std::collections::HashMap;
use std::{fs, path::Path, time::Instant};

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
    let machines = parse_input(file_content);

    machines
        .iter()
        .filter_map(|machine| minimum_button_presses(machine.indicator_lights, &machine.buttons, 0))
        .sum()
}

fn solve_part_two(file_content: &str) -> usize {
    0
}

type Pattern = usize;
type Buttons = Vec<Button>;
type Joltage = Vec<usize>;

fn minimum_button_presses(
    objective: Pattern,
    buttons: &[Button],
    button_pressed: usize,
) -> Option<usize> {
    if objective == 0 {
        return Some(button_pressed);
    }

    if buttons.is_empty() {
        return None;
    }

    let pressed = minimum_button_presses(
        objective ^ buttons[0].as_bits(),
        &buttons[1..],
        button_pressed + 1,
    );
    let not_pressed = minimum_button_presses(objective, &buttons[1..], button_pressed);

    match (pressed, not_pressed) {
        (None, None) => None,
        (Some(x), Some(y)) => Some(std::cmp::min(x, y)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
    }
}

fn parse_input(file_content: &str) -> Vec<Machine> {
    file_content
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let indicator_lights = parts
                .next()
                .unwrap()
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .chars()
                .enumerate()
                .filter_map(|(idx, char)| {
                    if char == '#' {
                        Some(2u32.pow(idx as u32) as usize)
                    } else {
                        None
                    }
                })
                .sum();

            let joltage_requirements = parts
                .next_back()
                .unwrap()
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect();

            let buttons = parts
                .map(|button| {
                    let mut switches = Vec::new();
                    button
                        .strip_prefix("(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .split(",")
                        .for_each(|num| {
                            switches.push(num.parse().unwrap());
                        });
                    Button::new(switches)
                })
                .collect();

            Machine {
                indicator_lights,
                buttons,
                joltage_requirements,
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Machine {
    pub indicator_lights: Pattern,
    pub buttons: Buttons,
    pub joltage_requirements: Joltage,
}

#[derive(Debug, Clone)]
struct Button {
    switches: Vec<usize>,
    bits: usize,
}

impl Button {
    pub fn new(switches: Vec<usize>) -> Self {
        let bits = switches
            .iter()
            .map(|switch| 2u32.pow(*switch as u32) as usize)
            .sum();
        Self { switches, bits }
    }
    pub fn as_switches(&self) -> &Vec<usize> {
        &self.switches
    }

    pub fn as_bits(&self) -> usize {
        self.bits
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );

        assert_eq!(result, 7);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );

        assert_eq!(result, 33);
    }
}
