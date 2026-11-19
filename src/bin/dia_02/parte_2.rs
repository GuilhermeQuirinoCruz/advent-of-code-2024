use crate::util;

fn filter_report_list_index(report_list: &Vec<u32>, index: u32) -> Vec<u32> {
  return report_list
    .iter()
    .enumerate()
    .filter(|&(i, _)| i != index as usize)
    .map(|(_, e)| *e)
    .collect();
}

pub fn solve(input: &str) {
  let report_lists: Vec<Vec<u32>> = util::get_report_lists(input);
  let mut safe_report_count: u32 = 0;
  
  for i in 0..report_lists.iter().len() {
    let base_list: Vec<u32> = report_lists.get(i).unwrap().clone();
    let mut report_list: Vec<u32> = base_list.clone();
    println!("Checking: {:?}", report_list);
    let report_count: u32 = report_list.len() as u32;

    for report_index in 0..=report_count {
      if util::is_report_list_safe(&report_list) {
        println!("Safe: {:?}", report_list);
        safe_report_count += 1;
        break;
      }

      if report_index == report_count {
        println!("Unsafe");
        break;
      }
      
      report_list = filter_report_list_index(&base_list, report_index);
      println!("Retrying: {:?}", report_list);
    }
  }

  println!("Safe reports: {}", safe_report_count);
}
