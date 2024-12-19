//!day_20.rs

use anyhow::Result;

#[derive(Debug)]
struct Day20Data {}

impl From<&str> for Day20Data {
    fn from(_value: &str) -> Self {
        Self {}
    }
}

impl Day20Data {}

pub fn day_20() -> Result<()> {
    let input = include_str!("../../assets/day_20.txt");
    let _challenge = Day20Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 20 part 1: {}", result_part1.len());
    assert_eq!(result_part1.len(), XXX);

    let result_part2: usize = challenge
    println!("result day 20 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_20_example.txt");
        let _challenge = Day20Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 20 part 1: {}", result_part1.len());
        assert_eq!(result_part1.len(), XXX);

        let result_part2: usize = challenge
        println!("result day 20 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
