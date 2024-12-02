//!day_03.rs

use anyhow::Result;

#[derive(Debug)]
struct Day03Data {
}

impl From<&str> for Day03Data {
    fn from(_value: &str) -> Self {
        Self {
        }
    }
}

impl Day03Data {
}

pub fn day_03() -> Result<()> {
    let input = include_str!("../../assets/day_03.txt");
    let _challenge = Day03Data::from(input);
    /*
    let result_part1 = challenge;
    println!("result day 03 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge;
    println!("result day 03 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_03_example.txt");
        let _challenge = Day03Data::from(input);
        /*
        let result_part1 = challenge;
        println!("result day 03 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge;
        println!("result day 03 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
