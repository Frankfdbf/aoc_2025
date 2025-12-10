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
    let vertices: Vec<Vertex> = file_content
        .lines()
        .map(|line| {
            let mut coordinates = line.split(",");
            Vertex {
                x: coordinates.next().unwrap().parse().unwrap(),
                y: coordinates.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut max_area = 0;

    for (idx, vertex) in vertices[..vertices.len() - 1].iter().enumerate() {
        for other_vertex in vertices[idx..].iter() {
            let area = area(vertex, other_vertex);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn solve_part_two(file_content: &str) -> usize {
    let vertices: Vec<Vertex> = file_content
        .lines()
        .map(|line| {
            let mut coordinates = line.split(",");
            Vertex {
                x: coordinates.next().unwrap().parse().unwrap(),
                y: coordinates.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut edges: Vec<Edge> = vertices
        .windows(2)
        .map(|window| Edge(&window[0], &window[1]))
        .collect();
    // finish loop by adding last edge
    edges.push(Edge(vertices.last().unwrap(), vertices.first().unwrap()));

    let mut max_area = 0;

    for (idx, vertex) in vertices[..vertices.len() - 1].iter().enumerate() {
        for other_vertex in vertices[idx..].iter() {
            let area = area(vertex, other_vertex);
            if area <= max_area {
                continue;
            }

            if !rectangle_intersects_with_edges(vertex, other_vertex, &edges) {
                max_area = area;
            }
        }
    }
    max_area
}

fn area(vertex: &Vertex, other_vertex: &Vertex) -> usize {
    let min_x = vertex.x.min(other_vertex.x);
    let max_x = vertex.x.max(other_vertex.x);
    let min_y = vertex.y.min(other_vertex.y);
    let max_y = vertex.y.max(other_vertex.y);

    (max_x - min_x + 1) * (max_y - min_y + 1)
}

fn rectangle_intersects_with_edges(from: &Vertex, to: &Vertex, edges: &Vec<Edge>) -> bool {
    let rect_min_x = from.x.min(to.x);
    let rect_max_x = from.x.max(to.x);
    let rect_min_y = from.y.min(to.y);
    let rect_max_y = from.y.max(to.y);

    for edge in edges {
        let edge_min_x = edge.0.x.min(edge.1.x);
        let edge_max_x = edge.0.x.max(edge.1.x);
        let edge_min_y = edge.0.y.min(edge.1.y);
        let edge_max_y = edge.0.y.max(edge.1.y);

        if edge_min_x < rect_max_x
            && edge_max_x > rect_min_x
            && edge_min_y < rect_max_y
            && edge_max_y > rect_min_y
        {
            return true;
        }
    }
    false
}

struct Vertex {
    pub x: usize,
    pub y: usize,
}

struct Edge<'a>(pub &'a Vertex, pub &'a Vertex);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let result = solve_part_one(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );

        assert_eq!(result, 50);
    }

    #[test]
    fn test_solve_part_two() {
        let result = solve_part_two(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );

        assert_eq!(result, 24);
    }
}
