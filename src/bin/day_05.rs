use std::cmp::max;
use std::cmp::min;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Range {
    from: u64,
    to: u64,
}

impl Range {
    fn contains(&self, id: u64) -> bool {
        self.from <= id && id <= self.to
    }

    fn intersects(&self, other: &Range) -> bool {
        let left = if self.from < other.from { self } else { other };
        let right = if self.from > other.from { self } else { other };
        left.to >= right.from
    }

    fn merge(&self, other: &Range) -> Range {
        let from = min(self.from, other.from);
        let to = max(self.to, other.to);
        Range { from: from, to: to }
    }
}

fn read_ranges(file: &str) -> (Vec<Range>, Vec<u64>) {
    let file_content = fs::read_to_string(file).expect("Failed to read file");
    let mut ranges: Vec<Range> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    file_content
        .lines()
        .filter(|line| line.trim().len() > 0)
        .for_each(|line| {
            if line.contains("-") {
                let (from, to) = line.split_once("-").expect("Couldn't parse");
                ranges.push(Range {
                    from: from.trim().parse::<u64>().unwrap(),
                    to: to.trim().parse::<u64>().unwrap(),
                });
            } else {
                ids.push(line.trim().parse::<u64>().unwrap());
            }
        });

    (ranges, ids)
}

fn solve_2(ranges: &Vec<Range>) -> u64 {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|r| r.from);

    let mut merged_ranges: Vec<Range> = Vec::new();
    let mut i = 0;
    while i < sorted_ranges.len() {
        let mut cur_root = sorted_ranges[i];
        i += 1;

        while i < sorted_ranges.len() && cur_root.intersects(&sorted_ranges[i]) {
            cur_root = cur_root.merge(&sorted_ranges[i]);
            i += 1;
        }
        merged_ranges.push(cur_root);
    }

    merged_ranges.iter().map(|r| r.to - r.from + 1).sum()
}

fn main() {
    let (ranges, ids) = read_ranges("day5_real.txt");

    let mut part_1 = 0;
    for id in ids {
        part_1 += ranges.iter().any(|r| r.contains(id)) as u16;
    }

    let part_2 = solve_2(&ranges);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
