const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("puzzle_input.txt");

fn main() {
    let total = INPUT.lines().filter(|line| process_line(line)).count();
    println!("{total}");
}

fn process_line(line: &str) -> bool {
    //println!("\n{}", line);
    
    let spaces: Vec<&str> = line.split(' ').collect();
    
    // Get min/max
    let min_max = get_min_max(spaces[0]);
    
    // Get char
    let char_val = spaces[1].chars().nth(0).unwrap_or(' ');

    is_valid(min_max.0, min_max.1, char_val, spaces[2])
    
    //println!("Min: {} Max: {} Char: {}  Valid: {}", min_max.0, min_max.1, char_val, valid);
}

fn get_min_max(line: &str) -> (i32, i32) {
    let parts: Vec<&str> = line.split('-').collect();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn is_valid(min: i32, max: i32, letter: char, line: &str) -> bool {
    let count = line.chars().filter(|&c| c == letter).count() as i32;

    count >= min && count <= max
}
