use std::fs;

fn read_jolts(file: &str) -> Vec<Vec<u8>> {
    let file_content = fs::read_to_string(file).expect("Failed to read file");
    file_content
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn first_max_of_range(row: &Vec<u8>, from: usize, to: usize) -> (usize, u8) {
    let (i, v) = row[from..=to]
        .iter()
        .enumerate()
        .reduce(|best, (i, v)| if v > best.1 { (i, v) } else { best })
        .unwrap();
    (i + from, *v)
}

fn max_of_size(row: &Vec<u8>, size: usize) -> u64 {
    let n = row.len();

    let mut ans = 0u64;
    let mut left_i = 0;
    for i in 1..=size {
        ans *= 10;
        let (max_i, max_val) = first_max_of_range(row, left_i, n - size - 1 + i);
        ans += max_val as u64;
        left_i = max_i + 1;
    }

    ans
}

fn main() {
    let jolts = read_jolts("day3_real.txt");

    let mut part1 = 0;
    let mut part2 = 0;
    for row in jolts {
        part1 += max_of_size(&row, 2);
        part2 += max_of_size(&row, 12);
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
