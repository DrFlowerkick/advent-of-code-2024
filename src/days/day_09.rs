//!day_09.rs

use anyhow::Result;

#[derive(Debug)]
struct Day09Data {}

impl From<&str> for Day09Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day09Data {}

pub fn day_09() -> Result<()> {
    let input = include_str!("../../assets/day_09.txt");
    let mut _challenge = Day09Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 09 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 09 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_09_example.txt");
        let mut _challenge = Day09Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 09 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 09 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
