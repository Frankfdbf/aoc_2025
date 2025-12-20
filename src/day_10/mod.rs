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
        .filter_map(|machine| {
            let all_combinations = create_all_button_combinations(&machine.buttons);
            all_combinations
                .get(&machine.indicator_lights)
                .map(|combinations| {
                    combinations
                        .iter()
                        .map(|combination| combination.len())
                        .min()
                })?
        })
        .sum()
}

fn solve_part_two(file_content: &str) -> usize {
    let machines = parse_input(file_content);

    machines
        .iter()
        .filter_map(|machine| {
            let all_combinations = create_all_button_combinations(&machine.buttons);
            minimum_button_presses_to_match_joltage(
                &machine.joltage_requirements,
                &all_combinations,
            )
        })
        .sum()
}

fn minimum_button_presses_to_match_joltage(
    joltage: &Joltage,
    all_combinations: &Combinations,
) -> Option<usize> {
    if joltage.iter().all(|num| *num == 0) {
        return Some(0);
    }

    let pattern = joltage
        .iter()
        .enumerate()
        .filter_map(|(idx, counter)| {
            if counter % 2 == 0 {
                None
            } else {
                Some(2u32.pow(idx as u32) as usize)
            }
        })
        .sum::<usize>();

    if let Some(combinations) = all_combinations.get(&pattern) {
        combinations
            .iter()
            .filter_map(|combination| {
                // copy joltage
                let mut new_joltage = joltage.clone();

                // reduce requirements by pressing buttons
                for button in combination.iter() {
                    for switch in button.as_switches().iter() {
                        if new_joltage[*switch] == 0 {
                            return None;
                        }
                        new_joltage[*switch] -= 1;
                    }
                }

                // half the joltage requirement
                new_joltage.iter_mut().for_each(|element| {
                    *element /= 2;
                });

                Some(
                    combination.len()
                        + 2 * minimum_button_presses_to_match_joltage(
                            &new_joltage,
                            all_combinations,
                        )?,
                )
            })
            .min()
    } else {
        None
    }
}

fn create_all_button_combinations(buttons: &[Button]) -> Combinations {
    let pattern: Pattern = 0; // starting point no button pressed
    let mut combinations = HashMap::new();

    add_combinations(pattern, buttons, Vec::new(), &mut combinations);
    combinations
}

fn add_combinations(
    pattern: Pattern,
    buttons: &[Button],
    buttons_pressed: Buttons,
    combinations: &mut Combinations,
) {
    if buttons.is_empty() {
        let entry = combinations.entry(pattern).or_default();
        entry.push(buttons_pressed);
        return;
    }

    // pressed
    let mut new_button_pressed = buttons_pressed.clone();
    new_button_pressed.push(buttons[0].clone());

    add_combinations(
        pattern ^ buttons[0].as_bits(),
        &buttons[1..],
        new_button_pressed,
        combinations,
    );
    // not pressed
    add_combinations(
        pattern,
        &buttons[1..],
        buttons_pressed.clone(),
        combinations,
    );
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

type Pattern = usize;
type Buttons = Vec<Button>;
type Joltage = Vec<usize>;
type Combinations = HashMap<Pattern, Vec<Buttons>>;

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
