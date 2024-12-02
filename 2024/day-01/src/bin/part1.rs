use std::iter::zip;

fn main() {
    println!("Part 1");
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let mut items = line.split_ascii_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();

    let result: i32 = zip(left, right).map(|(l, r)| (l - r).abs()).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, "11".to_string());
    }
}
