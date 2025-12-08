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
    let graph: Vec<JunctionBox> = file_content
        .trim()
        .lines()
        .map(|line| {
            let mut coordinates = line.split(',');
            let x = coordinates.next().unwrap().parse().unwrap();
            let y = coordinates.next().unwrap().parse().unwrap();
            let z = coordinates.next().unwrap().parse().unwrap();
            JunctionBox { x, y, z }
        })
        .collect();

    let edges: Vec<(JunctionBox, JunctionBox)> = graph
        .iter()
        .map(|node| {
            let closest_node = graph
                .iter()
                .filter_map(|other_node| {
                    if other_node == node {
                        return None;
                    }

                    Some((other_node, other_node.squared_distance_to(&node)))
                })
                .min_by(|x, y| x.1.cmp(&y.1))
                .map(|(other_node, distance)| other_node.clone())
                .unwrap();
            (node.clone(), closest_node)
        })
        .collect();
    dbg!(edges);
    0
}

fn solve_part_two(file_content: &String) -> usize {
    0
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    pub fn squared_distance_to(&self, other: &JunctionBox) -> usize {
        // save on the sqrt operation
        (self.x + other.x).pow(2) + (self.y + other.y).pow(2) + (self.z + other.z).pow(2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            &"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
                .to_string(),
        );

        assert_eq!(result, 21);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            &"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
                .to_string(),
        );

        assert_eq!(result, 40);
    }
}
