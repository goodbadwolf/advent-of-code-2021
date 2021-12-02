use std::env;
use std::fs;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
    None,
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn parse_input(raw_input: String) -> Vec<Command> {
    raw_input
        .lines()
        .map(|line| {
            let cmd_parts: Vec<&str> = line.split_ascii_whitespace().collect();
            let cmd = cmd_parts[0];
            let dist = cmd_parts[1].parse::<i32>().unwrap();
            match cmd {
                "forward" => Command::Forward(dist),
                "down" => Command::Down(dist),
                "up" => Command::Up(dist),
                _ => Command::None,
            }
        })
        .collect()
}

fn track_position_part1(commands: &[Command], position: &mut Position) {
    for command in commands {
        match command {
            Command::Forward(dist) => {
                position.horizontal += dist;
            }
            Command::Up(dist) => {
                position.depth -= dist;
            }
            Command::Down(dist) => {
                position.depth += dist;
            }
            _ => {}
        }
    }
}

fn track_position_part2(commands: &[Command], position: &mut Position) {
    for command in commands {
        match command {
            Command::Forward(dist) => {
                position.horizontal += dist;
                position.depth += position.aim * dist;
            }
            Command::Up(dist) => {
                position.aim -= dist;
            }
            Command::Down(dist) => {
                position.aim += dist;
            }
            _ => {}
        }
    }
}

fn main() {
    let mut file_name: String = "input.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let commands = parse_input(
        fs::read_to_string(&file_name)
            .expect(format!("Could not read input file {}", &file_name).as_ref()),
    );
    let mut submarine_position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    track_position_part1(&commands, &mut submarine_position);
    println!(
        "Day 2: Part 1 = {}",
        (submarine_position.horizontal * submarine_position.depth)
    );

    let mut submarine_position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    track_position_part2(&commands, &mut submarine_position);
    println!(
        "Day 2: Part 2 = {}",
        (submarine_position.horizontal * submarine_position.depth)
    );
}
