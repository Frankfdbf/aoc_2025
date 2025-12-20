use std::collections::HashMap;
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
    let mut edges: Edges = HashMap::new();

    for line in file_content.trim().lines() {
        let parts: Vec<&str> = line.split(':').collect();
        edges
            .entry(parts[0])
            .insert_entry(parts[1].split_whitespace().collect::<Vec<&str>>());
    }
    dfs(&edges, "you")
}

fn solve_part_two(file_content: &str) -> usize {
    let mut edges: Edges = HashMap::new();

    for line in file_content.trim().lines() {
        let parts: Vec<&str> = line.split(':').collect();
        edges
            .entry(parts[0])
            .insert_entry(parts[1].split_whitespace().collect::<Vec<&str>>());
    }
    let mut cache = Cache::new();
    dfs_2(&edges, "svr", false, false, &mut cache)
}

fn dfs(edges: &Edges, start: &str) -> usize {
    let mut possible_paths = 0;
    for edge in edges.get(start).unwrap().iter() {
        if *edge == "out" {
            possible_paths += 1;
        } else {
            possible_paths += dfs(edges, edge);
        }
    }
    possible_paths
}

fn dfs_2(
    edges: &Edges,
    start: &str,
    visited_dac: bool,
    visited_fft: bool,
    cache: &mut Cache,
) -> usize {
    let key = (start.to_string(), visited_fft, visited_dac);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut possible_paths = 0;
    for edge in edges.get(start).unwrap().iter() {
        if *edge == "out" && visited_dac && visited_fft {
            possible_paths += 1;
        } else if *edge != "out" {
            let mut new_visited_dac = visited_dac;
            let mut new_visited_fft = visited_fft;
            if *edge == "dac" {
                new_visited_dac = true;
            }
            if *edge == "fft" {
                new_visited_fft = true;
            }
            possible_paths += dfs_2(edges, edge, new_visited_dac, new_visited_fft, cache);
        }
    }
    let _ = cache.insert(key, possible_paths);
    possible_paths
}

type Edges<'a> = HashMap<&'a str, Vec<&'a str>>;
type Cache<'a> = HashMap<(String, bool, bool), usize>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        );

        assert_eq!(result, 5);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        );

        assert_eq!(result, 2);
    }
}
