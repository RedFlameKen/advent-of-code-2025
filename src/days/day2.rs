use crate::util::file::read_file;

fn get_id_range(range: &str) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    print!("range: {range}\n");
    let mut range_parts = range.split('-');
    let min_str = range_parts.nth(0);
    let max_str = range_parts.nth(0);

    let min = match min_str {
        Some(value) => match value.parse::<i64>() {
            Ok(result) => result,
            Err(why) => panic!("failed to parse max {value} because: {why}"),
        },
        None => panic!("could not find item"),
    };
    print!("min: {}\n", min);

    let max = match max_str {
        Some(value) => match value.parse::<i64>() {
            Ok(result) => result,
            Err(why) => panic!("failed to parse max {value} because: {why}"),
        },
        None => panic!("could not find item"),
    };
    print!("max: {}\n", max);

    for id in min..max {
        ids.append(&mut vec![id.to_string()]);
    }

    return ids;
}

fn get_invalid_ids(ids: Vec<String>) -> Vec<String> {
    let mut invalid_ids: Vec<String> = Vec::new();
    for id in ids.iter() {
        let half = id.len() / 2;
        if id[0..half] == id[half..id.len()] {
            invalid_ids.insert(invalid_ids.len(), id.to_string());
            print!("repeating: {}\n", id);
        }
    }
    return invalid_ids;
}

pub fn day2() {
    let mut input = read_file("inputs/day2");
    input.pop();
    let mut value = 0;
    let ranges = input.split(",");
    for range in ranges {
        if range.is_empty() {
            continue;
        }
        let ids = get_id_range(range);
        let invalid_ids = get_invalid_ids(ids);
        for id in invalid_ids {
            let parsed = match id.parse::<i64>() {
                Ok(value) => value,
                Err(why) => panic!("could not parse {id} because: {why}"),
            };

            value += parsed;
        }
    }
    print!("value: {value}\n");
}

fn check_invalid(id: &str) -> bool {

    let mut pattern;
    let mut pattern_size = 1;
    while pattern_size <= id.len() / 2 {

        pattern = match id.get(0..pattern_size) {
            Some(value) => value.to_string(),
            None => panic!("nothing to get"),
        };

        let mut matched = true;
        let check_iter = id.len() / pattern_size;
        for i in 0..check_iter {
            if id.len() % pattern_size != 0 {
                matched = false;
                break;
            }
            let start = i * pattern_size;
            let end = (i * pattern_size) + pattern_size;
            let sub = match id.get(start..end) {
                Some(value) => value,
                None => panic!("could not find substring of {} to {} in {}",
                    start, end, id),
            };

            if sub != pattern {
                matched = false;
                break;
            }
        }

        if matched {
            return true;
        }

        pattern_size += 1;
    }

    return false;
}

fn get_invalid_ids2(ids: Vec<String>) -> Vec<String> {
    let mut invalid_ids: Vec<String> = Vec::new();
    for id in ids.iter() {
        if check_invalid(id) {
            invalid_ids.insert(invalid_ids.len(), id.to_string());
            print!("repeating: {}\n", id);
        }
    }
    return invalid_ids;
}

pub fn day2p2() {
    let mut input = read_file("inputs/day2");
    input.pop();
    let mut value = 0;
    let ranges = input.split(",");
    for range in ranges {
        if range.is_empty() {
            continue;
        }
        let ids = get_id_range(range);
        let invalid_ids = get_invalid_ids2(ids);
        for id in invalid_ids {
            let parsed = match id.parse::<i64>() {
                Ok(value) => value,
                Err(why) => panic!("could not parse {id} because: {why}"),
            };

            value += parsed;
        }
    }
    print!("value: {value}\n");
}
