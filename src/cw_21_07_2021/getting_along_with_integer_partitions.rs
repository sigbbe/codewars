
// https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b/rust


mod solution {}
use codewars::remove_duplicates;

#[allow(dead_code)]
fn part(n: i64) -> String {
    let mut factors = partition_factors(n);
    let range = range(&factors);
    let average = average(&factors);
    let median = median(&mut factors);
    format!("Range: {} Average: {:.2} Median: {:.2}", range, average, median)
}

#[allow(dead_code)]
fn range(v: &Vec<i64>) -> i64 {
    let max: i64 = match v.iter().max() {
        Some(m) => *m, 
        None => return -1
    }; 
    let min: i64 = match v.iter().min() {
        Some(m) => *m, 
        None => return -1
    };
    max - min
}

#[allow(dead_code)]
fn average(v: &Vec<i64>) -> f64 {
    v.iter().fold(0, |acc, x| acc + x) as f64 / (v.len() as f64)
}

#[allow(dead_code)]
fn median(v: &mut Vec<i64>) -> f64 {
    v.sort();

    let mid = v.len() / 2;
    if v.len() % 2 == 0 {
        average(&vec![v[mid - 1], v[mid]]) as f64
    } else {
        v[mid] as f64
    }
}

fn partition_factors(n: i64) -> Vec<i64> {
    // Calculate product of each sub-vector of integer_partition(n)
    let mut n_partitions = integer_partition(n).iter().map(|sub_vec| sub_vec.iter().fold(1, |acc, x| acc * x)).collect::<Vec<i64>>();
    
    // Remove duplicates
    n_partitions = remove_duplicates(&n_partitions);

    // Sort the vector
    n_partitions.sort();

    n_partitions
}

// https://jeromekelleher.net/category/combinatorics.html
#[allow(dead_code)]
fn integer_partition(n: i64) -> Vec<Vec<i64>> {
    if n < 1 || n > 50 {
        panic!("Expected 1 <= n <= 50, n={}", n)
    }
    let mut res: Vec<Vec<i64>> = Vec::new();
    let mut a: Vec<i64> = (0..n + 1).map(|_| 0 as i64).collect();
    let mut k: usize = 1;
    let l: usize = n as usize;
    a[1] = n;
    while k > 0 {
        let x = a[k - 1] + 1;
        let mut y = a[k] - 1;
        k -= 1;
        while (x <= y) & (k < l - 1) {
            a[k] = x;
            y -= x;
            k += 1;
        }
        a[k] = x + y;
        res.push(a[0..k + 1].to_vec())
    }
    res
}

#[allow(dead_code)]
pub fn run() {
    let mut i = 1;
    loop {
        if i > 5 {
            break;
        } else {
            let res = part(i);
            println!("{:?}", res);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn testequal(ans: &str, sol: &str) {
        assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
    }

    #[test]
    fn returns_expected() {
        testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
        testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
        testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
        testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
        testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
    }
}
