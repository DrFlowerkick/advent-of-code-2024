//!day_16.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Day16Data<const N: usize> {
    map: MyMap2D<char, N, N>,
    start: MapPoint<N, N>,
    end: MapPoint<N, N>,
}

impl<const N: usize> From<&str> for Day16Data<N> {
    fn from(value: &str) -> Self {
        let map: MyMap2D<char, N, N> = MyMap2D::from(value);
        let start = map
            .iter()
            .find(|(_, c)| **c == 'S')
            .map(|(s, _)| s)
            .unwrap();
        let end = map
            .iter()
            .find(|(_, c)| **c == 'E')
            .map(|(s, _)| s)
            .unwrap();
        Self { map, start, end }
    }
}

impl<const N: usize> Day16Data<N> {
    fn get_lowest_score(&self) -> usize {
        let mut score_cache: HashMap<(MapPoint<N, N>, Compass), usize> = HashMap::with_capacity(N);
        let mut visit: VecDeque<(MapPoint<N, N>, Compass, usize)> = VecDeque::new();
        visit.push_back((self.start, Compass::E, 0));
        let mut min_score: Option<usize> = None;
        while let Some((tile, direction, score)) = visit.pop_front() {
            // check current score
            if let Some(end_score) = min_score {
                if score > end_score {
                    // current score is too expensive -> skip it
                    continue;
                }
            }
            // check cache score
            if let Some(visit_score) = score_cache.get(&(tile, direction)) {
                if score > *visit_score {
                    // current path is expensive -> skip it
                    continue;
                }
            }
            // insert or update score
            score_cache.insert((tile, direction), score);
            // check end
            if tile == self.end {
                // end of maze
                min_score = Some(score);
                continue;
            }
            for (turns, (neighbor, dir)) in tile
                .iter_neighbors(direction, false, false, false)
                .enumerate()
                .filter(|(t, (n, _))| *t != 2 && *self.map.get(*n) != '#')
            {
                let new_score = if turns == 0 { score + 1 } else { score + 1001 };
                visit.push_back((neighbor, dir, new_score));
            }
        }
        *score_cache
            .iter()
            .filter(|((p, _), _)| *p == self.end)
            .map(|(_, s)| s)
            .min()
            .unwrap()
    }
}

// taken from ../../assets/day_16.txt
const N: usize = 141;

pub fn day_16() -> Result<()> {
    let input = include_str!("../../assets/day_16.txt");
    let challenge = Day16Data::<N>::from(input);

    let result_part1 = challenge.get_lowest_score();
    println!("result day 16 part 1: {}", result_part1);
    assert_eq!(result_part1, 83_432);
    /*
    let result_part2 = challenge
    println!("result day 16 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    // taken from ../../assets/day_16_example_01.txt
    const E01: usize = 15;
    // taken from ../../assets/day_16_example_02.txt
    const E02: usize = 17;

    #[test]
    fn test_example_part() -> Result<()> {
        let input_01 = include_str!("../../assets/day_16_example_01.txt");
        let challenge_01 = Day16Data::<E01>::from(input_01);
        let result_part1_1 = challenge_01.get_lowest_score();
        println!("result day 16 part 1_1: {}", result_part1_1);
        assert_eq!(result_part1_1, 7_036);

        let input_02 = include_str!("../../assets/day_16_example_02.txt");
        let challenge_02 = Day16Data::<E02>::from(input_02);
        let result_part1_2 = challenge_02.get_lowest_score();
        println!("result day 16 part 1_2: {}", result_part1_2);
        assert_eq!(result_part1_2, 11_048);

        /*
        let result_part2 = challenge
        println!("result day 16 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
