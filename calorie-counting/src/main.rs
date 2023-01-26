use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("failure opening file");

    let mut counts = content
        .split("\n\n")
        .map(|chunk| -> usize { chunk.split("\n").map(|row| row.parse().unwrap_or(0)).sum() })
        .collect::<Vec<_>>();

    // let mut v = counts.collect::<Vec<_>>();

    counts.sort();

    let last_idx = counts.len() - 1;

    println!("The highest count is {}", counts[last_idx]);
}
