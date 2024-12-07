//!day_08.rs

use anyhow::Result;

#[derive(Debug)]
struct Day08Data {
}

impl From<&str> for Day08Data {
    fn from(_value: &str) -> Self {
        Self {
        }
    }
}

impl Day08Data {
}

pub fn day_08() -> Result<()> {
    let input = include_str!("../../assets/day_08.txt");
    let _challenge = Day08Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 08 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 08 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_08_example.txt");
        let _challenge = Day08Data::from(input);
        /*
        let result_part1 = challenge
        println!("result day 08 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = challenge
        println!("result day 08 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
