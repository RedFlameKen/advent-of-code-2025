use crate::util::file::read_file;

fn get_cells(input: &str) -> Vec<Vec<&str>> {
    let mut cells: Vec<Vec<&str>> = Vec::new();
    let split = input.split("\n");

    for line in split {
        if line.is_empty() {
            continue;
        }
        let mut line_cells: Vec<&str> = Vec::new();
        let parts = line.split(" ");
        for part in parts {
            if part.is_empty() {
                continue;
            }
            line_cells.push(part);
        }

        cells.push(line_cells);
    }

    return cells;
}

fn get_start_indeces(op_line: &str) -> Vec<usize> {
    let mut start_indeces: Vec<usize> = Vec::new();
    let mut iter = op_line.char_indices();
    let first_option = iter.nth(0);
    if first_option == None {
        return start_indeces;
    }

    let prev = first_option.unwrap().0;
    start_indeces.push(prev);

    for char in iter {
        if char.1 != ' ' {
            start_indeces.push(char.0 - prev);
        }
    }
    return start_indeces;
}

fn get_cells2(start_indeces: Vec<usize>, lines:  Vec<&str>) -> Vec<Vec<&str>>{
    let mut columns = Vec::new();
    for index in 1..start_indeces.len() + 1 {
        let mut row = Vec::new();

        for line in &lines {
            let start;
            let end;

            if index >= start_indeces.len() {
                start = start_indeces[start_indeces.len() - 1];
                end = line.len();
            } else {
                start = start_indeces[index-1];
                end = start_indeces[index]-1;
            }
            let number = match line.get(start..end) {
                Some(value) => value,
                None => panic!("could not get substring from {}", line),
            };
            row.push(number);
        }
        columns.push(row);
    }
    return columns;
}

fn operate(op: &str, num1: i64, num2: i64) -> i64 {
    match op {
        "*" => num1 * num2,
        "+" => num1 + num2,
        _ => 0,
    }
}

fn to_number_line(num_strings: Vec<&str>) -> Vec<i64> {
    let mut nums = Vec::new();
    for num_str in num_strings {
        let num = match num_str.parse::<i64>() {
            Ok(value) => value,
            Err(_why) => panic!("could not parse str {} to i64 because {}", num_str, _why),
        };

        nums.push(num);
    }
    return nums;
}

fn get_number_cells(lines: &[Vec<&str>]) -> Vec<Vec<i64>> {
    let mut cells = Vec::new();
    for line in lines {
        let number_line = to_number_line(line.to_vec());
        cells.push(number_line);
    }
    return cells;
}

pub fn day6() {
    let input = read_file("inputs/day6");

    let cells = get_cells(&input);
    let numbers = match cells.get(0..cells.len() - 1) {
        Some(value) => get_number_cells(value),
        None => panic!(
            "could not find numbers from ranges {}-{}",
            0,
            cells.len() - 1
        ),
    };

    let mut total = 0;
    let operations = &cells[cells.len() - 1];
    for op_index in 0..operations.len() {
        let op = operations[op_index];
        let mut result = numbers[0][op_index];
        print!("{} ", result);
        for num_index in 1..numbers.len() {
            let next_num = numbers[num_index][op_index];
            print!("{} {} ", op, next_num);
            let output = operate(op, result, next_num);
            result = output;
        }

        print!("= {}\n", result);
        total += result;
    }

    print!("total: {}\n", total);
}

fn get_column_width(column: Vec<&str>) -> usize {
    let mut width = 0;
    for cell in column {
        if cell.len() > width {
            width = cell.len();
        }
    }
    return width;
}

fn string_vec_to_str_vec(vector: &Vec<String>) -> Vec<&str>{
    let mut out = Vec::new();
    for string in vector {
        out.push(string.as_str());
    }
    return out;
}

fn to_cephalopod_math(columns: Vec<Vec<&str>>) -> Vec<Vec<String>> {
    let mut cephalopod_columns: Vec<Vec<String>> = Vec::new();
    for column in columns {
        let width = get_column_width(column.clone());
        let mut cephalopod_column: Vec<String> = Vec::new();
        for num_pos in (0..width).rev() {
            let mut num = String::new();
            for col_pos in 0..column.len() {
                let chars: Vec<char> = column[col_pos].chars().collect();
                if chars[num_pos] == ' ' {
                    continue;
                }
                num.push(chars[num_pos]);
            }
            cephalopod_column.push(num);
        }
        cephalopod_columns.push(cephalopod_column);

    }
    return cephalopod_columns;
}

pub fn day6p2() {
    let input = read_file("inputs/day6");

    let split = input.split("\n");
    let split_collection: Vec<&str> = split.clone().collect();

    let op_line = split_collection[split_collection.len() - 2];

    let operation_split = op_line.split(" ");
    let mut operations = Vec::new();
    for op in operation_split {
        if op.is_empty() {
            continue;
        }
        operations.push(op);
    }

    let number_lines = match split_collection.get(0..split_collection.len() - 2) {
        Some(value) => value.to_vec(),
        None => panic!("could not get number lines"),
    };

    let start_indeces = get_start_indeces(op_line);
    let cells = get_cells2(start_indeces, number_lines);

    let cephalopod_numbers = to_cephalopod_math(cells);

    let mut str_numbers = Vec::new();
    for column in &cephalopod_numbers {
        str_numbers.push(string_vec_to_str_vec(column));
    }

    let number_grid = get_number_cells(&str_numbers);

    let mut total = 0;

    for index in 0..operations.len() {
        let op = operations[index];
        let column = number_grid[index].clone();
        let mut result = column[0];
        print!("{} ", result);
        for index in 1..column.len() {
            print!("{} {} ", op, column[index]);
            result = operate(op, result, column[index]);
        }
        print!("= {}\n", result);
        total += result;
    }

    print!("total: {}\n", total);
}
