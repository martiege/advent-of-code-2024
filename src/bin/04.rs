use std::iter::zip;

advent_of_code::solution!(4);

const SEARCH_WORD: &str = "XMAS";
const SEARCH_WORD_REVERSED: &str = "SAMX";

type SubWordSlice = [usize; 4];
type WordSlice = (SubWordSlice, SubWordSlice);

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn negative_word_range(i: usize) -> Option<SubWordSlice> {
    if i + 1 < SEARCH_WORD.len() {
        None
    } else {
        Some(
            (i + 1 - SEARCH_WORD.len()..i + 1)
                .rev()
                .collect::<Vec<usize>>()
                .try_into()
                .expect("This should always fit!"),
        )
    }
}

fn positive_word_range(i: usize, max_value: usize) -> Option<SubWordSlice> {
    if i + SEARCH_WORD.len() <= max_value {
        Some(
            (i..i + SEARCH_WORD.len())
                .collect::<Vec<usize>>()
                .try_into()
                .expect("This should always fit!"),
        )
    } else {
        None
    }
}

fn slice_left_down_diagonal(row: usize, col: usize, max_value: usize) -> Option<WordSlice> {
    Some((
        positive_word_range(row, max_value)?,
        negative_word_range(col)?,
    ))
}

fn slice_left(row: usize, col: usize) -> Option<WordSlice> {
    Some(([row; 4], negative_word_range(col)?))
}

fn slice_left_up_diagonal(row: usize, col: usize) -> Option<WordSlice> {
    Some((negative_word_range(row)?, negative_word_range(col)?))
}

fn slice_up(row: usize, col: usize) -> Option<WordSlice> {
    Some((negative_word_range(row)?, [col; 4]))
}

fn slices(row: usize, col: usize, max_value: usize) -> [Option<WordSlice>; 4] {
    [
        slice_left_down_diagonal(row, col, max_value),
        slice_left(row, col),
        slice_left_up_diagonal(row, col),
        slice_up(row, col),
    ]
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);

    let mut count = 0;

    for row in 0..data.len() {
        for col in 0..data[row].len() {
            for (row_slice, col_slice) in slices(row, col, data[row].len()).into_iter().flatten() {
                let word = zip(row_slice, col_slice)
                    .fold(String::new(), |acc, (r, c)| acc + &data[r][c].to_string());

                if word == SEARCH_WORD || word == SEARCH_WORD_REVERSED {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

fn mtc(c1: char, c2: char) -> bool {
    (c1.to_ascii_lowercase() == 'm' && c2.to_ascii_lowercase() == 's')
        || (c1.to_ascii_lowercase() == 's' && c2.to_ascii_lowercase() == 'm')
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);

    let mut count = 0;

    for row in 1..data.len() - 1 {
        for col in 1..data[row].len() - 1 {
            if data[row][col].to_ascii_lowercase() != 'a' {
                continue;
            }

            let tl = data[row - 1][col - 1];
            let tr = data[row - 1][col + 1];
            let bl = data[row + 1][col - 1];
            let br = data[row + 1][col + 1];

            if mtc(tl, br) && mtc(bl, tr) {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
