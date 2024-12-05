//!day_06.rs

use anyhow::Result;

#[derive(Debug)]
struct Day06Data {
}

impl From<&str> for Day06Data {
    fn from(_value: &str) -> Self {
        Self { }
    }
}

impl Day06Data {
}

pub fn day_06() -> Result<()> {
    println!("Happy Nikolaus!");
    let input = include_str!("../../assets/day_06.txt");
    let _challenge = Day06Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 06 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 06 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_06_example.txt");
        let _challenge = Day06Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 06 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 06 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
