use super::common::ProgramError;
use super::day0::median;

use std::io;

pub fn quartiles(data: &Vec<i32>) -> (f32, f32, f32) {
    let q2 = median(data);
    let lower: Vec<i32> = data.iter().filter(|&&x| x < q2 as i32).cloned().collect();
    let upper: Vec<i32> = data.iter().filter(|&&x| x > q2 as i32).cloned().collect();

    (median(&lower), q2, median(&upper))
}

/// Runns all the Day1 Challenges
pub fn run() -> Result<(), ProgramError> {
    println!("------------------------------");
    println!("Day 1");
    println!("------------------------------");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<u32>()?;

    input.clear();
    io::stdin().read_line(&mut input)?;
    let data = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Failed to parse data .."))
        .collect::<Vec<_>>();

    if data.len() != n as usize {
        return Err(ProgramError::Logic(format!(
            "n: {}, data.len: {}",
            n,
            data.len()
        )));
    }

    let (q1, q2, q3) = quartiles(&data);
    println!("{}\n{}\n{}", q1, q2, q3);

    Ok(())
}
