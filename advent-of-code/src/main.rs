use std::error::Error;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> impl std::fmt::Display {
    let (mut x, mut y) = INPUT
        .lines()
        .map(|x| x.split_once("   ").unwrap())
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .collect::<(Vec<_>, Vec<_>)>();
    x.sort_unstable();
    y.sort_unstable();
    x.iter()
        .zip(y.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>()
}

fn part2() -> impl std::fmt::Display {
    let (x, y) = INPUT
        .lines()
        .map(|x| x.split_once("   ").unwrap())
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .collect::<(Vec<_>, Vec<_>)>();
    x.iter().fold(0, |acc, x| {
        acc + (x * y.iter().filter(|&&y| y == *x).count() as i32)
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", part1());
    println!("{}", part2());
    Ok(())
}
