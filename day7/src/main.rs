use std::env;
use std::fs;

fn parse_input(raw_input: String) -> Vec<u32> {
    raw_input.split(",").map(|t| t.parse().unwrap()).collect()
}

fn find_least_fuel_position_linear(positions: &[u32]) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|p| {
            positions
                .iter()
                .map(|&p2| if p > p2 { p - p2 } else { p2 - p })
                .sum()
        })
        .min()
        .unwrap()
}

fn find_least_fuel_position_quadratic(positions: &[u32]) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|p| {
            positions
                .iter()
                .map(|&p2| {
                    let delta = if p > p2 { p - p2 } else { p2 - p };
                    delta * (delta + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn main() {
    let mut file_name: String = "input.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let crab_positions = parse_input(
        fs::read_to_string(&file_name)
            .expect(format!("Could not read input file {}", &file_name).as_ref()),
    );
    let min_fuel = find_least_fuel_position_linear(&crab_positions);
    println!("Day 7: Min linear position fuel = {}", min_fuel);
    let min_fuel = find_least_fuel_position_quadratic(&crab_positions);
    println!("Day 7: Min quadratic position fuel = {}", min_fuel);
}
