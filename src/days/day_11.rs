//!day_11.rs

use anyhow::Result;

#[derive(Debug)]
struct Day11Data {}

impl From<&str> for Day11Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day11Data {}

pub fn day_11() -> Result<()> {
    let input = include_str!("../../assets/day_11.txt");
    let _challenge = Day11Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 11 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 11 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_11_example.txt");
        let _challenge = Day11Data::from(input);

        /*
        let result_part1 = challenge
        println!("result day 11 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 11 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */

        Ok(())
    }
}
