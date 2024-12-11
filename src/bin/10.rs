use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Coordinate {
    row: usize,
    col: usize,
}

struct Map {
    data: Vec<u32>,
    n_rows: usize,
    n_cols: usize,
}

impl Map {
    fn get(&self, index: &Coordinate) -> Option<u32> {
        if index.row < self.n_rows && index.col < self.n_cols {
            Some(self.data[index.row * self.n_cols + index.col])
        } else {
            None
        }
    }

    fn index_to_coordinate(&self, index: usize) -> Option<Coordinate> {
        if index < self.n_rows * self.n_cols {
            Some(Coordinate {
                row: index / self.n_cols,
                col: index % self.n_cols,
            })
        } else {
            None
        }
    }

    fn find_trailheads(&self) -> Vec<Coordinate> {
        self.data
            .iter()
            .positions(|x| *x == 0)
            .filter_map(|i| self.index_to_coordinate(i))
            .collect_vec()
    }

    fn neighbours(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        [
            // up
            coordinate.row.checked_sub(1).map(|r| Coordinate {
                row: r,
                col: coordinate.col,
            }),
            // down
            coordinate.row.checked_add(1).map(|r| Coordinate {
                row: r,
                col: coordinate.col,
            }),
            // left
            coordinate.col.checked_sub(1).map(|c| Coordinate {
                row: coordinate.row,
                col: c,
            }),
            // right
            coordinate.col.checked_add(1).map(|c| Coordinate {
                row: coordinate.row,
                col: c,
            }),
        ]
        .into_iter()
        .filter_map(|c| c.filter(|c| c.row < self.n_rows && c.col < self.n_cols))
        .collect_vec()
    }

    fn depth_search_1(&self, c: &Coordinate) -> usize {
        let mut visited = HashSet::new();

        let mut search = Vec::new();
        search.push(c.clone());

        let mut count = 0;

        while let Some(v) = search.pop() {
            if !visited.contains(&v) {
                visited.insert(v.clone());

                if let Some(vvalue) = self.get(&v) {
                    if vvalue == 9 {
                        count += 1;
                    }

                    for w in self.neighbours(&v) {
                        if let Some(wvalue) = self.get(&w) {
                            if wvalue > vvalue && wvalue.abs_diff(vvalue) == 1 {
                                search.push(w);
                            }
                        }
                    }
                }
            }
        }

        count
    }

    fn depth_search_2(&self, c: &Coordinate) -> usize {
        let mut search = Vec::new();
        search.push(c.clone());

        let mut count = 0;

        // just visit everything
        // as many times as possible,
        // we are always forced to move 1
        // up, and thus will terminate
        while let Some(v) = search.pop() {
            if let Some(vvalue) = self.get(&v) {
                if vvalue == 9 {
                    count += 1;
                }

                for w in self.neighbours(&v) {
                    if let Some(wvalue) = self.get(&w) {
                        if wvalue > vvalue && wvalue.abs_diff(vvalue) == 1 {
                            search.push(w);
                        }
                    }
                }
            }
        }

        count
    }
}

fn parse(input: &str) -> Option<Map> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let n_rows = lines.len();
    let n_cols = lines[0].len();
    let capacity = n_rows * n_cols;

    let mut data = Vec::with_capacity(capacity);

    for line in lines {
        if line.len() != n_cols {
            return None;
        }

        for ch in line.chars() {
            let num = ch.to_digit(10)?;
            data.push(num);
        }
    }

    Some(Map {
        data,
        n_cols,
        n_rows,
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    parse(input).map(|map| {
        map.find_trailheads()
            .into_iter()
            .map(|x| map.depth_search_1(&x))
            .sum::<usize>()
    })
}

pub fn part_two(input: &str) -> Option<usize> {
    parse(input).map(|map| {
        map.find_trailheads()
            .into_iter()
            .map(|x| map.depth_search_2(&x))
            .sum::<usize>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
