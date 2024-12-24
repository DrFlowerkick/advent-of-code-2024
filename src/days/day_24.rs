//!day_24.rs

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn get_output(&self, input1: bool, input2: bool) -> bool {
        match self {
            Operator::And => input1 & input2,
            Operator::Or => input1 | input2,
            Operator::Xor => input1 ^ input2,
        }
    }
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("Unknown operator."),
        }
    }
}

#[derive(Debug)]
struct Day24Data {
    gates: HashMap<String, (Operator, String, String)>,
    values: HashMap<String, bool>,
}

impl From<&str> for Day24Data {
    fn from(value: &str) -> Self {
        let mut gates: HashMap<String, (Operator, String, String)> = HashMap::new();
        let mut values: HashMap<String, bool> = HashMap::new();
        let (init_values, gate_definitions) = value.split_once("\n\n").unwrap();
        for (node, value) in init_values.lines().filter_map(|l| l.split_once(": ")) {
            let value: bool = value == "1";
            values.insert(node.to_owned(), value);
        }
        let re = Regex::new(r"(\w+)\s+(XOR|OR|AND)\s+(\w+)\s+->\s+(\w+)").unwrap();
        for (output, operator, input1, input2) in gate_definitions.lines().filter_map(|line| {
            re.captures(line.trim()).map(|captures| {
                let input1 = captures.get(1).unwrap().as_str();
                let operator = Operator::from(captures.get(2).unwrap().as_str());
                let input2 = captures.get(3).unwrap().as_str();
                let output = captures.get(4).unwrap().as_str();
                (output, operator, input1, input2)
            })
        }) {
            gates.insert(
                output.to_owned(),
                (operator, input1.to_owned(), input2.to_owned()),
            );
        }

        Self { gates, values }
    }
}

impl Day24Data {
    fn calc_z_values(&mut self) -> u128 {
        let mut z_values: Vec<String> = self
            .gates
            .keys()
            .filter(|k| &k[..1] == "z")
            .cloned()
            .collect();
        z_values.sort();
        let mut output = 0;
        for z_node in z_values.iter().rev() {
            output <<= 1;
            let value: u128 = if self.calc_value(z_node) { 1 } else { 0 };
            output += value;
        }
        output
    }
    fn calc_value(&mut self, node: &String) -> bool {
        if let Some(value) = self.values.get(node) {
            return *value;
        }
        let (operator, input1, input2) = self.gates.get(node).cloned().unwrap();
        let input1 = self.calc_value(&input1);
        let input2 = self.calc_value(&input2);
        let output = operator.get_output(input1, input2);
        self.values.insert(node.clone(), output);
        output
    }
}

pub fn day_24() -> Result<()> {
    let input = include_str!("../../assets/day_24.txt");
    let mut challenge = Day24Data::from(input);

    let result_part1 = challenge.calc_z_values();
    println!("result day 24 part 1: {}", result_part1);
    assert_eq!(result_part1, 56_620_966_442_854);
    /*
    let result_part2 = challenge
    println!("result day 24 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_24_example.txt");
        let mut challenge = Day24Data::from(input);

        let result_part1 = challenge.calc_z_values();
        println!("result day 24 part 1: {}", result_part1);
        assert_eq!(result_part1, 2_024);
        /*
        let result_part2 = challenge
        println!("result day 24 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
