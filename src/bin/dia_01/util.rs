use std::str::Split;

pub fn get_numbers_from_line(line: &str) -> (u32, u32) {
  let mut numbers = line.split_whitespace();
  return (numbers.next().unwrap().parse().unwrap(), numbers.next().unwrap().parse().unwrap());
}

pub fn get_sorted_numbers_lists(lines: Split<&str>) -> (Vec<u32>, Vec<u32>) {
  let mut numbers_left: Vec<u32> = vec![];
  let mut numbers_right: Vec<u32> = vec![];

  for line in lines {
    let (left_number, right_number) = get_numbers_from_line(line);

    numbers_left.push(left_number);
    numbers_right.push(right_number);
  }

  numbers_left.sort();
  numbers_right.sort();

  return (numbers_left, numbers_right);
}