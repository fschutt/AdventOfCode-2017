const INPUT: &str = include_str!("input.txt");

fn main() {
    let input_numbers: Vec<u8> =
        INPUT.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let iter1 = input_numbers.iter().cycle().peekable();
    let iter2 = iter1.clone().skip(1);

    let result: u64 =
        iter1.zip(iter2)
        .take(input_numbers.len())
        .filter(|&(i1, i2)| *i1 == *i2)
        .map(|(i1, _)| *i1 as u64)
        .sum();

    println!("{}", result);
}
