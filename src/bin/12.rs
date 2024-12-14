use std::collections::{HashMap, HashSet};

use num::complex::Complex;

advent_of_code::solution!(12);

#[derive(Debug)]
struct Results {
    area: usize,
    perimeter: usize,
    sides: usize,
}

type ComplexMap = HashMap<Complex<i64>, char>;

fn parse_complex(input: &str) -> Option<ComplexMap> {
    let lines: Vec<&str> = input.trim().lines().collect();
    if lines.is_empty() {
        return None;
    }

    let n_rows = lines.len();
    let n_cols = lines[0].len();

    let mut data: HashMap<Complex<i64>, char> = lines
        .iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.chars()
                .enumerate()
                .map(move |(j, c)| (Complex::new(i as i64, j as i64), c))
        })
        .collect();

    for i_add_1 in 0..=n_rows + 1 {
        data.insert(Complex::new(i_add_1 as i64 - 1, n_cols as i64), '#');
        data.insert(Complex::new(i_add_1 as i64 - 1, -1), '#');
    }

    for j_add_1 in 0..=n_cols + 1 {
        data.insert(Complex::new(n_rows as i64, j_add_1 as i64 - 1), '#');
        data.insert(Complex::new(-1, j_add_1 as i64 - 1), '#');
    }

    Some(data)
}

fn search(
    map: &ComplexMap,
    coordinate: &Complex<i64>,
    visited: &mut HashSet<Complex<i64>>,
    current_value: char,
    direction: &Complex<i64>,
) -> Results {
    if map[coordinate] != current_value {
        if map[&(coordinate + direction * Complex::new(0, 1))] == current_value
            || map[&(coordinate - direction + direction * Complex::new(0, 1))] != current_value
        {
            return Results {
                area: 0,
                perimeter: 1,
                sides: 1,
            };
        } else {
            return Results {
                area: 0,
                perimeter: 1,
                sides: 0,
            };
        }
    }

    if visited.contains(coordinate) {
        return Results {
            area: 0,
            perimeter: 0,
            sides: 0,
        };
    }
    visited.insert(*coordinate);

    [
        Complex::new(1, 0),
        Complex::new(-1, 0),
        Complex::new(0, 1),
        Complex::new(0, -1),
    ]
    .iter()
    .fold(
        Results {
            area: 1,
            perimeter: 0,
            sides: 0,
        },
        |acc, x| {
            let r = search(map, &(coordinate + x), visited, current_value, x);
            Results {
                area: acc.area + r.area,
                perimeter: acc.perimeter + r.perimeter,
                sides: acc.sides + r.sides,
            }
        },
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    parse_complex(input).map(|map| {
        let mut visited = HashSet::new();
        map.iter()
            .filter_map(|(c, v)| {
                if !visited.contains(c) && map[c] != '#' {
                    Some(search(&map, c, &mut visited, *v, &Complex::new(1, 0)))
                } else {
                    None
                }
            })
            .fold(0, |acc, r| acc + r.area * r.perimeter)
    })
}

pub fn part_two(input: &str) -> Option<usize> {
    parse_complex(input).map(|map| {
        let mut visited = HashSet::new();
        map.iter()
            .filter_map(|(c, v)| {
                if !visited.contains(c) && map[c] != '#' {
                    Some(search(&map, c, &mut visited, *v, &Complex::new(1, 0)))
                } else {
                    None
                }
            })
            .fold(0, |acc, r| acc + r.area * r.sides)
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
