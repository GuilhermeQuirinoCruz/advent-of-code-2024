use crate::util;

pub fn solve(input: &str) {
  const DO_INSTRUCTION_OFFSET: usize = 4;
  let mut sum: u32 = 0;
  
  for part in input.split("don't()") {
    match part.find("do()") {
        Some(value) => {
          println!("{}", part.split_at(value + DO_INSTRUCTION_OFFSET).1);
          sum += util::get_instructions_sum(part.split_at(value + DO_INSTRUCTION_OFFSET).1);
        },
        None => {
          // Edge case: "At the beginning of the program, mul instructions are enabled."
          if sum == 0 {
            println!("{}", part);
            sum += util::get_instructions_sum(part);
          }
        }
    }
  }

  println!("Sum: {}", sum);
}
