use regex::{Captures, Regex};

fn get_number_from_instruction(instruction: &Captures<'_>, identifier: &str) -> u32 {
  return instruction.name(identifier).unwrap().as_str().parse::<u32>().unwrap();
}

pub fn get_instructions_sum(instructions: &str) -> u32 {
  let re = Regex::new(r"mul\((?<l>\d{1,3}),(?<r>\d{1,3})\)").unwrap();
  let mut sum: u32 = 0;
  
  for instruction in re.captures_iter(instructions) {
    let left_number: u32 = get_number_from_instruction(&instruction, "l");
    let right_number: u32 = get_number_from_instruction(&instruction, "r");
    let result: u32 = left_number * right_number;
    sum += result;
    
    println!("{} * {} = {}", left_number, right_number, result);
  }

  return sum;
}