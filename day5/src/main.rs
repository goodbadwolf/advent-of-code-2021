use std::env;
use std::fs;

#[derive(Clone, Copy, Debug)]
struct VentLine {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

fn parse_input(raw_input: String) -> Vec<VentLine> {
    raw_input
        .lines()
        .map(|line| {
            let line = line.replace(" -> ", ",");
            let mut coords = line.split(",");
            VentLine {
                x1: coords.next().unwrap().parse().unwrap(),
                y1: coords.next().unwrap().parse().unwrap(),
                x2: coords.next().unwrap().parse().unwrap(),
                y2: coords.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn count_straight_vent_overlaps(vents: &[VentLine]) -> u32 {
    let straight_vents: Vec<VentLine> = vents
        .iter()
        .filter(|&v| (v.x1 == v.x2) || (v.y1 == v.y2))
        .cloned()
        .collect();

    count_all_vent_overlaps(&straight_vents)
}

fn count_all_vent_overlaps(vents: &[VentLine]) -> u32 {
    let mut xmax = 0_u32;
    let mut ymax = 0_u32;
    vents.iter().for_each(|v| {
        xmax = xmax.max(v.x1);
        xmax = xmax.max(v.x2);
        ymax = ymax.max(v.y1);
        ymax = ymax.max(v.y2);
    });

    let mut diagram: Vec<Vec<u32>> = vec![vec![0; xmax as usize + 1]; ymax as usize + 1];
    for vent in vents.iter() {
        let xinc = if vent.x1 < vent.x2 {
            1
        } else if vent.x1 == vent.x2 {
            0
        } else {
            -1
        };
        let yinc = if vent.y1 < vent.y2 {
            1
        } else if vent.y1 == vent.y2 {
            0
        } else {
            -1
        };
        let mut x = vent.x1 as i32;
        let mut y = vent.y1 as i32;
        loop {
            diagram[y as usize][x as usize] += 1;
            x += xinc;
            y += yinc;
            if (x - xinc) == vent.x2 as i32 && (y - yinc) == vent.y2 as i32 {
                break;
            }
        }
    }

    let mut overlaps = 0;
    for y in 0..=ymax {
        for x in 0..=xmax {
            if diagram[y as usize][x as usize] >= 2 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

fn main() {
    let mut file_name: String = "input.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let vents = parse_input(
        fs::read_to_string(&file_name)
            .expect(format!("Could not read input file {}", &file_name).as_ref()),
    );
    let overlaps = count_straight_vent_overlaps(&vents);
    println!("Day 5: Straight vent line overlaps = {}", overlaps);
    let all_overlaps = count_all_vent_overlaps(&vents);
    println!("Day 5: All vent line overlaps = {}", all_overlaps);
}
