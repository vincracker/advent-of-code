fn main() {
    println!("Part 2");
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
        // dbg!(&items);

        let is_increasing = items.windows(2).all(|window| window[0] < window[1]);
        let is_decreasing = items.windows(2).all(|window| window[0] > window[1]);

        if !is_increasing && !is_decreasing {
            let mut can_be_increasing = false;
            let mut can_be_decreasing = false;

            // Check by removing each element
            let mut modified = items.clone();

            for i in 0..items.len() {
                modified = items.clone();
                modified.remove(i); // Remove the element at index i
                if modified.windows(2).all(|window| window[0] < window[1]) {
                    if modified
                        .windows(2)
                        .all(|window| (window[0] - window[1]).abs() <= 3)
                    {
                        can_be_increasing = true;
                    }
                }

                if modified.windows(2).all(|window| window[0] > window[1]) {
                    if modified
                        .windows(2)
                        .all(|window| (window[0] - window[1]).abs() <= 3)
                    {
                        can_be_decreasing = true;
                    }
                }
            }

            if !can_be_increasing && !can_be_decreasing {
                result -= 1;
                dbg!(&modified);
                continue;
            }
        } else {
            let mut moved = false;
            for window in items.windows(2) {
                if (window[0] - window[1]).abs() > 3 {
                    if window[0] == items[0] || window[1] == items[items.len() - 1] {
                        if !moved {
                            moved = true;
                            continue;
                        };
                    }
                    result -= 1;
                    break;
                }
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
        assert_eq!(result, "4".to_string());
    }
}
