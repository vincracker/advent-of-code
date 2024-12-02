fn main() {
    println!("Part 1");
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut result: i32 = 0;
    
    for line in input.lines() {
        result += 1;

        let items: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        dbg!(&items);

        let is_increasing = items.windows(2).all(|window| window[0] < window[1]);
        let is_decreasing = items.windows(2).all(|window| window[0] > window[1]);

        if !is_increasing && !is_decreasing {
            result -= 1;
            dbg!(&items);

            continue;
        }

        for window in items.windows(2) {
            if (window[0] - window[1]).abs() > 3 {
                result -= 1;
                break;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, "2".to_string());
    }
}
