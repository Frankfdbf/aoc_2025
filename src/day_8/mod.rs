use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use std::{fs, usize};

pub fn output_single_star(path: &Path) {
    let start = Instant::now();
    let file_content = fs::read_to_string(path).unwrap();
    let result = solve_part_one(&file_content, 1000);
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

fn solve_part_one(file_content: &str, iterations: usize) -> usize {
    let vertices: Vec<JunctionBox> = get_vertices(file_content);
    let edges: Vec<(usize, usize, usize)> = get_edges(&vertices);

    let mut distances: Vec<Vec<usize>> = vec![vec![usize::MAX; vertices.len()]; vertices.len()];

    for (i, vertex) in vertices.iter().enumerate() {
        for (y, other_vertex) in vertices[i + 1..].iter().enumerate() {
            distances[i][i + 1 + y] = vertex.squared_distance_to(&other_vertex);
        }
    }
    let mut circuits = vec![None; vertices.len()];
    let mut next_group = 0;

    for (idx, (x, y, _)) in edges.into_iter().enumerate() {
        if idx == iterations {
            break;
        }

        // get the circuit associated
        match (circuits.get(x).unwrap(), circuits.get(y).unwrap()) {
            (None, None) => {
                circuits[x] = Some(next_group);
                circuits[y] = Some(next_group);
                next_group += 1;
            }
            (Some(circuit), None) => {
                circuits[y] = Some(*circuit);
            }
            (None, Some(circuit)) => {
                circuits[x] = Some(*circuit);
            }
            (Some(circuit_x), Some(circuit_y)) => {
                if circuit_x != circuit_y {
                    circuits = circuits
                        .clone()
                        .into_iter()
                        .map(|x| {
                            if x == Some(*circuit_x) {
                                Some(*circuit_y)
                            } else {
                                x
                            }
                        })
                        .collect();
                }
            }
        };
    }

    let mut frequencies: Vec<usize> = circuits
        .iter()
        .copied()
        .filter_map(|x| x)
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
            map
        })
        .values()
        .copied()
        .collect();

    frequencies.sort_by(|a, b| b.cmp(a));
    frequencies[..3].iter().product()
}

fn solve_part_two(file_content: &str) -> usize {
    let vertices: Vec<JunctionBox> = get_vertices(file_content);
    let edges: Vec<(usize, usize, usize)> = get_edges(&vertices);
    let mut union_find = UnionFind::new(vertices.len());

    for (x, y, _) in edges.into_iter() {
        union_find.union(x, y);

        if union_find.circuit_count == 1 && union_find.parents.iter().all(|x| x.is_some()) {
            return vertices[x].x * vertices[y].x;
        }
    }
    0
}

fn get_vertices(file_content: &str) -> Vec<JunctionBox> {
    file_content
        .trim()
        .lines()
        .map(|line| {
            let mut coordinates = line.split(',');
            let x = coordinates.next().unwrap().parse().unwrap();
            let y = coordinates.next().unwrap().parse().unwrap();
            let z = coordinates.next().unwrap().parse().unwrap();
            JunctionBox { x, y, z }
        })
        .collect()
}

fn get_edges(vertices: &Vec<JunctionBox>) -> Vec<(usize, usize, usize)> {
    let mut edges = Vec::new();

    for (i, vertex) in vertices.iter().enumerate() {
        for (j, other_vertex) in vertices[i + 1..].iter().enumerate() {
            edges.push((i, i + 1 + j, vertex.squared_distance_to(&other_vertex)));
        }
    }
    edges.sort_by_key(|&(_, _, distance)| distance);
    edges
}

// fn mst(vertices: &Vec<JunctionBox>) -> &UnionFind {

// }

struct UnionFind {
    pub parents: Vec<Option<usize>>,
    pub circuit_count: usize,
}

impl UnionFind {
    pub fn new(capacity: usize) -> Self {
        Self { parents: vec![None; capacity], circuit_count: 0 }
    }

    fn root(&self, mut from: usize) -> Option<usize> {
        let root = None;
        while let Some(parent) = self.parents[from] {
            if parent == from {
                return Some(parent);
            } else {
                from = parent;
            }
        }
        root
    }

    pub fn union(&mut self, from: usize, to: usize) {
        // check out of bound
        match (self.parents[from], self.parents[to]) {
            (None, None) => {
                self.parents[from] = Some(from);
                self.parents[to] = Some(from);
                self.circuit_count += 1;
            }, 
            (Some(_), Some(_)) => {

                match (self.root(from), self.root(to)) {
                    (Some(x), Some(y)) => {
                        if x == y { return; }
                        self.parents[x] = Some(to);
                        self.circuit_count -= 1;

                    },
                    _ => { panic!("should never happen"); },
                }
            }, 
            (Some(_), None) => {
                self.parents[to] = Some(from);
            },
            (None, Some(_)) => {
                self.parents[from] = Some(to);
            },
        }

    }
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
        (self.x as i64 - other.x as i64).pow(2) as usize
            + (self.y as i64 - other.y as i64).pow(2) as usize
            + (self.z as i64 - other.z as i64).pow(2) as usize
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
            10,
        );

        assert_eq!(result, 40);
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

        assert_eq!(result, 25272);
    }
}
