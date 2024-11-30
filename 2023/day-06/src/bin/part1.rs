fn main() {
    println!("Part 1");
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let races_vec: Vec<&str> = input.split(",").collect();
    let mut answer: u128 = 1;

    for race in races_vec.iter() {
        let race_info: Vec<&str> = race.split(" ").collect();
        // dbg!(race_info);
        let time: u128 = race_info[0].parse().expect("Should be number");
        // dbg!(time);
        let distance: u128 = race_info[1].parse().expect("Should be number");
        // dbg!(distance);

        let mut win_way: u128 = 0;
        let mut hold_time: u128 = 0;

        while hold_time != time {
            if hold_time * (time - hold_time) > distance {
                win_way += 1;
            }

            hold_time += 1;
        }
        dbg!(win_way);
        answer *= win_way;
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("7 9,15 40,30 200");
        assert_eq!(result, "288".to_string());
    }
}
