//!day_02.rs

use anyhow::Result;

#[derive(Debug)]
struct Day02Data {
}

impl From<&str> for Day02Data {
    fn from(_value: &str) -> Self {
        Self { }
    }
}

impl Day02Data {
}

pub fn day_02() -> Result<()> {
    let input = include_str!("../../assets/day_02.txt");
    let mut _challenge = Day02Data::from(input);

    /*
    let result_part1 = _challenge
    println!("result day 02 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = _challenge
    println!("result day 02 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_02_example.txt");
        let mut _challenge = Day02Data::from(input);
        
        /*
        let result_part1 = _challenge
        println!("result day 02 part 1: {}", result_part1);
        assert_eq!(result_part1, XXX);

        let result_part2 = _challenge
        println!("result day 02 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
