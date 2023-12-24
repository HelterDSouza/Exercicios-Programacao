advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let it = input
        .lines()
        .filter_map(|c| c.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    let output = it.windows(2).filter(|window| window[0] < window[1]).count() as u32;

    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let it = input
        .lines()
        .filter_map(|c| c.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let output = it
        .windows(4)
        .filter(|window| window[0] < window[4 - 1])
        .count() as u32;

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
