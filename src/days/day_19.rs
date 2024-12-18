//!day_19.rs

use anyhow::Result;

#[derive(Debug)]
struct Day19Data {}

impl From<&str> for Day19Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day19Data {}

pub fn day_19() -> Result<()> {
    let input = include_str!("../../assets/day_19.txt");
    let _challenge = Day19Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 19 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 19 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_19_example.txt");
        let _challenge = Day19Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 19 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 19 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
