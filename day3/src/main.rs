use std::env;
use std::fs;

fn parse_bitstring(bitstring: &str) -> i32 {
    bitstring
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| 2_i32.pow(i as u32) * c.to_digit(10).unwrap() as i32)
        .sum::<i32>()
}

fn parse_input(raw_input: String) -> (usize, Vec<i32>) {
    let values: Vec<i32> = raw_input
        .lines()
        .map(|line| parse_bitstring(line))
        .collect();
    let size = raw_input.lines().next().unwrap().len();
    (size, values)
}

fn calculate_rates(size: usize, values: &[i32]) -> (i32, i32) {
    let num_values = values.len() as i32;
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..size {
        let mask = 2_i32.pow(i as u32);
        let ones: i32 = values.iter().map(|v| ((v & mask) == mask) as i32).sum();
        let zeros = num_values - ones;
        let gamma_bit = if ones > zeros { 1 } else { 0 };
        let epsilon_bit = 1 - gamma_bit;
        gamma += 2_i32.pow(i as u32) * gamma_bit;
        epsilon += 2_i32.pow(i as u32) * epsilon_bit;
    }
    (gamma, epsilon)
}

fn calculate_rates_part2(size: usize, values: &[i32]) -> (i32, i32) {
    let mut oxygen_rating = 0;
    let mut co2_rating = 0;

    let mut oxygen_values: Vec<i32> = Vec::from(values);
    for i in (0..size).rev() {
        let mask = 2_i32.pow(i as u32);
        let ones: i32 = oxygen_values
            .iter()
            .map(|v| ((v & mask) == mask) as i32)
            .sum();
        let zeros = oxygen_values.len() as i32 - ones;
        let most_common_bit = if zeros > ones { 0 } else { 1 };
        let potential_values: Vec<i32> = oxygen_values
            .iter()
            .filter(|&&v| ((v & mask) >> i) == most_common_bit)
            .map(|&v| v)
            .collect();
        if potential_values.len() > 1 {
            oxygen_values = potential_values;
        } else {
            oxygen_rating = potential_values[0];
            break;
        }
    }

    let mut co2_values: Vec<i32> = Vec::from(values);
    for i in (0..size).rev() {
        let mask = 2_i32.pow(i as u32);
        let ones: i32 = co2_values.iter().map(|v| ((v & mask) == mask) as i32).sum();
        let zeros = co2_values.len() as i32 - ones;
        let least_common_bit = if ones < zeros { 1 } else { 0 };
        let potential_values: Vec<i32> = co2_values
            .iter()
            .filter(|&&v| ((v & mask) >> i) == least_common_bit)
            .map(|&v| v)
            .collect();
        if potential_values.len() > 1 {
            co2_values = potential_values;
        } else {
            co2_rating = potential_values[0];
            break;
        }
    }

    (oxygen_rating, co2_rating)
}

fn main() {
    let mut file_name: String = "input.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let (size, values) = parse_input(
        fs::read_to_string(&file_name)
            .expect(format!("Could not read input file {}", &file_name).as_ref()),
    );

    let (gamma_rate, epsilon_rate) = calculate_rates(size, &values);
    let power_consumption = gamma_rate * epsilon_rate;
    println!("Day 3: Power consumption = {}", power_consumption);

    let (oxygen_rating, co2_rating) = calculate_rates_part2(size, &values);
    let life_support_rating = oxygen_rating * co2_rating;
    println!("Day 3: Life support rating = {}", life_support_rating);
}
