pub fn part_one(input: &str) -> Option<usize> {
    let rows = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut columns: Vec<Vec<u32>> = Vec::new();

    rows.iter().for_each(|row| {
        for (i, height) in row.iter().enumerate() {
            if columns.get_mut(i).is_none() {
                columns.push(vec![]);
            };

            columns.get_mut(i).unwrap().push(*height)
        }
    });

    let mut count = 2 * (columns.len() + rows.len()) - 4;

    (1..rows.len() - 1).for_each(|i| {
        rows.iter()
            .enumerate()
            .take(columns.len() - 1)
            .skip(1)
            .for_each(|(j, row)| {
                let left = row[..i].iter().max().unwrap();
                let right = row[i..].iter().skip(1).max().unwrap();
                let top = columns[i][..j].iter().max().unwrap();
                let bottom = columns[i][j..].iter().skip(1).max().unwrap();

                if left < &row[i] || right < &row[i] || top < &row[i] || bottom < &row[i] {
                    count += 1;
                }
            });
    });

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let rows = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut columns: Vec<Vec<u32>> = Vec::new();

    rows.iter().for_each(|row| {
        for (i, height) in row.iter().enumerate() {
            if columns.get_mut(i).is_none() {
                columns.push(vec![]);
            };

            columns.get_mut(i).unwrap().push(*height)
        }
    });

    let mut max = 0;

    (1..rows.len() - 1).for_each(|i| {
        rows.iter()
            .enumerate()
            .take(columns.len() - 1)
            .skip(1)
            .for_each(|(j, row)| {
                let left = match row[..i].iter().rev().position(|height| height >= &row[i]) {
                    Some(n) => n + 1,
                    None => row[..i].len(),
                };
                let right = match row[(i + 1)..].iter().position(|height| height >= &row[i]) {
                    Some(n) => n + 1,
                    None => row[(i + 1)..].len(),
                };
                let top = match columns[i][..j]
                    .iter()
                    .rev()
                    .position(|height| height >= &row[i])
                {
                    Some(n) => n + 1,
                    None => columns[i][..j].len(),
                };
                let bottom = match columns[i][(j + 1)..]
                    .iter()
                    .position(|height| height >= &row[i])
                {
                    Some(n) => n + 1,
                    None => columns[i][(j + 1)..].len(),
                };

                let view = left * right * top * bottom;

                if view > max {
                    max = view;
                }
            });
    });

    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
