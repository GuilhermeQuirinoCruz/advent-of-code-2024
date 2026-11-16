use crate::util;

pub fn solve(input: &str) {
  let (numbers_left, numbers_right) = util::get_sorted_numbers_lists(input.split("\n"));

  let mut distance: u32 = 0;
  for i in 0..numbers_left.len() {
    distance += numbers_left[i].abs_diff(numbers_right[i]);
  }

  // println!("Left numbers: {:?}", numbers_left);
  // println!("Right numbers: {:?}", numbers_right);
  println!("Distance: {}", distance);
}
