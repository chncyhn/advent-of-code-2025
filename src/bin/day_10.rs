use std::collections::HashSet;
use std::fs;

use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, constraint, default_solver, variable,
};

#[derive(Debug)]
struct Machine {
    required_toggles: Vec<u32>,
    buttons: Vec<Button>,
    jolts: Vec<u32>,
}

#[derive(Debug)]
struct Button {
    toggles: Vec<u32>,
}

fn solve_milp(machine: &Machine) -> u32 {
    let n = machine.buttons.len(); // variables
    let m = machine.jolts.len(); // constraints

    let mut problem = ProblemVariables::new();
    let x: Vec<_> = (0..n)
        .map(|_| problem.add(variable().integer().min(0)))
        .collect();

    let objective: Expression = x.iter().copied().sum();
    let mut model = problem.minimise(objective).using(default_solver);

    // for each indicator, sum of buttons with that indicator should sum up to m.jolts[i]
    for i in 0..m {
        let rhs = machine.jolts[i];
        let lhs: Expression = (0..n)
            .filter(|j| machine.buttons[*j].toggles.contains(&(i as u32)))
            .map(|j| x[j])
            .sum();
        model = model.with(constraint!(lhs == rhs));
    }
    let solution = model.solve().unwrap();
    let solved_x: Vec<u32> = x
        .iter()
        .map(|&var| solution.value(var).round() as u32)
        .collect();
    solved_x.iter().sum()
}

fn bfs(m: &Machine) -> u32 {
    type State = Vec<u32>;
    let next = |s: &State| -> Vec<State> {
        let mut nxt_states: Vec<State> = vec![];
        for b in &m.buttons {
            let mut nxt_st = s.clone();
            for t in &b.toggles {
                if s.contains(t) {
                    nxt_st.remove(nxt_st.iter().position(|x| x == t).unwrap());
                } else {
                    nxt_st.push(*t);
                }
            }
            nxt_st.sort();
            nxt_states.push(nxt_st);
        }
        nxt_states
    };
    let mut queue: Vec<State> = vec![vec![]];
    let mut depth = 0;
    let mut visited: HashSet<State> = HashSet::new();
    while queue.len() > 0 {
        let mut frontier: Vec<State> = vec![];
        for cur in &queue {
            if *cur == m.required_toggles {
                return depth;
            }
            for nxt in next(cur) {
                if visited.insert(nxt.clone()) {
                    frontier.push(nxt);
                }
            }
        }
        depth += 1;
        queue = frontier;
    }
    panic!("Cannot solve?")
}

fn read_machines(file: &str) -> Vec<Machine> {
    fn parse_button(s: &str) -> Button {
        Button {
            toggles: s[1..s.len() - 1]
                .split(",")
                .map(|c| c.parse().unwrap())
                .collect(),
        }
    }

    fn parse_req_toggles(s: &str) -> Vec<u32> {
        s.chars()
            .enumerate()
            .filter(|(_, v)| *v == '#')
            .map(|(i, _)| (i - 1) as u32)
            .collect()
    }

    fn parse_jolts(s: &str) -> Vec<u32> {
        s[1..s.len() - 1]
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect()
    }

    fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .map(|l| {
            let substrs: Vec<&str> = l.trim().split(" ").collect();
            let required_toggles = parse_req_toggles(substrs[0]);
            let buttons: Vec<Button> = substrs[1..substrs.len() - 1]
                .iter()
                .map(|s| parse_button(s))
                .collect();
            let jolts = parse_jolts(substrs[substrs.len() - 1]);
            Machine {
                required_toggles,
                buttons,
                jolts,
            }
        })
        .collect()
}

fn main() {
    let machines = read_machines("day10_real.txt");

    let part_1: u32 = machines.iter().map(bfs).sum();
    let part_2: u32 = machines.iter().map(solve_milp).sum();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
