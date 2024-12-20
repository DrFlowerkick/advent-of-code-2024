//!day_21.rs

use anyhow::Result;

#[derive(Debug)]
struct Day21Data {}

impl From<&str> for Day21Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day21Data {}

pub fn day_21() -> Result<()> {
    let input = include_str!("../../assets/day_21.txt");
    let _challenge = Day21Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 21 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 21 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_21_example.txt");
        let _challenge = Day21Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 21 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 21 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
