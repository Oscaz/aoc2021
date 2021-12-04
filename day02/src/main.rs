use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = read_lines("./input.txt").unwrap();
    let mut horizontal = 0;
    let mut depth = 0;
    lines.for_each(|x| {
        let str = x.unwrap();
        let direction = str.split(" ").nth(0).unwrap();
        let amount = str.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        if direction.eq_ignore_ascii_case("forward") { 
            horizontal += amount;
        }
        if direction.eq_ignore_ascii_case("down") { 
            depth += amount;
        }
        if direction.eq_ignore_ascii_case("up") {
            depth -= amount;
        }
    });
    println!("Horizontal: {}, Depth: {}, Multiplied: {}", horizontal, depth, horizontal * &depth);
}

fn part2() {
    let lines = read_lines("./input.txt").unwrap();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    lines.for_each(|x| {
        let str = x.unwrap();
        let direction = str.split(" ").nth(0).unwrap();
        let amount = str.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        if direction.eq_ignore_ascii_case("forward") { 
            horizontal += amount;
            depth += aim * amount;
        }
        if direction.eq_ignore_ascii_case("down") { 
            aim += amount;
        }
        if direction.eq_ignore_ascii_case("up") {
            aim -= amount;
        }
    });
    println!("Horizontal: {}, Depth: {}, Multiplied: {}", horizontal, depth, horizontal * &depth);
}

// Credit: Rust docs

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}