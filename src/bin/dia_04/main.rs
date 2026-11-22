mod util;
mod parte_1;
mod parte_2;

fn main() {
  // let input: &str = include_str!("./example_1.txt");
  let input: &str = include_str!("./input.txt");

  parte_1::solve(input);
  parte_2::solve(input);
}
