//!day_16.rs

use anyhow::Result;

#[derive(Debug)]
struct Day16Data {}

impl From<&str> for Day16Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day16Data {}

pub fn day_16() -> Result<()> {
    let input = include_str!("../../assets/day_16.txt");
    let _challenge = Day16Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 16 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 16 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_16_example.txt");
        let _challenge = Day16Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 16 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 16 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
