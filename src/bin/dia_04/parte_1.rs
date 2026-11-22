use crate::util;
use std::collections::HashMap;

fn search_xmas_hash(
  letters_map: &HashMap<(i32, i32), char>,
  starting_position: (i32, i32),
  direction: (i32, i32)) -> bool {
  let letters_to_search: Vec<char> = vec!['X', 'M', 'A', 'S'];
  let mut current_letter_index: usize = 0;
  
  let mut x: i32 = starting_position.0;
  let mut y: i32 = starting_position.1;

  while current_letter_index < letters_to_search.len() {
    match letters_map.get(&(x, y)) {
        Some(letter) => {
          if letter != &letters_to_search[current_letter_index] {
            return false;
          }

          current_letter_index += 1;
          x += direction.0;
          y += direction.1;
        },
        None => return false,
    }
  }

  return true;
}

fn solve_hash(input: &str) {
  let mut letters_map: HashMap<(i32, i32), char> = HashMap::new();
  
  let mut x: i32 = 0;
  let mut y: i32 = 0;
  for line in input.lines() {
    for letter in line.chars() {
      letters_map.insert((x, y), letter);
      x += 1;
    }
    
    x = 0;
    y += 1;
  }

  let directions: Vec<(i32, i32)> = util::get_directions_vec();

  let mut xmas_count: u32 = 0;
  while y >= 0 {
    x = 0;
    while letters_map.contains_key(&(x, y)) {
      for direction in &directions {
        if search_xmas_hash(&letters_map, (x, y), *direction) {
          xmas_count += 1;
        }
      }
      
      x += 1;
    }
    
    y -= 1;
  }

  println!("{}", xmas_count);
}

fn solve_matrix(input: &str) {
  let letters_matrix: Vec<Vec<char>> = util::get_letters_matrix(input);
  let directions: Vec<(i32, i32)> = util::get_directions_vec();
  let xmas_letters: Vec<char> = vec!['X', 'M', 'A', 'S'];
  let mut xmas_count: u32 = 0;
  
  for y in 0..letters_matrix.len() {
    for x in 0..letters_matrix[y].len() {
      for direction in &directions {
        if util::search_letters_in_matrix(
          &letters_matrix,
          &xmas_letters,
          (x as i32, y as i32),
          *direction) {
          xmas_count += 1;
        }
      }
    }
  }

  println!("{}", xmas_count);
}

pub fn solve(input: &str) {
  solve_matrix(input);
  solve_hash(input);
}
