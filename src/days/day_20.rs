//!day_20.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    usize,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Cheat<const N: usize> {
    pos_1: MapPoint<N, N>,
    pos_2: MapPoint<N, N>,
}

impl<const N: usize> Cheat<N> {
    fn new(point_1: MapPoint<N, N>, point_2: MapPoint<N, N>) -> Self {
        Self {
            pos_1: point_1.min(point_2),
            pos_2: point_1.max(point_2),
        }
    }
}

#[derive(Debug)]
struct Day20Data<const N: usize> {
    map: MyMap2D<char, N, N>,
    start: MapPoint<N, N>,
    end: MapPoint<N, N>,
}

impl<const N: usize> From<&str> for Day20Data<N> {
    fn from(value: &str) -> Self {
        let map: MyMap2D<char, N, N> = MyMap2D::from(value);
        let start = map
            .iter()
            .find(|(_, c)| **c == 'S')
            .map(|(p, _)| p)
            .unwrap();
        let end = map
            .iter()
            .find(|(_, c)| **c == 'E')
            .map(|(p, _)| p)
            .unwrap();
        Self { map, start, end }
    }
}

impl<const N: usize> Day20Data<N> {
    fn calc_distance(&self) -> (Vec<(MapPoint<N, N>, usize)>, usize) {
        let mut seen: HashSet<MapPoint<N, N>> = HashSet::with_capacity(N);
        seen.insert(self.start);
        let mut path: Vec<(MapPoint<N, N>, usize)> = Vec::with_capacity(N);
        path.push((self.start, 0));
        let mut visit: VecDeque<(MapPoint<N, N>, usize)> = VecDeque::new();
        visit.push_back((self.start, 0));
        let mut start_to_end = usize::MAX;
        while let Some((point, distance)) = visit.pop_front() {
            if point == self.end {
                start_to_end = distance;
                break;
            }
            for neighbor in [Compass::N, Compass::E, Compass::W, Compass::S]
                .iter()
                .filter_map(|dir| point.neighbor(*dir))
                .filter(|p| *self.map.get(*p) != '#')
            {
                if seen.insert(neighbor) {
                    path.push((neighbor, distance + 1));
                    visit.push_back((neighbor, distance + 1));
                }
            }
        }
        (path, start_to_end)
    }
    fn find_cheats(&mut self, min_distance_reduction: usize) -> usize {
        let (path, start_to_end) = self.calc_distance();
        // the racetrack has truly only one path from start to end.
        assert_eq!(path.len(), start_to_end + 1);
        let mut count = 0;
        assert!(min_distance_reduction > 1);
        for (path_tile, distance) in path.iter() {
            for (_cheat_end, cheat_end_distance) in path_tile
                .iter_neighbors(Compass::N, false, false, false)
                .filter(|(n, _)| *self.map.get(*n) == '#')
                .flat_map(|(wall, _)| {
                    wall.iter_neighbors(Compass::N, false, false, false)
                        .filter(|(w, _)| *self.map.get(*w) != '#')
                })
                .filter_map(|(ce, _)| path.iter().find(move |(p, d)| *p == ce && d > distance))
            {
                if cheat_end_distance - distance - 2 >= min_distance_reduction {
                    count += 1;
                }
            }
        }
        /*for (index, a) in keys.iter().enumerate() {
            for b in keys[index + 1..].iter() {
                if distances
                    .get(a)
                    .unwrap()
                    .abs_diff(*distances.get(b).unwrap())
                    > 3 + min_distance_reduction
                {
                    let possible_cheats = match a.distance(*b) {
                        2 => self.get_cheats_distance_two(a, b),
                        //3 => self.get_cheats_distance_three(a, b),
                        _ => continue,
                    };
                    for possible_cheat in possible_cheats.iter() {
                        if cheats.insert(*possible_cheat) {
                            let pos_1 = self.map.swap_value(possible_cheat.pos_1, '.');
                            let pos_2 = self.map.swap_value(possible_cheat.pos_2, '.');
                            let (_, cheat_distance) = self.calc_distance();
                            if start_to_end - cheat_distance >= min_distance_reduction {
                                count += 1;
                            }
                            self.map.swap_value(possible_cheat.pos_1, pos_1);
                            self.map.swap_value(possible_cheat.pos_2, pos_2);
                        }
                    }
                }
            }
        }*/
        count
    }
    /*fn get_cheats_distance_two(&self, a: &MapPoint<N, N>, b: &MapPoint<N, N>) -> Vec<Cheat<N>> {
        let mut cheats: Vec<Cheat<N>> = Vec::with_capacity(2);
        for (d1, _) in a
            .iter_neighbors(Compass::N, false, false, false)
            .filter(|(n, _)| n.distance(*b) == 1)
        {
            cheats.push(Cheat::<N>::new(*a, d1));
            //cheats.push(Cheat::<N>::new(*b, d1));
        }

        cheats
    }
    fn get_cheats_distance_three(&self, a: &MapPoint<N, N>, b: &MapPoint<N, N>) -> Vec<Cheat<N>> {
        let mut cheats: Vec<Cheat<N>> = Vec::with_capacity(3);
        for (d2, _) in a
            .iter_neighbors(Compass::N, false, false, false)
            .filter(|(n2, _)| n2.distance(*b) == 2)
        {
            for (d1, _) in d2
                .iter_neighbors(Compass::N, false, false, false)
                .filter(|(n1, _)| n1.distance(*b) == 1)
            {
                cheats.push(Cheat::<N>::new(d1, d2));
            }
        }
        cheats
    }*/
}

const N: usize = 141;

pub fn day_20() -> Result<()> {
    let input = include_str!("../../assets/day_20.txt");
    let mut challenge = Day20Data::<N>::from(input);

    let result_part1 = challenge.find_cheats(100);
    println!("result day 20 part 1: {}", result_part1);
    assert_eq!(result_part1, 1_393);
    /*
    let result_part2 = challenge
    println!("result day 20 part 2: {}", result_part2);
    assert_eq!(result_part2, XXX);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    const E: usize = 15;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_20_example.txt");
        let mut challenge = Day20Data::<E>::from(input);

        let result_part1 = challenge.find_cheats(6);
        println!("result day 20 part 1: {}", result_part1);
        assert_eq!(result_part1, 16);
        /*
        let result_part2 = challenge
        println!("result day 20 part 2: {}", result_part2);
        assert_eq!(result_part2, XXX);
        */
        Ok(())
    }
}
