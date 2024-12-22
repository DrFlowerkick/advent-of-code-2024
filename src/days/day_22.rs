//!day_22.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct Day22Data {
    secrets: Vec<u128>,
}

impl From<&str> for Day22Data {
    fn from(value: &str) -> Self {
        Self {
            secrets: value.lines().map(|v| v.parse::<u128>().unwrap()).collect(),
        }
    }
}

impl Day22Data {
    fn sum_up_new_secrets(&self, generations: usize) -> u128 {
        let mut cache: HashMap<(u128, usize), u128> = HashMap::new();
        let mut sum_new_secrets = 0;
        for secret in self.secrets.iter() {
            sum_new_secrets += self.calc_secret(*secret, generations, &mut cache);
        }
        sum_new_secrets
    }
    fn calc_secret(
        &self,
        secret: u128,
        remaining_generations: usize,
        cache: &mut HashMap<(u128, usize), u128>,
    ) -> u128 {
        if remaining_generations == 0 {
            return secret;
        }
        if let Some(cached_secret) = cache.get(&(secret, remaining_generations)) {
            return *cached_secret;
        }
        // multiplying the secret number by 64 -> shift 6 bits to the left
        let shift_secret = secret << 6;
        // mix + prune (prune module 16777216 is equal to & 16777216, because 2^24 == 16777216)
        let mut new_secret = (shift_secret ^ secret) & (16_777_216 - 1);
        // dividing the secret number by 32 -> shift 5 bits to the right
        let shift_secret = new_secret >> 5;
        // mix + prune (prune module 16777216 is equal to & 16777216, because 2^24 == 16777216)
        new_secret = (shift_secret ^ new_secret) & (16_777_216 - 1);
        // multiplying the secret number by 2048 -> shift 11 bits to the left
        let shift_secret = new_secret << 11;
        // mix + prune (prune module 16777216 is equal to & 16777216, because 2^24 == 16777216)
        new_secret = (shift_secret ^ new_secret) & (16_777_216 - 1);
        cache.insert((secret, remaining_generations), new_secret);
        self.calc_secret(new_secret, remaining_generations - 1, cache)
    }
}

pub fn day_22() -> Result<()> {
    let input = include_str!("../../assets/day_22.txt");
    let challenge = Day22Data::from(input);

    let result_part1 = challenge.sum_up_new_secrets(2000);
    println!("result day 22 part 1: {}", result_part1);
    assert_eq!(result_part1, 13_753_970_725);
    /*
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
    fn test_calc_secret() {
        let mut cache: HashMap<(u128, usize), u128> = HashMap::new();
        let dummy = Day22Data {
            secrets: Vec::new(),
        };
        let secret = 123_u128;
        let secret = dummy.calc_secret(secret, 1, &mut cache);
        assert_eq!(secret, 15_887_950);
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_22_example.txt");
        let challenge = Day22Data::from(input);
        dbg!(&challenge);
        let result_part1 = challenge.sum_up_new_secrets(2000);
        println!("result day 22 part 1: {}", result_part1);
        assert_eq!(result_part1, 37_327_623);
        /*
        let result_part2 = challenge
        println!("result day 22 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
