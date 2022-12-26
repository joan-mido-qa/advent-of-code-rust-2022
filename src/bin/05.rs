pub fn part_one(input: &str) -> Option<String> {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();

    let mut output = String::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    inputs[0].lines().rev().for_each(|line| {
        for (i, n) in (1..line.len()).step_by(4).enumerate() {
            let c = line.chars().nth(n).unwrap();

            if stacks.get_mut(i).is_none() {
                stacks.push(vec![]);
            }

            if !c.eq(&' ') && !c.is_numeric() {
                stacks.get_mut(i).unwrap().push(c);
            };
        }
    });

    inputs[1]
        .lines()
        .map(|line| {
            let process = line
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            (process[0], process[1], process[2])
        })
        .for_each(|(n, a, b)| {
            for _i in 0..n {
                let tmp = stacks[a - 1].pop().unwrap();

                stacks[b - 1].push(tmp);
            }
        });

    stacks
        .iter()
        .for_each(|stack| output.push(*stack.last().unwrap()));

    Some(output)
}

pub fn part_two(input: &str) -> Option<String> {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();

    let mut output = String::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    inputs[0].lines().rev().for_each(|line| {
        for (i, n) in (1..line.len()).step_by(4).enumerate() {
            let c = line.chars().nth(n).unwrap();

            if stacks.get_mut(i).is_none() {
                stacks.push(vec![]);
            }

            if !c.eq(&' ') && !c.is_numeric() {
                stacks.get_mut(i).unwrap().push(c);
            };
        }
    });

    inputs[1]
        .lines()
        .map(|line| {
            let process = line
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            (process[0], process[1], process[2])
        })
        .for_each(|(n, a, b)| {
            let len = stacks[a - 1].len() - n;
            let tmp = stacks[a - 1].drain(len..).collect::<Vec<char>>();

            stacks[b - 1].extend(tmp);
        });

    stacks
        .iter()
        .for_each(|stack| output.push(*stack.last().unwrap()));

    Some(output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
