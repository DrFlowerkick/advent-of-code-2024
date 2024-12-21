//!day_21.rs

use anyhow::Result;
use std::collections::HashMap;
use my_lib::my_geometry::my_point::Point;

#[derive(Debug)]
struct KeyPad {
    keys: HashMap<Point, char>,
}

impl KeyPad {
    fn new_num_pad() -> Self {
        let mut keys: HashMap<Point, char> = HashMap::with_capacity(11);
        keys.insert((0, 0).into(), '7');
        keys.insert((1, 0).into(), '8');
        keys.insert((2, 0).into(), '9');
        keys.insert((0, 1).into(), '4');
        keys.insert((1, 1).into(), '5');
        keys.insert((2, 1).into(), '6');
        keys.insert((0, 2).into(), '1');
        keys.insert((1, 2).into(), '2');
        keys.insert((2, 2).into(), '3');
        keys.insert((1, 3).into(), '0');
        keys.insert((2, 3).into(), 'A');
        Self { keys }
    }
    fn new_dir_pad() -> Self {
        let mut keys: HashMap<Point, char> = HashMap::with_capacity(5);
        keys.insert((1, 0).into(), '^');
        keys.insert((2, 0).into(), 'A');
        keys.insert((0, 1).into(), '<');
        keys.insert((1, 1).into(), 'v');
        keys.insert((2, 1).into(), '>');
        Self { keys }
    }
    fn key_strokes(&self, from: char, to: char) -> Vec<String> {
        if from == to {
            return vec!["A".into()];
        }
        let from_pos = self
            .keys
            .iter()
            .find(|(_, v)| **v == from)
            .map(|(k, _)| *k)
            .unwrap();
        let to_pos = self
            .keys
            .iter()
            .find(|(_, v)| **v == to)
            .map(|(k, _)| *k)
            .unwrap();
        self.key_strokes_recursive(from_pos, to_pos)
    }
    fn key_strokes_recursive(&self, from: Point, to: Point) -> Vec<String> {
        if from == to {
            return vec!["A".into()];
        }
        let new_delta = from.delta(to) - 1;
        let mut sequences: Vec<String> = Vec::new();
        for (new_from, dir_char) in [
            (Point::new(0, -1), "^"),
            (Point::new(1, 0), ">"),
            (Point::new(0, 1), "v"),
            (Point::new(-1, 0), "<"),
        ]
        .iter()
        .map(|(p, d)| (p.add(from), d))
        .filter(|(p, _)| self.keys.contains_key(p) && p.delta(to) == new_delta)
        {
            for sub_sequence in self.key_strokes_recursive(new_from, to).iter() {
                let sequence = dir_char.to_string() + sub_sequence;
                sequences.push(sequence);

            }
        }
        sequences
    }
}

#[derive(Debug)]
struct Day21Data {
    codes: Vec<String>,
    dir_robots: usize,
}

impl From<&str> for Day21Data {
    fn from(value: &str) -> Self {
        let codes: Vec<String> = value.lines().map(|l| l.to_owned()).collect();
        Self {
            codes,
            dir_robots: 2,
        }
    }
}

impl Day21Data {
    fn calc_complexities(&mut self, dir_robots: usize) -> usize {
        let keypad = KeyPad::default();
        let codes = self.codes.to_owned();
        let mut complexities = 0;
        let mut cache: HashMap<(char, char, usize), usize> = HashMap::new();
        self.dir_robots = dir_robots;
        for code in codes.iter() {
            let sequence_len = self.get_key_pad_sequence(&keypad, &code, &mut cache, 0);
            let num_value: usize = code[..3].parse::<usize>().unwrap();
            complexities += sequence_len * num_value;
        }
        complexities
    }
    fn get_key_pad_sequence(
        &self,
        keypad: &KeyPad,
        code: &str,
        cache: &mut HashMap<(char, char, usize), usize>,
        level: usize,
    ) -> usize {
        let mut sequence_len = 0;
        let mut previous_key = if level == 0 { 'A' } else { 'a' };
        for key in code.chars() {
            if let Some(cached_len) = cache.get(&(previous_key, key, level)) {
                sequence_len += cached_len;
                previous_key = key;
                continue;
            }
            let key_strokes = keypad.key_strokes(previous_key, key);
            let sub_sequence_len = if level == self.dir_robots {
                key_strokes.len()
            } else {
                self.get_key_pad_sequence(keypad, &key_strokes, cache, level + 1)
            };
            sequence_len += sub_sequence_len;
            cache.insert((previous_key, key, level), sub_sequence_len);
            previous_key = key;
        }
        sequence_len
    }
}

pub fn day_21() -> Result<()> {
    let input = include_str!("../../assets/day_21.txt");
    let mut challenge = Day21Data::from(input);

    let result_part1 = challenge.calc_complexities(2);
    println!("result day 21 part 1: {}", result_part1);
    assert_eq!(result_part1, 197_560);
    
    let result_part2 = challenge.calc_complexities(25);
    println!("result day 21 part 2: {}", result_part2);
    //assert_eq!(result_part2, XXX);
    
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_key_strokes() {
        let key_pad = KeyPad::default();
        assert_eq!(key_pad.key_strokes('A', '0'), "<a");
        assert_eq!(key_pad.key_strokes('0', '9'), ">^^^a");
        assert_eq!(key_pad.key_strokes('1', '0'), ">va");
        assert_eq!(key_pad.key_strokes('0', '1'), "^<a");
        assert_eq!(key_pad.key_strokes('<', '^'), ">^a");
        assert_eq!(key_pad.key_strokes('^', '<'), "v<a");
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_21_example.txt");
        let mut challenge = Day21Data::from(input);

        let result_part1 = challenge.calc_complexities(2);
        println!("result day 21 part 1: {}", result_part1);
        assert_eq!(result_part1, 126_384);
        /*
        let result_part2 = challenge
        println!("result day 21 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
