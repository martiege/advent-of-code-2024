use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use num::complex::Complex;

advent_of_code::solution!(12);

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Coordinate {
    row: usize,
    col: usize,
}

struct Map<T> {
    data: Vec<T>,
    n_rows: usize,
    n_cols: usize,
}

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl<T> Map<T>
where
    T: Clone,
{
    fn get(&self, coordinate: &Coordinate) -> Option<T> {
        let index = self.coordinate_to_index(coordinate.clone())?;
        Some(self.data[index].clone())
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

    fn coordinate_to_index(&self, coordinate: Coordinate) -> Option<usize> {
        if coordinate.row < self.n_rows && coordinate.col < self.n_cols {
            Some(coordinate.row * self.n_cols + coordinate.col)
        } else {
            None
        }
    }

    fn parse(input: &str, parser: fn(char) -> T) -> Option<Map<T>> {
        let lines: Vec<&str> = input.trim().lines().collect();
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
                data.push(parser(ch));
            }
        }

        Some(Map {
            data,
            n_cols,
            n_rows,
        })
    }

    fn limited_neighbourhood(&self, coordinate: &Coordinate) -> Vec<(Coordinate, Direction)> {
        [
            // up
            (
                coordinate.row.checked_sub(1).map(|r| Coordinate {
                    row: r,
                    col: coordinate.col,
                }),
                Direction::Up,
            ),
            // down
            (
                coordinate.row.checked_add(1).map(|r| Coordinate {
                    row: r,
                    col: coordinate.col,
                }),
                Direction::Down,
            ),
            // left
            (
                coordinate.col.checked_sub(1).map(|c| Coordinate {
                    row: coordinate.row,
                    col: c,
                }),
                Direction::Left,
            ),
            // right
            (
                coordinate.col.checked_add(1).map(|c| Coordinate {
                    row: coordinate.row,
                    col: c,
                }),
                Direction::Right,
            ),
        ]
        .into_iter()
        .filter_map(|(c, d)| Some((c.filter(|c| c.row < self.n_rows && c.col < self.n_cols)?, d)))
        .collect_vec()
    }
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    value: char,
    inner: HashSet<Coordinate>,
    edges: HashSet<Coordinate>,
    all: HashSet<Coordinate>,
}

struct Results {
    area: usize,
    perimeter: usize,
    sides: usize,
}

struct ComplexMap {
    data: HashMap<Complex<i64>, char>,
    n_rows: usize,
    n_cols: usize,
}

fn parse_complex(input: &str) -> Option<ComplexMap> {
    let lines: Vec<&str> = input.trim().lines().collect();
    if lines.is_empty() {
        return None;
    }

    let n_rows = lines.len();
    let n_cols = lines[0].len();
    let capacity = n_rows * n_cols;

    let mut data: HashMap<Complex<i64>, char> = lines
        .iter()
        .enumerate()
        .map(|(i, r)| r.chars().enumerate().map(|(j, c)| (Complex::new(i as i64, j as i64), c)))
        .flatten()
        .collect();

    for i_add_1 in 0..=n_rows+1 {
        data.insert(Complex::new(i_add_1 as i64 - 1, n_cols as i64), '#');
    }

    for j_add_1 in 0..=n_cols+1 {
        data.insert(Complex::new(n_rows as i64, j_add_1 as i64 - 1), '#');
    }

    Some(ComplexMap {
        data,
        n_rows,
        n_cols,
    })
}

fn search(map: &ComplexMap, coordinate: &Complex<i64>, visited: &mut HashSet<Complex<i64>>, border: char, direction: &Complex<i64>) -> Results {
    if map.data[coordinate] != border {
        if map.data[&(coordinate + direction * Complex::new(0, 1))] == border
        || map.data[&(coordinate - direction + direction * Complex::new(0, 1))] != border {
            return Results {area: 0, perimeter: 1, sides: 1};
        }
        else {
            return Results {area: 0, perimeter: 1, sides: 0};
        }
    }

    if visited.contains(coordinate) {
        return Results {area: 0, perimeter: 0, sides: 0};
    }
    visited.insert(*coordinate);

    [Complex::new(1, 0), Complex::new(-1, 0), Complex::new(0, 1), Complex::new(0, -1)]
        .iter()
        .fold(
            Results {area: 0, perimeter: 0, sides: 0},
            |Results {area, perimeter, sides}, x| {
                let r = search(map, &(coordinate + x), visited, border, x);
                Results {
                    area: area + r.area,
                    perimeter: perimeter + r.perimeter,
                    sides: sides + r.sides,
                }
            })
}

fn get_region(
    start: &Coordinate,
    map: &Map<char>,
    visited: &mut HashSet<Coordinate>,
) -> Option<Region> {
    let mut area = 0;
    let mut perimeter = 0;
    let mut inner = HashSet::new();
    let mut edges = HashSet::new();
    let mut all = HashSet::new();

    let mut search = Vec::new();

    search.push((start.clone(), None));

    while let Some((coordinate, direction)) = search.pop() {
        if !visited.contains(&coordinate) {
            visited.insert(coordinate.clone());

            // each new visited coordinate _WILL_ have
            // the same value per DFS, as we check
            // before adding the coordinate into the
            // stack.
            // the exception is the first, which is the one with
            // a value we want to find
            area += 1;

            if let Some(current_value) = map.get(&coordinate) {
                let mut directions = Vec::new();
                for (neighbour, neighbour_direction) in map.limited_neighbourhood(&coordinate) {
                    if let Some(neighbour_value) = map.get(&neighbour) {
                        if current_value == neighbour_value {
                            search.push((neighbour, Some(neighbour_direction.clone())));
                            directions.push(neighbour_direction);
                        }
                    }
                }

                let valid_neighbours = directions.len();

                // each different neighbour must mean one more perimeter
                // note that this must be done as N possible neighbours
                // minus M valid neighbours, as the perimeter must also be
                // calculated for the edge of the map, where no valid
                // neighbour exists
                perimeter += 4 - valid_neighbours;
                if valid_neighbours == 4 {
                    inner.insert(coordinate.clone());
                } else {
                    edges.insert(coordinate.clone());
                }
                all.insert(coordinate.clone());
            }
        }
    }

    if area != 0 && perimeter != 0 {
        Some(Region {
            area,
            perimeter,
            value: map.get(start)?,
            inner,
            edges,
            all,
        })
    } else {
        None
    }
}

fn count_sides(edges: &HashSet<Coordinate>, map: &Map<char>) -> usize {
    let mut visited = HashSet::new();

    let mut corners = 0;

    for edge in edges {
        if visited.contains(edge) {
            continue;
        }

        let mut previous = None;
        if let Some(current_value) = map.get(&edge) {
            for (neighbour, direction) in map.limited_neighbourhood(&edge) {
                if let Some(neighbour_value) = map.get(&neighbour) {
                    if current_value == neighbour_value {
                        if let Some((previous_neighbour, previous_direction)) = previous {
                            if previous_direction != direction {
                                corners += 1;
                            }
                            visited.insert(previous_neighbour);
                        }
                        previous = Some((neighbour, direction));
                    }
                }
            }
        }
    }

    corners
}

fn get_sides(region: &HashSet<Coordinate>) -> usize {
    let mut side_count = 0;
    for (row_dir_add_1, col_dir_add_1) in [(0, 1), (1, 0), (2, 1), (1, 2),] {
        let mut sides = HashSet::new();
        for coordinate in region {
            if let (Some(row_sub_1), Some(col_sub_1)) = (coordinate.row.checked_sub(1), coordinate.col.checked_sub(1)) {
                let row = row_sub_1 + row_dir_add_1;
                let col = col_sub_1 + col_dir_add_1;
                let c = Coordinate { row, col };
                if !region.contains(&c) {
                    sides.insert(c);
                }
            }
        }
        let mut remove = HashSet::new();
        for coordinate in &sides {
            if let (Some(row_sub_1), Some(col_sub_1)) = (coordinate.row.checked_sub(1), coordinate.col.checked_sub(1)) {
                let row = row_sub_1 + row_dir_add_1;
                let col = col_sub_1 + col_dir_add_1;
                let mut c = Coordinate { row, col };
                while sides.contains(&c) {
                    remove.insert(c.clone());
                    let row = c.row + row_dir_add_1;
                    let col = c.col + col_dir_add_1;
                    c = Coordinate {row, col};
                }
            }
        }

        side_count += sides.len() - remove.len();
    }

    side_count
}

fn get_regions(map: &Map<char>) -> Vec<Region> {
    let mut results = Vec::new();

    let mut visited = HashSet::new();

    for row in 0..map.n_rows {
        for col in 0..map.n_cols {
            let coord = Coordinate { row, col };

            if let Some(region) = get_region(&coord, map, &mut visited) {
                results.push(region)
            }
        }
    }

    results
}

pub fn part_one(input: &str) -> Option<usize> {
    Map::parse(input, |x| x).map(|map| {
        get_regions(&map)
            .iter()
            .fold(0, |acc, r| acc + r.area * r.perimeter)
    })
}

pub fn part_two(input: &str) -> Option<usize> {
    Map::parse(input, |x| x).map(|map| {
        get_regions(&map)
            .iter()
            // .fold(0, |acc, r| acc + dbg!(r).area * get_sides(&r.all))
            .fold(0, |acc, r| acc + dbg!(r).area * count_sides(&r.edges, &map))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
