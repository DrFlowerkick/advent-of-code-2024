//!day_13.rs

use anyhow::Result;

#[derive(Debug)]
struct Day13Data {}

impl From<&str> for Day13Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day13Data {}

pub fn day_13() -> Result<()> {
    let input = include_str!("../../assets/day_13.txt");
    let _challenge = Day13Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 13 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 13 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_13_example.txt");
        let _challenge = Day13Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 13 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 13 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
