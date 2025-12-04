use std::fs;

struct Range {
    from: u64,
    to: u64,
}

fn read_ranges(file: &str) -> Vec<Range> {
    let file_content = fs::read_to_string(file).expect("Failed to read file");
    file_content
        .split(",")
        .map(|line| {
            let (from, to) = line.split_once("-").expect("Invalid range");
            Range {
                from: from.trim().parse::<u64>().unwrap(),
                to: to.trim().parse::<u64>().unwrap(),
            }
        })
        .collect()
}

fn is_invalid(number: u64) -> bool {
    let num_str = number.to_string();
    if num_str.len() % 2 == 1 {
        return false;
    }

    let half = num_str.len() / 2;
    let (first, second) = (&num_str[..half], &num_str[half..]);
    first == second
}

fn is_invalid_2(number: u64) -> bool {
    (1..=6).any(|size| is_repeating(number, size))
}

fn is_repeating(number: u64, size: usize) -> bool {
    let num_str = number.to_string();
    let bytes = num_str.as_bytes();
    if bytes.len() <= size || bytes.len() % size != 0 {
        return false;
    }

    let pattern = &bytes[..size];
    bytes.chunks(size).skip(1).all(|chunk| chunk == pattern)
}

fn main() {
    let ranges = read_ranges("day2_real.txt");

    let mut part_1 = 0u64;
    let mut part_2 = 0u64;
    for Range { from, to } in ranges {
        for i in from..=to {
            if is_invalid(i) {
                part_1 += i;
            }
            if is_invalid_2(i) {
                part_2 += i;
            }
        }
    }
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
