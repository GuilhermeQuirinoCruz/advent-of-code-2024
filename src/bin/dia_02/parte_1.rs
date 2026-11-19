use crate::util;

pub fn solve(input: &str) {
  let report_lists: Vec<Vec<u32>> = util::get_report_lists(input);

  let safe_report_count: usize = report_lists
    .iter()
    .filter(|rl| util::is_report_list_safe(rl))
    .count();

  println!("Safe reports: {}", safe_report_count);
}
