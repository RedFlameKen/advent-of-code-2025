use crate::util::file::read_file;

fn find_highest_joltage_pos(bank: &str, start: usize, is_first: bool) -> (usize, char) {
    let mut highest: Option<(usize, char)> = None;
    for i in bank.char_indices() {

        if is_first && i.0 >= bank.len() - 1 {
            break;
        }

        if i.0 < start {
            print!("start is {}\n", start);
            print!("skipping on {}, {}\n", i.0, i.1);
            continue;
        }

        if highest == None {
            highest = Some(i);
            continue;
        }

        let highest_val = match highest {
            Some(value) => match value.1.to_digit(10) {
                Some(result) => result,
                None => panic!("could not convert {} to digit", value.1),
            },
            None => panic!("highest has no value"),
        };

        let current_val = match i.1.to_digit(10) {
            Some(value) => value,
            None => panic!("could not convert {} to digit", i.1),
        };

        if current_val > highest_val {
            highest = Some(i);
        }
    }
    return match highest {
        Some(value) => value,
        None => panic!("should not be None\n"),
    };
}

pub fn day3() {
    let input = read_file("inputs/day3");
    let banks = input.split("\n");

    let mut total_voltage: i32 = 0;
    for bank in banks {
        if bank.is_empty() {
            continue;
        }

        print!("first\n");
        let first_char = find_highest_joltage_pos(bank, 0, true);
        print!("second\n");
        let second_char = find_highest_joltage_pos(bank, first_char.0 + 1, false);
        let mut cat = String::new();
        cat.insert(0, first_char.1);
        cat.insert(1, second_char.1);

        let voltage = match cat.parse::<i32>() {
            Ok(value) => value,
            Err(why) => panic!("could not parse {} to i32 because {}", cat, why),
        };

        print!("voltage of bank {} is [{}]\n", bank, voltage);
        total_voltage += voltage;
    }

    print!("total voltage: {total_voltage}\n");
}
