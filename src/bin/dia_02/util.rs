pub fn get_report_lists(input: &str) -> Vec<Vec<u32>>{
  return input
    .lines()
    .map(|line| {
        line.split_whitespace()
            .into_iter()
            .map(|v| v.parse::<u32>().unwrap())
            .collect()
    })
    .collect();
}

pub fn is_report_list_safe(report_list: &Vec<u32>) -> bool {
  let mut i: u32 = 0;
  let mut last_report: u32 = 0;
  let mut is_increasing: bool = false;

  for report in report_list {
    if i == 1 {
      is_increasing = *report > last_report;
    }

    if i != 0 {
      let diff_to_last_report: u32 = report.abs_diff(last_report);
      if diff_to_last_report < 1 ||
        diff_to_last_report > 3 ||
        (*report > last_report) != is_increasing {
          return false;
      }
    }
    
    last_report = *report;
    i += 1;
  }
  
  return true;
}