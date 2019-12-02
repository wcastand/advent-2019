use std::fs::File;
use std::io::prelude::*;

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|s| {
            let i: i32 = s.parse().unwrap();
            return i / 3 - 2;
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|s| {
            let mut i: i32 = s.parse().unwrap();
            let mut total = 0;
            while i > 0 {
                i = i / 3 - 2;
                if i > 0 {
                    total += i;
                }
            }
            return total;
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/1.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let fuel = part1(&contents);

    println!("part 1 - fuel: {}", fuel);
    println!("part 2 - fuel: {}", part2(&contents));

    Ok(())
}
