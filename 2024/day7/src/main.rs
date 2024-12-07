use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let equations: Vec<(i64, Vec<i64>)> = reader.lines()
    .map(|line| {
        let line = line.expect("yeah");
        let mut parts = line.split(":");
        let target = parts.next().expect("lhs").trim().parse::<i64>().expect("i64");
        let nums = parts.next().expect("rhs")
            .split_whitespace()
            .map(|n| n.parse::<i64>().expect("Valid i64"))
            .collect::<Vec<i64>>();
        (target, nums)
    }).collect();

    // Part 1
    let mut tot_calibration = 0;
    for equation in &equations {
        let num_operators = equation.1.len() - 1;
        if num_operators < 1 {
            break;
        }

        let all_combs: i64 = 1 << num_operators;
        if (0..all_combs).any(|comb| {
            let result = equation.1.iter().enumerate().fold(
                0, |acc, (i, &num)| {
                    if i == 0 {
                        acc + num
                    } else if (comb >> (i - 1)) & 1 == 1 {
                        acc * num // Apply multiplication
                    } else {
                        acc + num // Apply addition
                    }
                });
            result == equation.0
        }) {
            tot_calibration += equation.0;
        }
    }

    println!("{}",  tot_calibration);

    // Part 2
    let mut tot_cal_2 = 0;
    for equation in &equations {
        let num_operators = equation.1.len() - 1;
        if num_operators < 1 {
            break;
        }

        let all_combs: i64 = 1 << (num_operators * 2);
        if (0..all_combs).any(|comb| {
            let result = equation.1.iter().enumerate().fold(
                Some(0i64), |acc, (i, &num)| {
                    if acc.is_none() {
                        return None;
                    }
                    if i == 0 {
                        return acc.expect("Is ok").checked_add(num);
                    }
                    match (comb >> (i - 1) * 2) & 3 {
                        0 => acc.expect("Is ok").checked_add(num),
                        1 => acc.expect("Is ok").checked_mul(num),
                        2 => {
                            let concat = acc?.to_string() + &num.to_string();
                            Some(concat.parse::<i64>().expect("Should be fine?"))
                        }
                        3 => None,
                        _ => panic!("How did this happen?"),
                    }
                });
            result == Some(equation.0)
        }) {
            tot_cal_2 += equation.0;
        }
    }

    println!("{}",  tot_cal_2);

    Ok(())
}