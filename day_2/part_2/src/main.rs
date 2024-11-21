const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("puzzle_input.txt");

fn main() {
    let total = INPUT.lines().filter(|line| process_line(line)).count();
    println!("{total}");
}

fn process_line(line: &str) -> bool {
    let spaces: Vec<&str> = line.split(' ').collect();
    let min_max = get_min_max(spaces[0]);
    let char_val = spaces[1].chars().nth(0).unwrap_or(' ');

    let valid = is_valid(min_max.0, min_max.1, char_val, spaces[2]);

    valid
}

fn get_min_max(line: &str) -> (i32, i32) {
    let parts: Vec<&str> = line.split('-').collect();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn is_valid(min: i32, max: i32, letter: char, line: &str) -> bool {
    let new_min = min - 1;
    let new_max = max - 1;

    let min_char = line.chars().nth(new_min.try_into().unwrap()).unwrap_or(' ');
    let max_char = line.chars().nth(new_max.try_into().unwrap()).unwrap_or(' ');

    (min_char == letter) ^ (max_char == letter)
}
