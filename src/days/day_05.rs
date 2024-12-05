//!day_05.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct Day05Data {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

impl From<&str> for Day05Data {
    fn from(value: &str) -> Self {
        let (rules_str, updates) = value.split_once("\n\n").unwrap();
        let mut rules: HashMap<u32, Vec<u32>> = HashMap::with_capacity(100);
        for line in rules_str.lines() {
            let (left, right) = line.split_once('|').unwrap();
            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();
            rules
                .entry(left)
                .and_modify(|v| v.push(right))
                .or_insert(vec![right]);
        }
        let updates: Vec<Vec<u32>> = updates
            .lines()
            .map(|line| {
                line.split(',')
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect()
            })
            .collect();

        Self { rules, updates }
    }
}

impl Day05Data {
    fn add_middle_numbers_of_correct_order_updates(&self) -> u32 {
        let mut sum_middle_numbers = 0;
        'outer: for update in self.updates.iter() {
            // len of update should be odd
            assert!(update.len() & 1 == 1);
            for (index, page) in update.iter().enumerate() {
                // check if any number right of left does not comply to order rules
                if update[index + 1..update.len()]
                    .iter()
                    .map(|right_of_left| match self.rules.get(page) {
                        Some(left_order_items) => {
                            left_order_items.iter().any(|loi| loi == right_of_left)
                        }
                        None => false,
                    })
                    .any(|c| !c)
                {
                    // skip current update and move on to next one
                    continue 'outer;
                }
            }
            // all checks are true -> get middle number and sum it up
            sum_middle_numbers += update[update.len() / 2];
        }
        sum_middle_numbers
    }
}

pub fn day_05() -> Result<()> {
    let input = include_str!("../../assets/day_05.txt");
    let challenge = Day05Data::from(input);

    let result_part1 = challenge.add_middle_numbers_of_correct_order_updates();
    println!("result day 05 part 1: {}", result_part1);
    assert_eq!(result_part1, 5_732);
    /*
    let result_part2 = challenge
    println!("result day 05 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_05_example.txt");
        let challenge = Day05Data::from(input);

        let result_part1 = challenge.add_middle_numbers_of_correct_order_updates();
        println!("result day 05 part 1: {}", result_part1);
        assert_eq!(result_part1, 143);
        /*
        let result_part2 = challenge
        println!("result day 05 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
