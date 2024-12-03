//!day_04.rs

use anyhow::Result;

#[derive(Debug)]
struct Day04Data {
}

impl From<&str> for Day04Data {
    fn from(_value: &str) -> Self {
        Self {
        }
    }
}

impl Day04Data {
}

pub fn day_04() -> Result<()> {
    let input = include_str!("../../assets/day_04.txt");
    let _challenge = Day04Data::from(input);
    /*
    let result_part1 = challenge.count_save_reports(false);
    println!("result day 04 part 1: {}", result_part1);
    assert_eq!(result_part1, 287);

    let result_part2 = challenge.count_save_reports(true);
    println!("result day 04 part 2: {}", result_part2);
    assert_eq!(result_part2, 354);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_04_example.txt");
        let _challenge = Day04Data::from(input);
        /*
        let result_part1 = challenge.count_save_reports(false);
        println!("result day 04 part 1: {}", result_part1);
        assert_eq!(result_part1, 2);

        let result_part2 = challenge.count_save_reports(true);
        println!("result day 04 part 2: {}", result_part2);
        assert_eq!(result_part2, 4);
        */
        Ok(())
    }
}
