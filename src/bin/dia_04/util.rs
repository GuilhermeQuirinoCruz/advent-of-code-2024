pub fn get_directions_vec() -> Vec<(i32, i32)>{
  return vec![
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1)
  ];
}

pub fn get_letters_matrix(input: &str) -> Vec<Vec<char>>{
  return input.lines().into_iter()
    .map(|l| l.chars().into_iter().collect())
    .collect();
}

pub fn search_letters_in_matrix(
  letters_matrix: &Vec<Vec<char>>,
  letters_to_search: &Vec<char>,
  starting_position: (i32, i32),
  direction: (i32, i32)) -> bool {
  let mut current_letter_index: usize = 0;
  
  let mut x: i32 = starting_position.0 as i32;
  let mut y: i32 = starting_position.1 as i32;

  while current_letter_index < letters_to_search.len() {
    if y < 0 || y >= letters_matrix.len() as i32 {
      return false;
    }
    let line = &letters_matrix[y as usize];

    if x < 0 || x >= line.len() as i32 {
      return false;
    }
    
    if line[x as usize] != letters_to_search[current_letter_index] {
      return false;
    }
    
    current_letter_index += 1;

    x += direction.0;
    y += direction.1;
  }

  return true;
}