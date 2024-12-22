//!day_23.rs

use anyhow::Result;

#[derive(Debug)]
struct Day23Data {}

impl From<&str> for Day23Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day23Data {}

pub fn day_23() -> Result<()> {
    let input = include_str!("../../assets/day_23.txt");
    let _challenge = Day23Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 23 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 23 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_23_example.txt");
        let _challenge = Day23Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 23 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 23 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
