use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Coordinate {
    row: i128,
    col: i128,
}

fn parse(input: &str) -> (Coordinate, HashMap<char, Vec<Coordinate>>) {
    let mut size = Coordinate { row: 0, col: 0 };

    let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();

    let lines: Vec<&str> = input.trim().lines().collect();

    size.row = lines.len() as i128;
    for (row, line) in lines.into_iter().enumerate() {
        size.col = line.len() as i128;
        for (col, c) in line.trim().chars().enumerate() {
            if c != '.' {
                if let Some(v) = antennas.get_mut(&c) {
                    v.push(Coordinate {
                        row: row as i128,
                        col: col as i128,
                    })
                } else {
                    antennas.insert(
                        c,
                        vec![Coordinate {
                            row: row as i128,
                            col: col as i128,
                        }],
                    );
                }
            }
        }
    }

    (size, antennas)
}

fn in_board(antenna: &Coordinate, size: &Coordinate) -> bool {
    antenna.row >= 0 && antenna.col >= 0 && antenna.row < size.row && antenna.col < size.col
}

fn validate(antenna: Coordinate, size: &Coordinate) -> Option<Coordinate> {
    if in_board(&antenna, size) {
        Some(antenna)
    } else {
        None
    }
}

fn nearest_antinodes(antennas: &[&Coordinate], size: &Coordinate) -> Vec<Option<Coordinate>> {
    let d = Coordinate {
        row: antennas[1].row - antennas[0].row,
        col: antennas[1].col - antennas[0].col,
    };

    let a1 = Coordinate {
        row: antennas[1].row + d.row,
        col: antennas[1].col + d.col,
    };
    let a2 = Coordinate {
        row: antennas[0].row - d.row,
        col: antennas[0].col - d.col,
    };

    vec![validate(a1, size), validate(a2, size)]
}

fn line_antinodes(antennas: &[&Coordinate], size: &Coordinate) -> Vec<Option<Coordinate>> {
    let d = Coordinate {
        row: antennas[1].row - antennas[0].row,
        col: antennas[1].col - antennas[0].col,
    };

    let mut results = vec![Some(antennas[0].clone()), Some(antennas[1].clone())];

    let mut result = Some(antennas[1].clone());
    while let Some(r) = result {
        let r = Coordinate {
            row: r.row + d.row,
            col: r.col + d.col,
        };
        result = validate(r, size);
        results.push(result.clone());
    }

    let mut result = Some(antennas[0].clone());
    while let Some(r) = result {
        let r = Coordinate {
            row: r.row - d.row,
            col: r.col - d.col,
        };
        result = validate(r, size);
        results.push(result.clone());
    }

    results
}

fn all_antinodes(
    antennas: &[Coordinate],
    size: &Coordinate,
    f: fn(&[&Coordinate], &Coordinate) -> Vec<Option<Coordinate>>,
) -> Vec<Coordinate> {
    antennas
        .iter()
        .permutations(2)
        .flat_map(|ants| f(&ants, size))
        .flatten()
        .unique()
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (size, data) = parse(input);

    Some(
        data.values()
            .flat_map(|v| all_antinodes(v, &size, nearest_antinodes))
            .unique()
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (size, data) = parse(input);

    let result = data
        .values()
        .flat_map(|v| all_antinodes(v, &size, line_antinodes))
        .unique();

    Some(result.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
