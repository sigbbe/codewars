type AB = (i32, i32);

#[allow(dead_code)]
fn remove_nb(m: i32) -> Vec<AB> {
    let mut res: Vec<AB> = Vec::new();
    if m <= 0 {
        return res;
    }
    let sequence = (1..(m + 1)).collect::<Vec<i32>>();
    let sum_of_sequence: i32 = sequence.iter().sum();
    for a in sequence.iter() {
        for b in sequence.iter().rev() {
            let product_of_a_and_b: i32 = {
                if a + b > sum_of_sequence {
                    continue;
                } else {
                    sum_of_sequence - a - b
                }
            };
            if product_of_a_and_b == a * b {
                let ans: AB = (*a, *b);
                res.push(ans);
            }
        }
    }
    res
}

#[allow(dead_code)]
fn remove_nb_clever_0(m: i32) -> Vec<AB> {
    let n: i64 = m as i64;
    let s: i64 = n * (n+1) / 2;
    ((s-n)/(n+1)..n)
        .filter(|i| (s-i) % (i+1) == 0)
        .map(|i| (i, (s-i) / (i+1)))
        .map(|(a, b)| (a as i32, b as i32))
        .collect()
}

#[allow(dead_code)]
fn remove_nb_clever_1(m: i32) -> Vec<AB> {
    let n: i64 = m as i64;
    let sum: i64 = n * (n + 1) / 2;
    let mut result = vec![];
    for a in 1..n {
        if (sum - a) % (a + 1) == 0 {
            let b = (sum - a) / (a + 1);
            if b < n {result.push((a as i32, b as i32))}
        }
    }
    result
}

use rand::prelude::*;
use rand;

#[allow(dead_code)]
pub fn run() {
    let mut rng = rand::thread_rng();
    let m: i32 = (rng.gen::<f32>() * 500.0) as i32;
    for i in m..m+10 {
        let res = remove_nb(i);
        println!("{:?}", res);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(dead_code)]
    fn test_remove_nb(m: i32, expected: Vec<AB>) {
        assert_eq!(remove_nb(m), expected);
    }

    #[test]
    fn basics_remove_nb() {
        test_remove_nb(26, vec![(15, 21), (21, 15)]);
        test_remove_nb(100, vec![]);
        test_remove_nb(101, vec![(55, 91), (91, 55)]);
        test_remove_nb(102, vec![(70, 73), (73, 70)]);
    }
}
