//!day_22.rs

use anyhow::Result;

#[derive(Debug)]
struct Day22Data {}

impl From<&str> for Day22Data {
    fn from(value: &str) -> Self {
        Self {}
    }
}

impl Day22Data {}

pub fn day_22() -> Result<()> {
    let input = include_str!("../../assets/day_22.txt");
    let _challenge = Day22Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 22 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 22 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_22_example.txt");
        let _challenge = Day22Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 22 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 22 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
