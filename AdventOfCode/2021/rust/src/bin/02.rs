use std::{collections::HashMap, process::Output, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug)]
enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((direction, distance)) = s.split_once(' ') {
            let distance = distance.parse()?;

            let result = match direction {
                "forward" => Direction::Forward(distance),
                "down" => Direction::Down(distance),
                "up" => Direction::Up(distance),
                _ => panic!("Invalid direction"),
            };
            Ok(result)
        } else {
            Err(anyhow::format_err!("Could not split direction"))
        }
    }
}

#[derive(Debug)]
struct Location {
    distance: u32,
    depth: u32,
    aim: u32,
}

impl Location {
    pub fn new() -> Self {
        Self {
            distance: 0,
            depth: 0,
            aim: 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Direction>>()
        .iter()
        .fold(Location::new(), |location, direction| match direction {
            Direction::Forward(distance) => Location {
                distance: location.distance + distance,
                ..location
            },
            Direction::Down(depth) => Location {
                depth: location.depth + depth,
                ..location
            },
            Direction::Up(depth) => Location {
                depth: location.depth - depth,
                ..location
            },
        });
    println!("{:?}", output);
    Some(output.depth * output.distance)
}
// pub fn part_one(input: &str) -> Option<i32> {
//     let mut hm: HashMap<&str, i32> = HashMap::new();
//     input.lines().for_each(|command: &str| {
//         let line = command.split_whitespace().take(2).collect::<Vec<&str>>();
//         match line[0] {
//             "up" => {
//                 hm.entry("y")
//                     .and_modify(|counter| *counter -= line[1].parse::<i32>().unwrap())
//                     .or_insert(line[1].parse::<i32>().unwrap());
//             }
//             "down" => {
//                 hm.entry("y")
//                     .and_modify(|counter| *counter += line[1].parse::<i32>().unwrap())
//                     .or_insert(line[1].parse::<i32>().unwrap());
//             }
//             "forward" => {
//                 hm.entry("x")
//                     .and_modify(|counter| *counter += line[1].parse::<i32>().unwrap())
//                     .or_insert(line[1].parse::<i32>().unwrap());
//             }
//             _ => {}
//         }
//     });
//     Some(hm.get("x").unwrap() * hm.get("y").unwrap())
// }

pub fn part_two(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Direction>>()
        .iter()
        .fold(Location::new(), |location, direction| match direction {
            Direction::Forward(distance) => Location {
                distance: location.distance + distance,
                depth: location.depth + (location.aim * distance),
                ..location
            },
            Direction::Down(aim) => Location {
                aim: location.aim + aim,
                ..location
            },
            Direction::Up(aim) => Location {
                aim: location.aim - aim,
                ..location
            },
        });
    println!("{:?}", output);
    Some(output.depth * output.distance)
}
// pub fn part_two(input: &str) -> Option<i32> {
//     let mut hm: HashMap<&str, i32> = HashMap::new();
//     input.lines().for_each(|command: &str| {
//         let line = command.split_whitespace().take(2).collect::<Vec<&str>>();
//         match line[0] {
//             "up" => {
//                 hm.entry("aim")
//                     .and_modify(|counter| *counter -= line[1].parse::<i32>().unwrap())
//                     .or_insert(line[1].parse::<i32>().unwrap());
//             }
//             "down" => {
//                 hm.entry("aim")
//                     .and_modify(|counter| *counter += line[1].parse::<i32>().unwrap())
//                     .or_insert(line[1].parse::<i32>().unwrap());
//             }
//             "forward" => {
//                 let aim = *hm.get("aim").unwrap_or(&0);
//                 hm.entry("x")
//                     .and_modify(|counter| *counter += line[1].parse::<i32>().unwrap())
//                     .or_insert(line[1].parse::<i32>().unwrap());
//                 hm.entry("y")
//                     .and_modify(|counter| *counter += aim * line[1].parse::<i32>().unwrap())
//                     .or_insert(0);
//             }
//             _ => {}
//         }
//     });
//     println!("{:?}", hm);
//     Some(hm.get("x").unwrap() * hm.get("y").unwrap())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(900));
    }
}
