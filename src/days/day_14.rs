//!day_14.rs

use anyhow::Result;

#[derive(Debug)]
struct Day14Data {
}

impl From<&str> for Day14Data {
    fn from(_value: &str) -> Self {
        Self {
        }
    }
}

impl Day14Data {
}

pub fn day_14() -> Result<()> {
    let input = include_str!("../../assets/day_14.txt");
    let _challenge = Day14Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 14 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 14 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
     */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_14_example.txt");
        let _challenge = Day14Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 14 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 14 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
