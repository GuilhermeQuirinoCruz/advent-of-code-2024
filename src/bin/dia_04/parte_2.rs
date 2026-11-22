use crate::util;

fn is_position_within_limits(postition: (i32, i32), limits: (i32, i32)) -> bool {
  return postition.0 >= 0 && postition.0 < limits.0 &&
    postition.1 >= 0 && postition.1 < limits.1;
}

fn search_mas_or_sam(
  letters_matrix: &Vec<Vec<char>>,
  starting_position: (i32, i32),
  direction: (i32, i32)
) -> bool {
  let mas_letters: Vec<char> = vec!['M', 'A', 'S'];
  let sam_letters: Vec<char> = vec!['S', 'A', 'M'];
  
  return util::search_letters_in_matrix(
    &letters_matrix,
    &sam_letters,
    starting_position,
    direction) || 
  util::search_letters_in_matrix(
    &letters_matrix,
    &mas_letters,
    starting_position,
    direction)
}

pub fn solve(input: &str) {
  let letters_matrix: Vec<Vec<char>> = util::get_letters_matrix(input);
  let mut x_mas_count: u32 = 0;
  
  for y in 0..letters_matrix.len() {
    for x in 0..letters_matrix[y].len() {
      if letters_matrix[y][x] != 'A' {
        continue;
      }

      println!("Found 'A' at {},{}", x, y);

      let limits: (i32, i32) = (letters_matrix[y].len() as i32, letters_matrix.len() as i32);
      let top_left: (i32, i32) = ((x as i32) - 1, (y as i32) - 1);
      let top_right: (i32, i32) = ((x as i32) + 1, (y as i32) - 1);

      if !is_position_within_limits(top_left, limits)
        || !is_position_within_limits(top_right, limits) {
        println!("Out of bounds: TL {},{}, TR {},{}\n", top_left.0, top_left.1, top_right.0, top_right.1);
        continue;
      }

      println!("Testing TL {},{}", top_left.0, top_left.1);
      if search_mas_or_sam(&letters_matrix, top_left, (1, 1)) {
        println!("Found SAM/MAS at {},{} - TL", x, y);
      } else {
        println!("No X-MAS found at {},{} - TL", x, y);
      }
      
      println!("Testing TR {},{}", top_right.0, top_right.1);
      if search_mas_or_sam(&letters_matrix, top_right, (-1, 1)) {
        println!("Found SAM/MAS at {},{} - TR", x, y);
      } else {
        println!("No X-MAS found at {},{} - TR", x, y);
      }

      if search_mas_or_sam(&letters_matrix, top_left, (1, 1)) &&
        search_mas_or_sam(&letters_matrix, top_right, (-1, 1)) {
          println!("X-MAS at {},{}", x, y);
          x_mas_count += 1;
      }

      println!();
    }
  }
  
  println!("{}", x_mas_count);
}
