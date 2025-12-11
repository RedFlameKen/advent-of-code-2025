use crate::util::file::read_file;

fn parse_range(range_str: &str) -> (i64, i64) {
    let mut range = range_str.split("-");
    let start_str = match range.nth(0) {
        Some(value) => value,
        None => panic!("there was no start to range {}", range_str),
    };
    let end_str = match range.nth(0) {
        Some(value) => value,
        None => panic!("there was no end to range {}", range_str),
    };
    let start = match start_str.parse::<i64>() {
        Ok(value) => value,
        Err(_why) => panic!("could not convert str {} to i64", start_str),
    };
    let end = match end_str.parse::<i64>() {
        Ok(value) => value,
        Err(_why) => panic!("could not convert str {} to i64", end_str),
    };

    return (start, end);
}

fn get_ranges(ranges: Vec<&str>) -> Vec<(i64, i64)> {
    let mut out: Vec<(i64, i64)> = Vec::new();
    for range in ranges {
        if range.is_empty() {
            continue;
        }
        out.push(parse_range(range));
    }
    return out;
}

fn get_i64_ids(ids: Vec<&str>) -> Vec<i64> {
    let mut out: Vec<i64> = Vec::new();
    for id in ids {
        if id.is_empty() {
            continue;
        }
        let i64_id = match id.parse::<i64>() {
            Ok(value) => value,
            Err(_why) => panic!("could not parse str {} to i64 because {}", id, _why),
        };
        out.push(i64_id);
    }
    return out;
}

fn is_fresh(ranges: &Vec<(i64, i64)>, id: i64) -> bool {
    for range in ranges {
        if (range.0..range.1 + 1).contains(&id) {
            return true;
        }
    }
    return false;
}

pub fn day5() {
    let input = read_file("inputs/day5");

    let mut split = input.split("\n\n");

    let ranges = match split.nth(0) {
        Some(value) => get_ranges(value.split("\n").collect::<Vec<&str>>()),
        None => panic!("could not get ranges"),
    };

    let ids = match split.nth(0) {
        Some(value) => get_i64_ids(value.split("\n").collect::<Vec<&str>>()),
        None => panic!("could not get ids"),
    };

    let mut fresh_id_count = 0;
    for id in ids {
        print!("id {}", id);
        if is_fresh(&ranges, id) {
            print!(" is fresh\n");
            fresh_id_count += 1;
        } else {
            print!(" is not fresh\n");
        }
    }

    print!("fresh id count: {}\n", fresh_id_count);
}

fn is_intersecting_ranges(range_a: &(i64, i64), range_b: &(i64, i64)) -> bool {
    return range_a.0 <= range_b.1 && range_a.0 >= range_b.0
        || range_a.1 >= range_b.0 && range_a.1 <= range_b.1
        || range_a.0 <= range_b.0 && range_a.1 >= range_b.1
        || range_a.0 >= range_b.0 && range_a.1 <= range_b.1;
}

fn join_ranges(ranges: Vec<(i64, i64)>) -> (Vec<(i64, i64)>, bool) {
    let mut out: Vec<(i64, i64)> = Vec::new();
    let mut has_joined = false;
    for range in ranges {
        if out.is_empty() {
            out.push(range);
            continue;
        }

        let mut joined = false;
        let mut duped = false;
        let len = out.len();
        for i in 0..len {
            let existing = out[i];
            if existing.0 == range.0 && existing.1 == range.1 {
                duped = true;
                continue;
            }

            if is_intersecting_ranges(&existing, &range) {
                let start;
                let end;

                if existing.0 <= range.1 && existing.0 >= range.0 {
                    start = range.0;
                } else {
                    start = existing.0;
                }

                if existing.1 >= range.0 && existing.1 <= range.1 {
                    end = range.1
                } else {
                    end = existing.1;
                }

                out[i] = (start, end);
                has_joined = true;
                joined = true;
                break;
            }
        }
        if !joined && !duped {
            out.push(range);
        }
    }
    return (out, has_joined);
}

pub fn day5p2() {
    let input = read_file("inputs/day5");

    let mut split = input.split("\n\n");

    let ranges = match split.nth(0) {
        Some(value) => get_ranges(value.split("\n").collect::<Vec<&str>>()),
        None => panic!("could not get ranges"),
    };

    let mut has_joined = true;
    let mut combined = ranges;
    while has_joined {
        let joined = join_ranges(combined);
        combined = joined.0;
        has_joined = joined.1;
    }

    let mut fresh_id_count = 0;

    for range in combined {
        print!("{}-{}\n", range.0, range.1);
        fresh_id_count += range.1 - range.0 + 1;
    }

    print!(
        "number of possible fresh indgredient ids: {}\n",
        fresh_id_count
    );
}
