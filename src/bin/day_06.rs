use std::fs;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
struct Equation {
    nums: Vec<u64>,
    op: Op,
}

fn read_equations(file: &str) -> Vec<Equation> {
    let file_content = fs::read_to_string(file).expect("Failed to read file");
    let words: Vec<Vec<&str>> = file_content
        .lines()
        .map(|line| line.trim().split_whitespace().collect())
        .collect();

    let num_eqs = words[0].len();
    let num_els = words.len();
    let mut eqs: Vec<Equation> = Vec::new();
    for e in 0..num_eqs {
        let nums = (0..num_els - 1)
            .map(|i| words[i][e].parse::<u64>().unwrap())
            .collect();
        let op = match words[num_els - 1][e] {
            "+" => Op::Add,
            _ => Op::Mul,
        };
        eqs.push(Equation { nums, op });
    }
    eqs
}

fn read_equations_2(file: &str) -> Vec<Equation> {
    let file_content = fs::read_to_string(file).expect("Failed to read file");
    let words: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // transpose
    let mut words_t: Vec<Vec<char>> = Vec::new();
    for j in 0..words[0].len() {
        words_t.push(Vec::new());
        for i in 0..words.len() {
            words_t[j].push(words[i][j]);
        }
    }

    let mut eqs: Vec<Equation> = Vec::new();
    let mut i = 0;
    while i < words_t.len() {
        let op = match words_t[i][words_t[i].len() - 1] {
            '+' => Op::Add,
            _ => Op::Mul,
        };
        let mut nums: Vec<u64> = Vec::new();
        while i < words_t.len() {
            let mut cur = 0u64;
            for j in 0..words_t[i].len() {
                if words_t[i][j].is_digit(10) {
                    cur = cur * 10 + (words_t[i][j].to_digit(10).unwrap() as u64);
                }
            }
            i += 1;
            if cur != 0 {
                nums.push(cur.clone());
            } else {
                break;
            }
        }
        eqs.push(Equation { nums, op });
    }
    eqs
}

fn process(eqs: &Vec<Equation>) -> u64 {
    let mut ret = 0;
    for eq in eqs {
        let mut ans = match eq.op {
            Op::Add => 0,
            Op::Mul => 1,
        };
        for num in &eq.nums {
            ans = match eq.op {
                Op::Add => ans + num,
                Op::Mul => ans * num,
            };
        }
        ret += ans;
    }
    ret
}

fn main() {
    let file = "day6_real.txt";
    let eqs = read_equations(file);
    let eqs_2 = read_equations_2(file);

    println!("Part 1: {}", process(&eqs));
    println!("Part 2: {}", process(&eqs_2));
}
