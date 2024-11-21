const INPUT: &str = include_str!("test_input.txt");
//const INPUT: &str = include_str!("puzzle_input.txt");

fn main() {
    let mut lines = Vec::<i32>::new();
    INPUT.lines().for_each(|line| lines.push(line.parse().unwrap()));

    lines.sort();

    for line in &lines {
        for line2 in &lines {
            for line3 in &lines {
                if line + line2 + line3 == 2020 {
                    println!("{}", line * line2 * line3);
                    return;
                }
            }
        }
    }
}
