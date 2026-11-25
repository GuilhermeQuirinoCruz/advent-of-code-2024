use crate::util;

pub fn solve(input: &str) {
  let (page_ordering_rules, pages_to_produce) = util::get_page_data(input);

  let middle_page_numbers_sum = pages_to_produce
    .iter()
    .filter(|page| !util::is_update_in_order(&page_ordering_rules, page))
    .map(|page| util::correct_page(&page_ordering_rules, page))
    .map(|numbers| numbers[numbers.len() / 2])
    .sum::<u32>();

  println!("Sum: {}", middle_page_numbers_sum);
}
