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

fn solve_part_one(file_content: &str) -> usize {
    let (shapes, regions) = parse_input(file_content);
    let status: Vec<Status> = regions
        .iter()
        .map(|region| {
            if region.height / 3 * region.width / 3 >= region.shapes_to_fit.iter().sum() {
                return Status::CanFit;
            } else if region.height * region.width
                < shapes
                    .iter()
                    .enumerate()
                    .map(|(idx, shape)| {
                        region.shapes_to_fit[idx]
                            * shape.iter().map(|row| row.iter().count()).sum::<usize>()
                    })
                    .sum()
            {
                return Status::CannotFit;
            } else {
                return Status::Undertermined;
            }
        })
        .collect::<Vec<Status>>();

    assert_eq!(
        status
            .iter()
            .filter(|&s| *s == Status::Undertermined)
            .count(),
        0
    );

    status.iter().filter(|&s| *s == Status::CanFit).count()
}

fn parse_input(file_content: &str) -> (Shapes, Regions) {
    let mut sections = file_content.trim().split("\n\n");
    let mut shapes = Shapes::new();

    for _ in 0..6 {
        let mut lines = sections.next().unwrap().lines();
        lines.next().unwrap(); // skip index

        let shape: Shape = lines
            .map(|line| {
                line.chars()
                    .map(|c| if c == '#' { true } else { false })
                    .collect()
            })
            .collect();
        shapes.push(shape);
    }

    let regions: Regions = sections
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut info = line.split(':');
            let size: Vec<usize> = info
                .next()
                .unwrap()
                .trim()
                .split('x')
                .map(|s| s.parse().unwrap())
                .collect();
            let shapes_to_fit: Vec<usize> = info
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|c| c.parse().unwrap())
                .collect();
            Region {
                width: size[0],
                height: size[1],
                shapes_to_fit,
            }
        })
        .collect();

    (shapes, regions)
}

type Shape = Vec<Vec<bool>>;
type Shapes = Vec<Shape>;

type Regions = Vec<Region>;
#[derive(Debug, Clone)]
struct Region {
    pub width: usize,
    pub height: usize,
    pub shapes_to_fit: Vec<usize>,
}

#[derive(PartialEq, Eq)]
enum Status {
    CanFit,
    CannotFit,
    Undertermined,
}
