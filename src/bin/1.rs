use std::collections::HashSet;

use aoc2016::utils;

fn main() {
    let input = utils::string_from_file("inputs/1");
    println!("Part 1: {} blocks away", part1(&input));
    println!("Part 2: {} blocks away", part2(&input));
}

#[derive(Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn part1(i: &str) -> u32 {
    let mut direction: Direction = Direction::N;
    let mut x = 0;
    let mut y = 0;

    i.split(", ").for_each(|s| {
        let mut char_iter = s.trim().chars();
        let dir = char_iter.next().expect("expected at least one");
        match dir {
            'R' => {
                direction = {
                    let ref mut this = direction;
                    match this {
                        Direction::N => Direction::E,
                        Direction::E => Direction::S,
                        Direction::S => Direction::W,
                        Direction::W => Direction::N,
                    }
                }
            }
            'L' => {
                direction = {
                    let ref mut this = direction;
                    match this {
                        Direction::N => Direction::W,
                        Direction::E => Direction::N,
                        Direction::S => Direction::E,
                        Direction::W => Direction::S,
                    }
                }
            }
            _ => panic!("unexpected rotation"),
        };
        let d: i32 = char_iter.as_str().parse().unwrap();

        match direction {
            Direction::N => y += d,
            Direction::E => x += d,
            Direction::S => y -= d,
            Direction::W => x -= d,
        }
    });

    x.unsigned_abs() + y.unsigned_abs()
}

#[test]
fn example1() {
    assert_eq!(part1("R2, L3"), 5);
}

#[test]
fn example2() {
    assert_eq!(part1("R2, R2, R2"), 2);
}

#[test]
fn example3() {
    assert_eq!(part1("R5, L5, R5, R3"), 12);
}

fn part2(i: &str) -> u32 {
    let mut direction: Direction = Direction::N;
    let mut x = 0;
    let mut y = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut answer: Option<u32> = None;
    visited.insert((x, y));

    i.split(", ").for_each(|s| {
        let mut char_iter = s.trim().chars();
        let dir = char_iter.next().expect("expected at least one");
        match dir {
            'R' => {
                direction = {
                    let ref mut this = direction;
                    match this {
                        Direction::N => Direction::E,
                        Direction::E => Direction::S,
                        Direction::S => Direction::W,
                        Direction::W => Direction::N,
                    }
                }
            }
            'L' => {
                direction = {
                    let ref mut this = direction;
                    match this {
                        Direction::N => Direction::W,
                        Direction::E => Direction::N,
                        Direction::S => Direction::E,
                        Direction::W => Direction::S,
                    }
                }
            }
            _ => panic!("unexpected rotation"),
        };
        let d: i32 = char_iter.as_str().parse().expect("expected int");

        (0..d).into_iter().for_each(|_| {
            match direction {
                Direction::N => y += 1,
                Direction::E => x += 1,
                Direction::S => y -= 1,
                Direction::W => x -= 1,
            }
            if visited.contains(&(x, y)) && answer == None {
                println!("first location twice: ({}, {})", x, y);
                answer = Some(x.unsigned_abs() + y.unsigned_abs());
            }
            visited.insert((x, y));
        });

        // dbg!(&visited);
    });

    answer.expect("expected answer")
}

#[test]
fn example4() {
    assert_eq!(part2("R8, R4, R4, R8"), 4);
}
