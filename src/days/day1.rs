use crate::util::file::read_file;

enum Direction {
    LEFT,
    RIGHT,
    NONE,
}

fn get_direction(input: &str) -> Direction {

    // Get the first character of the input (R/L)
    let direction_char = match input.chars().nth(0) {
        Some(_value) => _value,
        None => panic!("Could not get direction string of string {}", input),
    };

    if direction_char == 'L' {
        return Direction::LEFT;
    } else if direction_char == 'R' {
        return Direction::RIGHT;
    } else {
        return Direction::NONE;
    }
}

fn get_rotations(input: &str) -> i32 {
    let rotations_string = &input[1..];

    let rotations = rotations_string.parse::<i32>().unwrap();

    return rotations % 100;
}

fn get_left_rotation(value: i32, rotations: i32) -> i32 {
    if (value - rotations) < 0 {
        return 100 - (rotations - value);
    } else {
        return value - rotations;
    }
}

fn get_right_rotation(value: i32, rotations: i32) -> i32 {
    if (value + rotations) >= 100 {
        return (rotations + value) - 100;
    } else {
        return value + rotations;
    }
}

pub fn day1() {
    let input_string = read_file("inputs/day1");
    let inputs = input_string.split("\n");

    let mut value: i32 = 50;
    let mut zeros = 0;
    for input in inputs {
        // The last one int the split string is empty, ignore it
        if input.is_empty() {
            continue;
        }

        let direction = get_direction(input);
        let rotations = get_rotations(input);

        value = match direction {
            Direction::LEFT => get_left_rotation(value, rotations),
            Direction::RIGHT => get_right_rotation(value, rotations),
            Direction::NONE => panic!("invalid rotation"),
        };
        assert!(value < 100 && value >= 0, "value is {value}");
        print!("current: {}\n", value);

        if value == 0 {
            zeros += 1;
        }

    }

    print!("value: {}\n", zeros);
}
