use crate::util;
use std::cmp::Ordering;

fn count_equal_numbers(number: u32, number_list: &Vec<u32>) -> u32 {
  return number_list.iter().filter(|n| n == &&number).count() as u32;
}

fn solve_by_filtering(numbers_left: &Vec<u32>, numbers_right: &Vec<u32>) -> u32 {
  let mut similarity_score: u32 = 0;
  for number in numbers_left {
    similarity_score += count_equal_numbers(*number, numbers_right) * number;
  }

  return similarity_score;
}

fn solve_by_iterating(numbers_left: &Vec<u32>, numbers_right: &Vec<u32>) -> u32{
  let mut similarity_score: u32 = 0;

  let mut i: usize = 0;
  let mut i2: usize = 0;
  let list_size: &usize = &numbers_left.len();
  while i < *list_size {
    match &numbers_left[i].cmp(&numbers_right[i2]) {
        Ordering::Less => {
          i2 = *list_size;
        },
        Ordering::Equal => {
          similarity_score += &numbers_left[i];
          i2 += 1;
        },
        Ordering::Greater => {
          i2 += 1;
        },
    }

    if i2 == *list_size {
      i2 = 0;
      i += 1;
    }
  }

  return similarity_score;
}

pub fn solve(input: &str) {
  let (numbers_left, numbers_right) = util::get_sorted_numbers_lists(input.split("\n"));

  println!("Similarity score (filter): {}", solve_by_filtering(&numbers_left, &numbers_right));
  println!("Similarity score (iterative): {}", solve_by_iterating(&numbers_left, &numbers_right));
}
