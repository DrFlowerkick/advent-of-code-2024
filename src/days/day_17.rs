//!day_17.rs

use anyhow::Result;

#[derive(Debug)]
struct Day17Data {}

impl From<&str> for Day17Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day17Data {}

pub fn day_17() -> Result<()> {
    let input = include_str!("../../assets/day_17.txt");
    let _challenge = Day17Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 17 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 17 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_17_example.txt");
        let _challenge = Day17Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 17 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 17 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
