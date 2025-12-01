advent_of_code::solution!(1);

fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().filter(|l| !l.is_empty()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    // Read string file input into a vector
    let lines = parse_lines(input);

    let mut count = 0; // number of zeros
    let mut current = 50; // starting

    for line in lines.iter() {
        // println!("{}", line)
        let dir = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

        // println!("dir = {}, amount = {}", dir, amount);

        if dir == 'L' {
            current = (current - amount).rem_euclid(100);
        } else {
            current = (current + amount).rem_euclid(100);
        }

        if current == 0 {
            count += 1;
        }
    }
    println!("count = {}", count);
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = parse_lines(input);

    let mut count: u64 = 0; // number of zeros
    let mut current: i32 = 50; // starting

    for line in lines {
        let dir = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

        let step = if dir == 'L' { -1 } else { 1 };

        // go 1 by 1
        for _ in 0..amount {
            current = (current + step).rem_euclid(100);
            if current == 0 {
                count += 1;
            }
        }
    }

    println!("count = {}", count);

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1180));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6892));
    }
}
