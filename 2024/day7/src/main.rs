use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

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
        println!("{:?}", equation);
        let num_operators = equation.1.len() - 1;
        if num_operators < 1 {
            break;
        }

        let all_combs: i64 = 3_i64.pow(num_operators as u32);
        if (0..all_combs).any(|mut comb| {
            equals_result(
                equation.0,
                0,
                &mut VecDeque::from(equation.1.clone()),
                &mut comb
            )
        }) {
            tot_cal_2 += equation.0;
        }
    }

    println!("{}",  tot_cal_2);

    Ok(())
}

fn equals_result(result: i64, curr: i64, parts: &mut VecDeque<i64>, comb: &mut i64) -> bool {
    if curr > result {
        return false;
    }
    match parts.pop_front() {
        Some(num) => {
            let new_curr;
            if curr == 0 {
                new_curr = num;
            } else {
                match *(comb) % 3 {
                    0 => new_curr = curr.checked_add(num).unwrap_or(i64::MAX),
                    1 => new_curr = curr.checked_mul(num).unwrap_or(i64::MAX),
                    2 => {
                        let concat = curr.to_string() + &num.to_string();
                        new_curr = concat.parse::<i64>().expect("Should be fine?");
                    }
                    _ => panic!("How did this happen?"),
                }
                *comb /= 3;
            }

            equals_result(result, new_curr, parts, comb)
        }
        None => result == curr,
    }
}
