use super::common::ProgramError;
use std::collections::HashMap;
use std::io;

/// Cacluclates the mean value of the values given by the `data` vector.
///
/// # Example
///
/// ```
/// assert_eq!(mean(&vec![1, 2, 3]), 2.0);
/// ```
pub fn mean(data: &Vec<i32>) -> f32 {
    data.iter().fold(0, |acc, x| acc + x) as f32 / data.len() as f32
}

/// Calculates the weighted mean of the values given by the `data` vector. The `weights` are
/// applied in order.
///
/// # Example
///
/// ```
/// let data = vec![1, 2, 4];
/// let weights = vec![10, 5, 1];
///
/// assert_eq!(weighted_mean(&data, &weights), 1.5);
/// ```
pub fn weighted_mean(data: &Vec<i32>, weights: &Vec<i32>) -> f32 {
    let upper = data
        .iter()
        .zip(weights.iter())
        .fold(0, |acc, (d, w)| acc + d * w) as f32;
    let lower = weights.iter().fold(0, |acc, x| acc + x) as f32;

    upper / lower
}

/// Calculates the median of the values given by the `data` vector.
///
/// # Example
///
/// ```
/// assert_eq!(median(&vec![1, 2, 3]), 2.0);
/// assert_eq!(medain(&vec![1, 3]), 2.0);
/// ```
pub fn median(data: &Vec<i32>) -> f32 {
    let mut copy = data.clone();
    copy.sort();
    match copy.len() % 2 {
        0 => mean(&vec![copy[copy.len() / 2 - 1], copy[copy.len() / 2]]),
        1 => copy[copy.len() / 2] as f32,
        _ => unreachable!(),
    }
}

/// Calculates the mode of the values given by the `data` vector.
///
/// # Example
///
/// ```
/// assert_eq!(mode(&vec![1, 1, 2, 3]), 1);
/// assert_eq!(mode(&vec![1, 2, 2, 3, 3]), 2);
/// ```
pub fn mode(data: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in data.iter() {
        *map.entry(*i).or_insert(0) += 1;
    }

    let (_, max) = map.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
    let mut filtered = map
        .iter()
        .filter(|(_, v)| *v == max)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    filtered.sort();
    *filtered[0]
}

/// Runns all the Day0 Challenges
pub fn run() -> Result<(), ProgramError> {
    println!("------------------------------");
    println!("Day 0");
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

    input.clear();
    io::stdin().read_line(&mut input)?;
    let weights = input
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

    if data.len() != weights.len() {
        return Err(ProgramError::Logic(format!(
            "data.len: {}, weights.len(): {}",
            data.len(),
            weights.len()
        )));
    }

    println!("mean:          {}", mean(&data));
    println!("median:        {}", median(&data));
    println!("mode:          {}", mode(&data));
    println!("weighted mean: {}", weighted_mean(&data, &weights));

    Ok(())
}
