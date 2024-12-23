//!day_24.rs

use anyhow::Result;

#[derive(Debug)]
struct Day24Data {}

impl From<&str> for Day24Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day24Data {}

pub fn day_24() -> Result<()> {
    let input = include_str!("../../assets/day_24.txt");
    let _challenge = Day24Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 24 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 24 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_24_example.txt");
        let _challenge = Day24Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 24 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 24 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
