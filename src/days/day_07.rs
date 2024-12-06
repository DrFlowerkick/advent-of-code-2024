//!day_01.rs

use anyhow::Result;

#[derive(Debug)]
struct Day07Data {
}

impl From<&str> for Day07Data {
    fn from(_value: &str) -> Self {
        Self { }
    }
}

impl Day07Data {
}

pub fn day_07() -> Result<()> {
    let input = include_str!("../../assets/day_07.txt");
    let mut _challenge = Day07Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 07 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 07 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_07_example.txt");
        let mut _challenge = Day07Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 07 part 1: {}", result_part1);
        assert_eq!(result_part1, XX);

        let result_part2 = challenge
        println!("result day 07 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
