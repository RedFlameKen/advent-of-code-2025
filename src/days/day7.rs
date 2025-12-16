use std::collections::HashMap;

use crate::util::file::read_file;

fn split_input_by_line(input: &String) -> Vec<&str> {
    let mut split_vec = Vec::new();
    let split = input.split("\n");
    for line in split {
        if line.is_empty() {
            continue;
        }
        split_vec.push(line);
    }
    return split_vec;
}

fn get_start_pos(line: &str) -> usize {
    return match line.find("S") {
        Some(value) => value,
        None => panic!("Could not find start position"),
    };
}

fn should_split(pos: usize, line: &str) -> bool {
    let char: char = line.chars().collect::<Vec<char>>()[pos];
    return char == '^';
}

fn fire_beam(
    beam_positions: Vec<usize>,
    lines: Vec<&str>,
    cur_line: usize,
    mut split_count: usize,
) -> usize {
    if cur_line >= lines.len() {
        print!("finished\n");
        return split_count;
    }
    let line = lines[cur_line];
    let mut new_positions = Vec::<usize>::new();
    for pos in beam_positions {
        if should_split(pos, line) {
            split_count += 1;
            let left = pos - 1;
            let right = pos + 1;
            if !new_positions.contains(&left) {
                new_positions.push(left);
            }
            if !new_positions.contains(&right) {
                new_positions.push(right);
            }
        } else {
            if !new_positions.contains(&pos) {
                new_positions.push(pos);
            }
        }
    }

    for index in 0..line.len() {
        if new_positions.contains(&index) {
            print!("|");
        } else {
            print!("{}", line.chars().collect::<Vec<char>>()[index]);
        }
    }
    print!("\n");

    return fire_beam(new_positions, lines, cur_line + 1, split_count);
}

fn fire_beam4(
    beam_position: usize,
    lines: Vec<&str>,
    cur_line: usize,
    mut timelines: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if cur_line >= lines.len() {
        return timelines + 1;
    }

    let line = lines[cur_line];
    let should_split = should_split(beam_position, line);

    let skip = if cur_line > 0 { 2 } else { 1 };
    if should_split {
        let left = beam_position - 1;
        let right = beam_position + 1;
        let next_line = cur_line + skip;
        let lines_ref = &lines;

        let left_key = (left, next_line);
        let left_timelines;
        if memo.contains_key(&left_key) {
            left_timelines = match memo.get(&left_key) {
                Some(value) => *value,
                None => panic!("could not get the value of key ({}, {})", left, next_line),
            };
        } else {
            left_timelines = fire_beam4(left, lines_ref.to_vec(), cur_line + skip, timelines, memo);
            memo.insert(left_key, left_timelines);
        }

        let right_key = (right, next_line);
        let right_timelines;
        if memo.contains_key(&right_key) {
            right_timelines = match memo.get(&right_key) {
                Some(value) => *value,
                None => panic!("could not get the value of key ({}, {})", right, next_line),
            };
        } else {
            right_timelines = fire_beam4(right, lines_ref.to_vec(), cur_line + skip, timelines, memo);
            memo.insert(right_key, right_timelines);
        }

        timelines = left_timelines + right_timelines;
    } else {
        timelines = fire_beam4(beam_position, lines, cur_line + skip, timelines, memo);
    }

    return timelines;
}

pub fn day7() {
    let input = read_file("inputs/day7");

    let split = split_input_by_line(&input);

    let start_pos = get_start_pos(split[0]);

    let travel_lines = match split.get(1..split.len()) {
        Some(value) => value.to_vec(),
        None => panic!("there were no travel lines"),
    };

    let split_count = fire_beam([start_pos].to_vec(), travel_lines, 0, 0);

    print!("split count: {}\n", split_count);
}

pub fn day7p2() {
    let input = read_file("inputs/day7");

    let split = split_input_by_line(&input);

    let start_pos = get_start_pos(split[0]);

    let travel_lines = match split.get(1..split.len()) {
        Some(value) => value.to_vec(),
        None => panic!("there were no travel lines"),
    };

    let mut memo = HashMap::<(usize, usize), usize>::new();
    let split_count = fire_beam4(start_pos, travel_lines, 0, 0, &mut memo);

    print!("timeline count: {}\n", split_count);
}
