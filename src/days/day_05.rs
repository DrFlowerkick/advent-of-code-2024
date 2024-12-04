//!day_05.rs

use anyhow::Result;

#[derive(Debug)]
struct Day05Data {
}

impl From<&str> for Day05Data {
    fn from(_value: &str) -> Self {
        Self {
        }
    }
}

impl Day05Data {
}

pub fn day_05() -> Result<()> {
    let input = include_str!("../../assets/day_05.txt");
    let _challenge = Day05Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 05 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 05 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_05_example.txt");
        let _challenge = Day05Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 05 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 05 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
