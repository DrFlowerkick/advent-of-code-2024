//!day_21.rs

use anyhow::Result;
use my_lib::my_geometry::my_point::Point;
use std::collections::{HashMap, hash_map::Entry};

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
    num_pad: KeyPad,
    dir_pad: KeyPad,
    cache_key_strokes: HashMap<(char, char), Vec<String>>,
    cache_num_pad: HashMap<(char, char), String>,
    cache_radiation_dir_pad: HashMap<(char, char), String>,
    cache_frozen_dir_pad: HashMap<(char, char), String>,
}

impl From<&str> for Day21Data {
    fn from(value: &str) -> Self {
        let codes: Vec<String> = value.lines().map(|l| l.to_owned()).collect();
        Self {
            codes,
            num_pad: KeyPad::new_num_pad(),
            dir_pad: KeyPad::new_num_pad(),
            cache_key_strokes: HashMap::new(),
            cache_num_pad: HashMap::new(),
            cache_radiation_dir_pad: HashMap::new(),
            cache_frozen_dir_pad: HashMap::new(),
        }
    }
}

impl Day21Data {
    fn calc_complexities(&mut self) -> usize {
        let codes = self.codes.to_owned();
        let mut complexities = 0;
        for code in codes.iter() {
            let sequence = self.get_num_pad_sequence(&code);
            dbg!(code);
            dbg!(&sequence);
            let num_value: usize = code[..3].parse::<usize>().unwrap();
            complexities += sequence.len() * num_value;
            dbg!(sequence.len() * num_value);
        }
        complexities
    }
    fn get_num_pad_sequence(&mut self, code: &str) -> String {
        let mut sequence = String::new();
        let mut previous_char = 'A';
        for key in code.chars() {
            if let Some(cached_sequence) = self.cache_num_pad.get(&(previous_char, key)) {
                //println!("rc bot: {}", cached_sequence);
                sequence += cached_sequence;
                previous_char = key;
                continue;
            }
            todo!();/*
            let radiation_bot_sequence = self.get_key_strokes(keypad, previous_char, key);
            //println!("r  bot: {}", radiation_bot_sequence);
            let key_sequence = self.get_radiation_dir_pad_sequence(keypad, &radiation_bot_sequence);
            sequence += &key_sequence;
            if (previous_char, key) == ('3', '7') {
                println!("('3', '7'): {}", key_sequence);
            }
            self.cache_num_pad
                .insert((previous_char, key), key_sequence);
            previous_char = key;
            */
        }
        sequence
    }
    fn get_radiation_dir_pad_sequence(&mut self, code: &str) -> String {
        let mut sequence = String::new();
        let mut previous_char = 'A';
        for key in code.chars() {
            if let Some(cached_sequence) = self.cache_radiation_dir_pad.get(&(previous_char, key)) {
                //println!("fc bot: {}", cached_sequence);
                sequence += cached_sequence;
                previous_char = key;
                continue;
            }
            todo!();/*
            let frozen_bot_sequence = self.get_key_strokes(keypad, previous_char, key);
            //println!("f  bot: {}", frozen_bot_sequence);
            let key_sequence = self.get_frozen_dir_pad_sequence(keypad, &frozen_bot_sequence);
            sequence += &key_sequence;
            self.cache_radiation_dir_pad
                .insert((previous_char, key), key_sequence);
            previous_char = key;
            */
        }
        sequence
    }
    fn get_frozen_dir_pad_sequence(&mut self, code: &str) -> String {
        let mut sequence = String::new();
        let mut previous_char = 'A';
        for key in code.chars() {
            if let Some(cached_sequence) = self.cache_frozen_dir_pad.get(&(previous_char, key)) {
                sequence += cached_sequence;
                previous_char = key;
            }
            let key_sequences = self.get_key_strokes(true, previous_char, key);
            
            previous_char = key;
        }
        sequence
    }
    fn get_key_strokes(&mut self, dir_pad: bool, from: char, to: char) -> Vec<String> {
        if let Some(cached_key_strokes) = self.cache_key_strokes.get(&(from, to)) {
            cached_key_strokes.to_owned()
        } else {
            let key_strokes = if dir_pad {
                self.dir_pad.key_strokes(from, to)
            } else {
                self.num_pad.key_strokes(from, to)
            };
            self.cache_key_strokes
                .insert((from, to), key_strokes.clone());
            key_strokes
        }
    }
}

pub fn day_21() -> Result<()> {
    let input = include_str!("../../assets/day_21.txt");
    let _challenge = Day21Data::from(input);
    /*
    let result_part1 = challenge
    println!("result day 21 part 1: {}", result_part1);
    assert_eq!(result_part1, XXX);

    let result_part2 = challenge
    println!("result day 21 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_key_strokes() {
        let num_pad = KeyPad::new_num_pad();
        assert_eq!(num_pad.key_strokes('A', '0'), ["<A"]);
        assert_eq!(num_pad.key_strokes('0', '9').len(), 4);
        assert_eq!(num_pad.key_strokes('1', '0'), [">vA"]);
        assert_eq!(num_pad.key_strokes('0', '1'), ["^<A"]);
        let from_7_to_a = num_pad.key_strokes('7', 'A').len();
        assert_eq!(from_7_to_a, 9);
        assert_eq!(num_pad.key_strokes('A', '7').len(), from_7_to_a);
        assert_eq!(num_pad.key_strokes('0', '0'), ["A"]);

        let dir_pad = KeyPad::new_dir_pad();
        assert_eq!(dir_pad.key_strokes('<', '^'), [">^A"]);
        assert_eq!(dir_pad.key_strokes('^', '<'), ["v<A"]);
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_21_example.txt");
        let mut challenge = Day21Data::from(input);

        let result_part1 = challenge.calc_complexities();
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
