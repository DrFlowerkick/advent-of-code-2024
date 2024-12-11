//!day_12.rs

use anyhow::Result;

#[derive(Debug)]
struct Day12Data {}

impl From<&str> for Day12Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day12Data {}

pub fn day_12() -> Result<()> {
    let input = include_str!("../../assets/day_12.txt");
    let _challenge = Day12Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 12 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 12 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_12_example.txt");
        let _challenge = Day12Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 12 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 12 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
