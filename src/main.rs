use std::path::Path;

mod day_1;
mod day_2;
mod day_3;

fn main() {
    day_1::output_single_star(Path::new("src/day_1/input.txt"));
    day_1::output_double_star(Path::new("src/day_1/input.txt"));
    day_2::output_single_star(Path::new("src/day_2/input.txt"));
    day_2::output_double_star(Path::new("src/day_2/input.txt"));
    day_3::output_single_star(Path::new("src/day_3/input.txt"));
    day_3::output_double_star(Path::new("src/day_3/input.txt"));
}
