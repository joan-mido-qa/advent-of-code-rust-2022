pub fn part_one(input: &str) -> Option<u32> {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    return Some(
        input
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(a, b)| {
                a.chars()
                    .nth(a.find(|char| b.contains(char)).unwrap())
                    .unwrap()
            })
            .map(|letter| (letters.find(letter).unwrap() + 1) as u32)
            .sum(),
    );
}

pub fn part_two(input: &str) -> Option<u32> {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    return Some(
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|chunk| {
                chunk[0]
                    .chars()
                    .nth(
                        chunk[0]
                            .find(|char| chunk[1].contains(char) && chunk[2].contains(char))
                            .unwrap(),
                    )
                    .unwrap()
            })
            .map(|letter| (letters.find(letter).unwrap() + 1) as u32)
            .sum(),
    );
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
