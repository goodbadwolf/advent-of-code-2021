use std::env;
use std::fs;

fn parse_input(raw_input: String) -> Vec<u32> {
    raw_input.split(",").map(|t| t.parse().unwrap()).collect()
}

fn simulate_laternfish_cycles(timers: &[u32], days: u32) -> u64 {
    let mut age_counts: [u64; 9] = [0; 9];

    for t in timers {
        let t = *t as usize;
        age_counts[t] += 1;
    }

    for _ in 1..=days {
        let new_fishes = age_counts[0];
        for i in 1..=8 {
            age_counts[i - 1] = age_counts[i];
        }
        age_counts[6] += new_fishes;
        age_counts[8] = new_fishes;
    }

    age_counts.iter().sum()
}

fn main() {
    let mut file_name: String = "input.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let timers = parse_input(
        fs::read_to_string(&file_name)
            .expect(format!("Could not read input file {}", &file_name).as_ref()),
    );
    let simulated_timers = simulate_laternfish_cycles(&timers, 80);
    println!("Day 6: After 80 days, num fishes = {}", simulated_timers);
    let simulated_timers = simulate_laternfish_cycles(&timers, 256);
    println!("Day 6: After 256 days, num fishes = {}", simulated_timers);
}
