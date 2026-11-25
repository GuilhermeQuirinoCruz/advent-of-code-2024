use std::collections::HashMap;
use std::collections::HashSet;

fn get_page_ordering_rules(rules: &str) -> HashMap<u32, HashSet<u32>> {
  let mut page_ordering_map: HashMap<u32, HashSet<u32>> = HashMap::new();
  for rule in rules.lines() {
    let (key, value) = rule.split_once("|")
      .map(|x| (x.0.parse::<u32>().unwrap(), x.1.parse::<u32>().unwrap()))
      .unwrap();

    page_ordering_map.entry(key)
      .or_insert_with(|| HashSet::new())
      .insert(value);
  }

  return page_ordering_map;
}

fn get_pages_to_produce(page_lists: &str) -> Vec<Vec<u32>> {
  return page_lists
    .lines()
    .map(|l| l.split(",")
      .map(|x| x.parse::<u32>().unwrap())
      .collect::<Vec<u32>>())
    .collect::<Vec<Vec<u32>>>();
}

pub fn get_page_data(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
  return input
    .split_once("\n\n")
    .map(|l| (get_page_ordering_rules(l.0), get_pages_to_produce(l.1)))
    .unwrap();
}

pub fn is_update_in_order(rules: &HashMap<u32, HashSet<u32>>, update: &Vec<u32>) -> bool {
  println!("Analyzing {:?}, length {}", update, update.len());
  for i in (1..update.len()).rev() {
    let current: &u32 = &update[i];
    println!("Checking [{}]: {}", i, current);
    if !rules.contains_key(current) {
      println!("No rules found for {}", current);
      continue;
    }

    let numbers_after = rules.get(current).unwrap();
    println!("Numbers that should come AFTER: {:?}", numbers_after);
    for i2 in (0..i).rev() {
      let compare_to: &u32 = &update[i2];
      println!("Comparing to {}", compare_to);
      if numbers_after.contains(compare_to) {
        println!("Incorrect because of {} showing up before {}", compare_to, current);
        return false;
      }
    }
  }

  return true;
}

pub fn correct_page(rules: &HashMap<u32, HashSet<u32>>, update: &Vec<u32>) -> Vec<u32> {
  println!("Analyzing {:?}, length {}", update, update.len());
  let mut correct: Vec<u32> = update.clone();

  // There's no need to check the entire Vec
  // for i in ((correct.len() / 2)..correct.len()).rev() {
  for i in (0..correct.len()).rev() {
    for i2 in (0..i).rev() {
      let current: &u32 = &correct[i];
      println!("Checking [{}]: {}", i, current);
      if !rules.contains_key(current) {
        println!("No rules found for {}", current);
        continue;
      }
      
      let numbers_after = rules.get(current).unwrap();
      println!("Numbers that should come AFTER: {:?}", numbers_after);
      
      let compare_to: &u32 = &correct[i2];
      println!("Comparing {} to {}", current, compare_to);
      if numbers_after.contains(compare_to) {
        println!("Swapping");
        (correct[i], correct[i2]) = (correct[i2], correct[i]);
      }
    }
  }
  
  return correct;
}