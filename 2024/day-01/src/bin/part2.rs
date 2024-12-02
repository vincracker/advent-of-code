
fn main() {
    println!("Part 2");
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

    let result: i32 = left.into_iter().map(|l| {
        let count:i32 = right.iter().filter(|&&x| x == l).count().try_into().unwrap();
        l * count
    }).sum();

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
        assert_eq!(result, "31".to_string());
    }
}
