use crate::util::file::read_file;

fn parse_input_to_2d_array(input: Vec<&str>) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();
    for line in input {
        let mut line_vec = Vec::new();
        for char in line.chars() {
            line_vec.push(char);
        }
        out.push(line_vec);
    }
    return out;
}

fn parse_input_to_string_arrays(input: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let split = input.split("\n");
    for line in split {
        if !line.is_empty() {
            out.push(line);
        }
    }
    return out;
}

fn adjacent_roll_count(grid: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut adjecent_rolls: usize = 0;
    let y_start: usize = if y == 0 { 0 } else { y - 1 };
    let y_end: usize = if y + 1 >= grid.len() { y } else { y + 1 };

    let x_start: usize = if x == 0 { 0 } else { x - 1 };
    let x_end: usize = if x + 1 >= grid[0].len() { x } else { x + 1 };
    for i in y_start..y_end + 1 {

        for j in x_start..x_end + 1 {
            if i == y && j == x {
                continue;
            }

            if grid[i][j] == '@' {
                adjecent_rolls += 1;
            }
        }
    }

    return adjecent_rolls;
}

pub fn day4() {
    let input = read_file("inputs/day4");
    let split = parse_input_to_string_arrays(&input);
    let grid: Vec<Vec<char>> = parse_input_to_2d_array(split);

    let mut accessible_roll_count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let adjacent_rolls = adjacent_roll_count(&grid, j, i);
            if grid[i][j] == '@' && adjacent_rolls < 4 {
                accessible_roll_count += 1;
                print!("{}", adjacent_rolls);
            } else {
                print!("{}", grid[i][j]);
            }
        }
        print!("\n");
    }

    print!("accessible roll count: {}\n", accessible_roll_count);
}


pub fn day4p2() {
    let input = read_file("inputs/day4");
    let split = parse_input_to_string_arrays(&input);
    let mut grid: Vec<Vec<char>> = parse_input_to_2d_array(split);

    let mut accessible_roll_count = 0;

    let mut removed_roll_count = 1;

    while removed_roll_count > 0 {
        removed_roll_count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let adjacent_rolls = adjacent_roll_count(&grid, j, i);
                if grid[i][j] == '@' && adjacent_rolls < 4 {
                    grid[i][j] = '.';
                    accessible_roll_count += 1;
                    removed_roll_count += 1;
                    print!("{}", adjacent_rolls);
                } else {
                    print!("{}", grid[i][j]);
                }
            }
            print!("\n");
        }
    }

    print!("accessible roll count: {}\n", accessible_roll_count);
}
