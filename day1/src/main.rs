use std::env;
use std::fs;

fn parse_input(raw_input: String) -> Vec<u32> {
    raw_input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

fn count_depth_increases(depths: &[u32]) -> u32 {
    let count_increases: u32 = depths
        .iter()
        .zip(depths.iter().skip(1))
        .map(|(lhs, rhs)| if rhs > lhs { 1 } else { 0 })
        .sum();
    count_increases
}

fn count_depth_increases_part2(depths: &[u32]) -> u32 {
    let sliding_window_depths: Vec<u32> = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((first, second), third)| first + second + third)
        .collect();

    count_depth_increases(&sliding_window_depths)
}

fn main() {
    let mut file_name: String = "input.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let depths = parse_input(fs::read_to_string(file_name).expect("Could not read input file"));
    let increases_count = count_depth_increases(&depths);
    println!("Sonar Sweep: Increasing depth count = {}", increases_count);

    let sliding_window_count = count_depth_increases_part2(&depths);
    println!(
        "Sonar Sweep: Increasing sliding window depth count = {}",
        sliding_window_count
    );
}
