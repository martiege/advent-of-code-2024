use std::cmp::min;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

advent_of_code::solution!(13);

#[derive(Debug)]
struct ButtonSettings {
    dx: usize,
    dy: usize,
}

#[derive(Debug)]
struct PrizeLocation {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Machine {
    a: ButtonSettings,
    b: ButtonSettings,
    p: PrizeLocation,
}

fn parse_button_coordinate_x(input: &str) -> IResult<&str, usize> {
    preceded(tag("X+"), map_res(digit1, |s: &str| s.parse::<usize>()))(input)
}

fn parse_button_coordinate_y(input: &str) -> IResult<&str, usize> {
    preceded(tag("Y+"), map_res(digit1, |s: &str| s.parse::<usize>()))(input)
}

fn parse_button_a(input: &str) -> IResult<&str, ButtonSettings> {
    map(
        preceded(
            tag("Button A: "),
            separated_pair(
                parse_button_coordinate_x,
                tag(", "),
                parse_button_coordinate_y,
            ),
        ),
        |(dx, dy)| ButtonSettings { dx, dy },
    )(input)
}

fn parse_button_b(input: &str) -> IResult<&str, ButtonSettings> {
    map(
        preceded(
            tag("Button B: "),
            separated_pair(
                parse_button_coordinate_x,
                tag(", "),
                parse_button_coordinate_y,
            ),
        ),
        |(dx, dy)| ButtonSettings { dx, dy },
    )(input)
}

fn parse_prize_coordinate_x(input: &str) -> IResult<&str, usize> {
    preceded(tag("X="), map_res(digit1, |s: &str| s.parse::<usize>()))(input)
}

fn parse_prize_coordinate_y(input: &str) -> IResult<&str, usize> {
    preceded(tag("Y="), map_res(digit1, |s: &str| s.parse::<usize>()))(input)
}

fn parse_prize_location(input: &str) -> IResult<&str, PrizeLocation> {
    map(
        preceded(
            tag("Prize: "),
            separated_pair(
                parse_prize_coordinate_x,
                tag(", "),
                parse_prize_coordinate_y,
            ),
        ),
        |(x, y)| PrizeLocation { x, y },
    )(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map(
        separated_pair(
            separated_pair(parse_button_a, tag("\n"), parse_button_b),
            tag("\n"),
            parse_prize_location,
        ),
        |((a, b), p)| Machine { a, b, p },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(multispace1, parse_machine)(input)
}

// x = [n_a, n_b]^T
// c = [-1, -1]
// Ax <= b
// A = [
// [ dx_a, dx_b ]
// [ dy_a, dy_b ]
// ]
// b = [p_x, p_y]^T
//
fn naive_find_minimum_token(machine: &Machine) -> Option<usize> {
    let max_a = min(machine.p.x / machine.a.dx, machine.p.y / machine.a.dy) + 1;
    let max_b = min(machine.p.x / machine.b.dx, machine.p.y / machine.b.dy) + 1;

    let mut min_tokens = None;

    for a in 0..min(max_a, 100) {
        for b in 0..min(max_b, 100) {
            let x = machine.a.dx * a + machine.b.dx * b;
            let y = machine.a.dy * a + machine.b.dy * b;

            if x == machine.p.x && y == machine.p.y {
                let new_tokens = 3 * a + b;
                if let Some(tokens) = min_tokens {
                    min_tokens = Some(min(tokens, new_tokens));
                } else {
                    min_tokens = Some(new_tokens);
                }
            }
        }
    }

    min_tokens
}

fn improved_find_minimum_token(machine: &Machine) -> Option<usize> {
    let ax = machine.a.dx as i64;
    let ay = machine.a.dy as i64;
    let bx = machine.b.dx as i64;
    let by = machine.b.dy as i64;
    let px = machine.p.x as i64;
    let py = machine.p.y as i64;

    // TODO: why does this work while a
    // checked, unsigned version does not?
    let d = ax * by - ay * bx;
    let di = px * by - py * bx;
    let dj = py * ax - px * ay;

    if di % d == 0 && dj % d == 0 {
        Some((3 * di / d + dj / d) as usize)
    } else {
        None
    }
    // // singular
    // // let adx_bdy = machine.a.dx.checked_mul(machine.b.dy).unwrap();
    // // let ady_bdx = machine.b.dx.checked_mul(machine.a.dy).unwrap();
    // let adx_bdy = machine.a.dx * machine.b.dy;
    // let ady_bdx = machine.a.dy * machine.b.dx;
    // let determinant = adx_bdy.abs_diff(ady_bdx);
    // if determinant == 0 {
    //     println!("det");
    //     return None;
    // }
    //
    // // let px_bdy = machine.p.x.checked_mul(machine.b.dy).unwrap();
    // // let py_bdx = machine.p.y.checked_mul(machine.b.dx).unwrap();
    // let px_bdy = machine.p.x * machine.b.dy;
    // let py_bdx = machine.p.y * machine.b.dx;
    //
    // let n_a_diff = px_bdy.abs_diff(py_bdx);
    // if n_a_diff % determinant != 0 {
    //     println!("n_a");
    //     return None;
    // }
    //
    // // let n_a = n_a_diff.checked_div(determinant).unwrap();
    // let n_a = n_a_diff / determinant;
    //
    // // let px_ady = machine.p.x.checked_mul(machine.a.dy).unwrap();
    // // let py_ady = machine.p.y.checked_mul(machine.a.dy).unwrap();
    // let px_ady = machine.p.x * machine.a.dy;
    // let py_ady = machine.p.y * machine.a.dy;
    //
    // let n_b_diff = px_ady.abs_diff(py_ady);
    // if n_b_diff % determinant != 0 {
    //     println!("n_b");
    //     return None;
    // }
    //
    // // let n_b = n_b_diff.checked_div(determinant).unwrap();
    // let n_b = n_b_diff / determinant;
    //
    // Some(3 * n_a + n_b)
}

pub fn part_one(input: &str) -> Option<usize> {
    if let Ok((_rest, data)) = parse(input) {
        Some(
            data
            .iter()
            // .filter_map(naive_find_minimum_token)
            .filter_map(improved_find_minimum_token)
            .sum()
        )
    }
    else {
        None
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let offset = 10000000000000;
    if let Ok((_rest, data)) = parse(input) {
        Some(
            data
            .iter()
            .map(|Machine {a, b, p}| Machine {
                    a: ButtonSettings {dx: a.dx, dy: a.dy},
                    b: ButtonSettings {dx: b.dx, dy: b.dy},
                    // p: PrizeLocation {x: p.x + offset, y: p.y + offset}
                    p: PrizeLocation {x: p.x.checked_add(offset).unwrap(), y: p.y.checked_add(offset).unwrap()}
                })
            .filter_map(|machine| improved_find_minimum_token(&machine))
            .sum()
        )
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
