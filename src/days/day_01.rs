//!day_01.rs

use anyhow::Result;

pub fn day_01() -> Result<()> {
    let _input = include_str!("../../assets/day_01.txt");

    /*
    let result_part1 = max_calories.last().unwrap();
    println!("result day 01 part 1: {}", result_part1);
    assert_eq!(*result_part1, 74_711);

    let result_part2: u64 = max_calories[max_calories.len() - 3..].iter().sum();
    println!("result day 01 part 2: {}", result_part2);
    assert_eq!(result_part2, 209_481);
    */

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let _input = include_str!("../../assets/day_01_example.txt");

        /*
        let result_part1 = max_calories.last().unwrap();
        println!("result day 01 part 1: {}", result_part1);
        assert_eq!(*result_part1, 74_711);

        let result_part2: u64 = max_calories[max_calories.len() - 3..].iter().sum();
        println!("result day 01 part 2: {}", result_part2);
        assert_eq!(result_part2, 209_481);
        */
        Ok(())
    }
}