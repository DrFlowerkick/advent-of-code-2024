//!day_01.rs

use anyhow::Result;

#[derive(Debug)]
struct TestData {
    result: u128,
    items: Vec<u128>,
}

impl TestData {
    fn test_data(&self) -> bool {
        assert!(!self.items.is_empty());
        self.test_data_slice(self.items[0], &self.items[1..])
    }
    fn test_data_slice(&self, current_result: u128, slice: &[u128]) -> bool {
        if slice.is_empty() {
            return self.result == current_result;
        }
        let add_result = current_result + slice[0];
        if self.test_data_slice(add_result, &slice[1..]) {
            return true;
        }
        let mul_result = current_result * slice[0];
        self.test_data_slice(mul_result, &slice[1..])
    }
}

#[derive(Debug)]
struct Day07Data {
    samples: Vec<TestData>,
}

impl From<&str> for Day07Data {
    fn from(value: &str) -> Self {
        let samples: Vec<TestData> = value
            .lines()
            .filter_map(|l| l.split_once(':'))
            .filter_map(|(r, i)| {
                r.trim().parse::<u128>().ok().map(|result| TestData {
                    result,
                    items: i
                        .trim()
                        .split_whitespace()
                        .filter_map(|li| li.parse::<u128>().ok())
                        .collect(),
                })
            })
            .collect();
        Self { samples }
    }
}

impl Day07Data {
    fn test_samples(&self) -> u128 {
        self.samples
            .iter()
            .filter(|td| td.test_data())
            .map(|td| td.result)
            .sum()
    }
}

pub fn day_07() -> Result<()> {
    let input = include_str!("../../assets/day_07.txt");
    let challenge = Day07Data::from(input);

    let result_part1 = challenge.test_samples();
    println!("result day 07 part 1: {}", result_part1);
    assert_eq!(result_part1, 20_665_830_408_335);
    /*
    let result_part2 = challenge
    println!("result day 07 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_07_example.txt");
        let challenge = Day07Data::from(input);

        let result_part1 = challenge.test_samples();
        println!("result day 07 part 1: {}", result_part1);
        assert_eq!(result_part1, 3_749);
        /*
        let result_part2 = challenge
        println!("result day 07 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
