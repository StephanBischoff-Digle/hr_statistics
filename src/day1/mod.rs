use super::common::ProgramError;
use super::day0::median;

use std::io;
use std::iter;

/// Calculates the quartiles of the values given by the `data` vector.
///
/// # Example
///
/// ```
/// assert_eq!(quartiles(&vec![3, 7, 8, 5, 12, 14, 21, 13, 18]), (6.0, 12.0, 16.0));
/// ```
pub fn quartiles(data: &Vec<i32>) -> (f32, f32, f32) {
    let mut data = data.clone();
    data.sort();
    let q2 = median(&data);

    let lower: Vec<i32>;
    let upper: Vec<i32>;

    if data.len() % 2 == 0 {
        lower = data.iter().take(data.len() / 2).cloned().collect();
        upper = data.iter().skip(data.len() / 2).cloned().collect();
    } else {
        lower = data.iter().take(data.len() / 2).cloned().collect();
        upper = data.iter().skip(data.len() / 2 + 1).cloned().collect();
    }

    (median(&lower), q2, median(&upper))
}

/// Calculates the interquartile range of the values given by the `data` vector. The `frequencys`
/// vector determines how often the values of `data` are repeated in the actual data.
///
/// # Example
///
/// ```
/// let data = vec![6, 12, 8, 10, 20, 16];
/// let frequencys = vec![5, 4, 3, 2, 1, 5];
///
/// assert_eq!(interquartile_range(&data, &frequencys), 9.0);
/// ```
pub fn interquartile_range(data: &Vec<i32>, frequencys: &Vec<i32>) -> f32 {
    let mut working_data = Vec::new();
    for (d, f) in data.iter().zip(frequencys.iter()) {
        let mut tmp = iter::repeat(d).take(*f as usize).cloned().collect();
        working_data.append(&mut tmp);
    }

    let (q1, _, q3) = quartiles(&working_data);

    q3 - q1
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

    input.clear();
    io::stdin().read_line(&mut input)?;
    let nums = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Failed to parse data .."))
        .collect::<Vec<_>>();

    if nums.len() != n as usize {
        return Err(ProgramError::Logic(format!(
            "n: {}, nums.len: {}",
            n,
            nums.len()
        )));
    }

    let (q1, q2, q3) = quartiles(&data);
    println!("------");
    println!("Q1: {}\nQ2: {}\nQ3: {}", q1, q2, q3);
    println!("------");
    println!(
        "Interquartile range: {:.1}",
        interquartile_range(&data, &nums)
    );

    Ok(())
}
