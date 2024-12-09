//!day_10.rs

use anyhow::Result;

#[derive(Debug)]
struct Day10Data {}

impl From<&str> for Day10Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day10Data {}

pub fn day_10() -> Result<()> {
    let input = include_str!("../../assets/day_10.txt");
    let _challenge = Day10Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 10 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 10 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_10_example.txt");
        let _challenge = Day10Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 10 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 10 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
