const INPUT: &str = include_str!("input.txt");

fn part1() {
    let input_numbers: Vec<Vec<u32>> = INPUT
        .lines()
        .collect::<Vec<&str>>()
        .iter().map(|line|
            line
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect()
        ).collect();

    let out: u32 = input_numbers.iter().map(|line| {
        let min = line.iter().min().unwrap();
        let max = line.iter().max().unwrap();
        max - min
    }).sum();

    println!("part 1: {}", out);
}

fn part2() {
    let input_numbers: Vec<Vec<u32>> = INPUT
        .lines()
        .collect::<Vec<&str>>()
        .iter().map(|line|
            line
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect()
        ).collect();

    let out: u32 = input_numbers.iter().map(|line| {
        // O(n log(n)) - there are better ways
        let mut max_diff = 0;
        for (idx, val) in line.iter().enumerate() {
            for other_idx in 0..idx {
                let other_val = line[other_idx];
                let max = val.max(&other_val);
                let min = val.min(&other_val);
                if max % min == 0 {
                    let diff = max / min;
                    if diff > max_diff {
                        max_diff = diff;
                    }
                }
            }
        }

        max_diff
    }).sum();

    println!("{:#?}", out);
}

fn main() {
    part1();
    part2();
}
