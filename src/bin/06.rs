use std::{collections::HashSet, thread::spawn};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

advent_of_code::solution!(6);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct DirectedPosition {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Board {
    size: Position,
    guard: DirectedPosition,
    obstacles: HashSet<Position>,
}

fn next_position(guard: &DirectedPosition, size: Position) -> Option<Position> {
    match guard.direction {
        Direction::Up => {
            if guard.position.row == 0 {
                None
            } else {
                Some(Position {
                    row: guard.position.row - 1,
                    col: guard.position.col,
                })
            }
        }
        Direction::Right => {
            if guard.position.col == size.col - 1 {
                None
            } else {
                Some(Position {
                    row: guard.position.row,
                    col: guard.position.col + 1,
                })
            }
        }
        Direction::Down => {
            if guard.position.row == size.row - 1 {
                None
            } else {
                Some(Position {
                    row: guard.position.row + 1,
                    col: guard.position.col,
                })
            }
        }
        Direction::Left => {
            if guard.position.col == 0 {
                None
            } else {
                Some(Position {
                    row: guard.position.row,
                    col: guard.position.col - 1,
                })
            }
        }
    }
}

fn next_direction(
    direction: &Direction,
    next_position: &Position,
    obstacles: &HashSet<Position>,
) -> Option<Direction> {
    if obstacles.contains(next_position) {
        match direction {
            Direction::Up => Some(Direction::Right),
            Direction::Right => Some(Direction::Down),
            Direction::Down => Some(Direction::Left),
            Direction::Left => Some(Direction::Up),
        }
    } else {
        None
    }
}

fn parse(input: &str) -> Option<Board> {
    let mut guard = None;
    let mut obstacles = HashSet::new();

    let lines: Vec<&str> = input.lines().collect();
    let n_rows = lines.len();
    let mut n_cols = None;

    for (row, line) in lines.into_iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();

        if let Some(n_cols) = n_cols {
            assert!(n_cols == chars.len());
        } else {
            n_cols = Some(chars.len());
        }

        for (col, c) in line.chars().enumerate() {
            let position = Position { row, col };
            match c {
                '^' => {
                    guard = Some(DirectedPosition {
                        position,
                        direction: Direction::Up,
                    });
                }
                '>' => {
                    guard = Some(DirectedPosition {
                        position,
                        direction: Direction::Right,
                    });
                }
                'v' => {
                    guard = Some(DirectedPosition {
                        position,
                        direction: Direction::Down,
                    });
                }
                '<' => {
                    guard = Some(DirectedPosition {
                        position,
                        direction: Direction::Left,
                    });
                }
                '#' => {
                    obstacles.insert(position);
                }
                _ => {}
            }
        }
    }

    if let (Some(guard), Some(n_cols)) = (guard, n_cols) {
        let size = Position {
            row: n_rows,
            col: n_cols,
        };

        Some(Board {
            size,
            guard,
            obstacles,
        })
    } else {
        None
    }
}

enum VisitedPositions {
    Loops(HashSet<DirectedPosition>),
    Finite(HashSet<DirectedPosition>),
}

fn follow_guard(board: &Board) -> VisitedPositions {
    let mut guard = board.guard.clone();
    let mut positions = HashSet::new();

    positions.insert(guard.clone());

    while let Some(next_position) = next_position(&guard, board.size) {
        let next_guard = DirectedPosition {
            position: next_position,
            direction: guard.direction,
        };
        if positions.contains(&next_guard) {
            return VisitedPositions::Loops(positions);
        }

        if let Some(next_direction) =
            next_direction(&guard.direction, &next_position, &board.obstacles)
        {
            guard.direction = next_direction;
        } else {
            guard.position = next_position;
            positions.insert(guard.clone());
        }
    }

    VisitedPositions::Finite(positions)
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = parse(input);

    if let Some(board) = board {
        if let VisitedPositions::Finite(positions) = follow_guard(&board) {
            Some(
                positions
                    .into_iter()
                    .map(|x| x.position)
                    .collect::<HashSet<_>>()
                    .len() as u32,
            )
        } else {
            None
        }
    } else {
        None
    }
}

fn find_loops(board: Board) -> Option<u32> {
    if let VisitedPositions::Finite(positions) = follow_guard(&board) {
        Some(
            positions
                .into_iter()
                .unique_by(|x| x.position)
                .collect_vec()
                .par_iter()
                .fold(
                    || 0,
                    |acc, x| {
                        // acc
                        let mut proposed_board = board.clone();
                        proposed_board.obstacles.insert(x.position);

                        if let VisitedPositions::Loops(_positions) = follow_guard(&proposed_board) {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                )
                .sum::<u32>(),
        )
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = parse(input);

    if let Some(board) = board {
        find_loops(board)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
