use std::{path::Path, fs::File, io::{self, BufRead}};

fn main() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut increased = 0;
        let mut last = -1;
        lines.for_each(|x| {
            match x {
                Ok(str) => {
                    let int = str.parse::<i32>().unwrap();
                    if last < 0 {
                        last = int;
                        return;
                    }
                    if last < int {
                        increased = increased + 1;
                    }
                    last = int;
                }
                Err(e) => {
                    println!("{}", e);
                    panic!("Error in lines for_each!");
                }
            }
        });
        println!("{}", increased);
    }
}

fn part2() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut vec = Vec::new(); // Working with io::Lines in part 1 was pain. Thank you vec.
        lines.for_each(|x| {
            match x {
                Ok(str) => {
                    vec.push(str.parse::<i32>().unwrap());
                }
                Err(e) => {
                    println!("{}", e);
                    panic!("Error in lines for_each!");
                }
            }
        });
        let mut increased = 0;
        let mut last_sum = -1;
        for i in 0..(vec.len()-2) {
            let sum = vec[i] + vec[i+1] + vec[i+2];
            if last_sum < 0 {
                last_sum = sum;
                continue;
            }
            if sum > last_sum {
                increased = increased + 1;
            }
            last_sum = sum;
        }
        println!("{}", increased);
    }
}


// Credit: Rust docs

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}