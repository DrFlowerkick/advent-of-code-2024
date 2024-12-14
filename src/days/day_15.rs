//!day_15.rs

use anyhow::Result;

#[derive(Debug)]
struct Day15Data {}

impl From<&str> for Day15Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day15Data {}

pub fn day_15() -> Result<()> {
    let input = include_str!("../../assets/day_15.txt");
    let _challenge = Day15Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 15 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 15 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_15_example.txt");
        let _challenge = Day15Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 15 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 15 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
