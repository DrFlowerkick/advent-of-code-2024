//!day_21.rs

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct KeyPad {
    keys: HashMap<char, (i32, i32)>,
}

impl Default for KeyPad {
    fn default() -> Self {
        let mut keys: HashMap<char, (i32, i32)> = HashMap::with_capacity(16);
        keys.insert('7', (0, 0));
        keys.insert('8', (1, 0));
        keys.insert('9', (2, 0));
        keys.insert('4', (0, 1));
        keys.insert('5', (1, 1));
        keys.insert('6', (2, 1));
        keys.insert('1', (0, 2));
        keys.insert('2', (1, 2));
        keys.insert('3', (2, 2));
        keys.insert('0', (1, 3));
        keys.insert('A', (2, 3));
        keys.insert('^', (1, 0));
        keys.insert('a', (2, 0));
        keys.insert('<', (0, 1));
        keys.insert('v', (1, 1));
        keys.insert('>', (2, 1));
        Self { keys }
    }
}

impl KeyPad {
    fn key_strokes(&self, from: char, to: char) -> String {
        let from_pos = self.keys.get(&from).unwrap();
        let to_pos = self.keys.get(&to).unwrap();
        let delta_x = to_pos.0 - from_pos.0;
        let dir_char = if delta_x > 0 { '>' } else { '<' };
        let dir_x: Vec<char> = vec![dir_char; delta_x.abs() as usize];
        let delta_y = to_pos.1 - from_pos.1;
        // y goes from top to bottom!
        let dir_char = if delta_y < 0 { '^' } else { 'v' };
        let dir_y: Vec<char> = vec![dir_char; delta_y.abs() as usize];
        let mut sequence = String::new();
        let mut index_x = 0;
        let mut index_y = 0;
        let mut current = from;
        loop {
            if index_x < dir_x.len() {
                if let Some(new_from) = self.key_from_dir(current, dir_x[index_x]) {
                    sequence.push(dir_x[index_x]);
                    current = new_from;
                    index_x += 1;
                    continue;
                }
            }
            if index_y < dir_y.len() {
                if let Some(new_from) = self.key_from_dir(current, dir_y[index_y]) {
                    sequence.push(dir_y[index_y]);
                    current = new_from;
                    index_y += 1;
                    continue;
                }
            }
            if index_x == dir_x.len() && index_y == dir_y.len() {
                return sequence + "a";
            }
            panic!("Unexpected sequence error from '{}' to '{}' (dir_x: {:?}, dir_y:{:?}", from, to, dir_x, dir_y);
        }
        /*
        if from.0 == 0 {
            format!("{}{}a", dir_x, dir_y)
        } else {
            format!("{}{}a", dir_y, dir_x)
        } */
    }
    fn key_from_dir(&self, from: char, dir: char) -> Option<char> {
        let dir = match dir {
            '^' => (0, -1),
            '<' => (-1, 0),
            'v' => (0, 1),
            '>' => (1, 0),
            _ => panic!("unexpected dir."),
        };
        let num_pad = "0123456789A";
        let dir_pad = "^<v>a";
        let is_num_pad = num_pad.contains(from);
        if let Some(pos) = self.keys.get(&from) {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            return self
                .keys
                .iter()
                .find(|(k, v)| {
                    **v == new_pos && ((is_num_pad && num_pad.contains(**k))
                        || (!is_num_pad && dir_pad.contains(**k)))
                })
                .map(|(k, _)| *k);
        }
        None
    }
}

#[derive(Debug)]
struct Day21Data {
    codes: Vec<String>,
    cache_key_strokes: HashMap<(char, char), String>,
    cache_num_pad: HashMap<(char, char), String>,
    cache_radiation_dir_pad: HashMap<(char, char), String>,
}

impl From<&str> for Day21Data {
    fn from(value: &str) -> Self {
        let codes: Vec<String> = value.lines().map(|l| l.to_owned()).collect();
        Self {
            codes,
            cache_key_strokes: HashMap::new(),
            cache_num_pad: HashMap::new(),
            cache_radiation_dir_pad: HashMap::new(),
        }
    }
}

impl Day21Data {
    fn calc_complexities(&mut self) -> usize {
        let keypad = KeyPad::default();
        let codes = self.codes.to_owned();
        let mut complexities = 0;
        for code in codes.iter() {
            let sequence = self.get_num_pad_sequence(&keypad, &code);
            dbg!(code);
            dbg!(&sequence);
            let num_value: usize = code[..3].parse::<usize>().unwrap();
            complexities += sequence.len() * num_value;
            dbg!(sequence.len() * num_value);
        }
        complexities
    }
    fn get_num_pad_sequence(&mut self, keypad: &KeyPad, code: &str) -> String {
        let mut sequence = String::new();
        let mut previous_char = 'A';
        for key in code.chars() {
            if let Some(cached_sequence) = self.cache_num_pad.get(&(previous_char, key)) {
                //println!("rc bot: {}", cached_sequence);
                sequence += cached_sequence;
                previous_char = key;
                continue;
            }
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
        }
        sequence
    }
    fn get_radiation_dir_pad_sequence(&mut self, keypad: &KeyPad, code: &str) -> String {
        let mut sequence = String::new();
        let mut previous_char = 'a';
        for key in code.chars() {
            if let Some(cached_sequence) = self.cache_radiation_dir_pad.get(&(previous_char, key)) {
                //println!("fc bot: {}", cached_sequence);
                sequence += cached_sequence;
                previous_char = key;
                continue;
            }
            let frozen_bot_sequence = self.get_key_strokes(keypad, previous_char, key);
            //println!("f  bot: {}", frozen_bot_sequence);
            let key_sequence = self.get_frozen_dir_pad_sequence(keypad, &frozen_bot_sequence);
            sequence += &key_sequence;
            self.cache_radiation_dir_pad
                .insert((previous_char, key), key_sequence);
            previous_char = key;
        }
        sequence
    }
    fn get_frozen_dir_pad_sequence(&mut self, keypad: &KeyPad, code: &str) -> String {
        let mut sequence = String::new();
        let mut previous_char = 'a';
        for key in code.chars() {
            let key_sequence = self.get_key_strokes(keypad, previous_char, key);
            //println!("me    : {}", key_sequence);
            sequence += &key_sequence;
            previous_char = key;
        }
        sequence
    }
    fn get_key_strokes(&mut self, keypad: &KeyPad, from: char, to: char) -> String {
        if let Some(cached_key_strokes) = self.cache_key_strokes.get(&(from, to)) {
            cached_key_strokes.to_owned()
        } else {
            let key_strokes = keypad.key_strokes(from, to);
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
    fn test_key_from_dir() {
        let key_pad = KeyPad::default();
        assert_eq!(key_pad.key_from_dir('0', '<'), None);
        assert_eq!(key_pad.key_from_dir('^', '<'), None);

    }

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
