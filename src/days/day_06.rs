//!day_06.rs

use anyhow::Result;
use my_lib::{my_compass::Compass, my_map_point::MapPoint, my_map_two_dim::MyMap2D};
use std::collections::HashSet;

struct SnapshotHashSet<T, const N: usize> {
    set: HashSet<T>,
    baseline: Option<HashSet<T>>,
}

impl<T: std::hash::Hash + Eq + Clone, const N: usize> SnapshotHashSet<T, N> {
    fn new() -> Self {
        Self {
            set: HashSet::with_capacity(N),
            baseline: None,
        }
    }

    fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }

    fn set_baseline(&mut self) {
        self.baseline = Some(self.set.clone());
    }

    fn reset_to_baseline(&mut self) {
        if let Some(baseline) = self.baseline.take() {
            self.set = baseline;
        }
    }
}


struct IterMap<'a, const N: usize> {
    map: &'a MyMap2D<char, N, N>,
    current_tile: MapPoint<N, N>,
    direction: Compass,
}

impl<'a, const N: usize> IterMap<'a, N> {
    fn new(map: &'a MyMap2D<char, N, N>, current_tile: MapPoint<N, N>, direction: Compass) -> Self {
        IterMap {
            map,
            current_tile,
            direction,
        }
    }
}

impl<'a, const N: usize> Iterator for IterMap<'a, N> {
    type Item = (MapPoint<N, N>, Compass);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next_tile) = self.current_tile.neighbor(self.direction) {
            if *self.map.get(next_tile) != '#' {
                self.current_tile = next_tile;
                return Some((self.current_tile, self.direction));
            }
            self.direction = self.direction.clockwise().clockwise();
        }
        None
    }
}

#[derive(Debug)]
struct Day06Data<const N: usize> {
    map: MyMap2D<char, N, N>,
    start_tile: MapPoint<N, N>,
}

impl<const N: usize> From<&str> for Day06Data<N> {
    fn from(value: &str) -> Self {
        let map = MyMap2D::from(value);
        let (start_tile, _) = map.iter().find(|(_, c)| **c == '^').unwrap();
        Self { map, start_tile }
    }
}

impl<const N: usize> Day06Data<N> {
    fn count_visited_map_tiles(&self) -> usize {
        let mut visited_tiles: HashSet<MapPoint<N, N>> = HashSet::with_capacity(N);
        visited_tiles.insert(self.start_tile);
        let iter_map = IterMap::new(&self.map, self.start_tile, Compass::N);
        for (next_tile, _) in iter_map {
            visited_tiles.insert(next_tile);
        }
        visited_tiles.len()
    }

    fn count_possible_loop_blocks(&mut self) -> usize {
        let mut visited_tiles: SnapshotHashSet<(MapPoint<N, N>, Compass), N> = SnapshotHashSet::new();
        let mut blocked_tiles: HashSet<MapPoint<N, N>> = HashSet::with_capacity(N);
        // insert start_tile to prevent it from being blocked during for loop
        blocked_tiles.insert(self.start_tile);
        // copy map, since borrow checker would prevent changing values of it.
        let map = self.map;
        let mut current_tile = self.start_tile;
        let mut current_direction = Compass::N;
        let iter_map = IterMap::new(&map, current_tile, current_direction);
        let mut loop_blocks = 0;
        for (next_tile, next_direction) in iter_map {
            visited_tiles.insert((current_tile, current_direction));
            // block next tile in path (if not blocked yet) and check if it results in a loop
            if blocked_tiles.insert(next_tile) {
                // block next_tile
                let next_tile_value = self.map.swap_value(next_tile, '#');
                // use a second visited cache for checking, because otherwise it would be filled with visits
                // from map states, which do not exist anymore
                visited_tiles.set_baseline();
                let check_loop_iter = IterMap::new(&self.map, current_tile, current_direction);
                for (check_tile, check_direction) in check_loop_iter {
                    if !visited_tiles.insert((check_tile, check_direction)) {
                        // new loop
                        loop_blocks += 1;
                        break;
                    }
                }
                visited_tiles.reset_to_baseline();
                // unblock next_tile
                self.map.set(next_tile, next_tile_value);
            }
            current_tile = next_tile;
            current_direction = next_direction;
        }
        loop_blocks
    }
}

// taken from ../../assets/day_06.txt
const N: usize = 130;

pub fn day_06() -> Result<()> {
    println!("Happy Nikolaus!");
    let input = include_str!("../../assets/day_06.txt");
    let mut challenge = Day06Data::<N>::from(input);

    let result_part1 = challenge.count_visited_map_tiles();
    println!("result day 06 part 1: {}", result_part1);
    assert_eq!(result_part1, 4_826);

    let result_part2 = challenge.count_possible_loop_blocks();
    println!("result day 06 part 2: {}", result_part2);
    assert_eq!(result_part2, 1_721);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    // taken from ../../assets/day_06_example.txt
    const E: usize = 10;

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../assets/day_06_example.txt");
        let mut challenge = Day06Data::<E>::from(input);

        let result_part1 = challenge.count_visited_map_tiles();
        println!("result day 06 part 1: {}", result_part1);
        assert_eq!(result_part1, 41);

        let result_part2 = challenge.count_possible_loop_blocks();
        println!("result day 06 part 2: {}", result_part2);
        assert_eq!(result_part2, 6);

        Ok(())
    }
}
