mod bruteforce {
    use super::*;

    #[allow(dead_code)]
    pub fn next_bigger_number(n: i64) -> i64 {
        let mut all_perm = vec_unique_elements(all_permutations(n));
        all_perm.sort();
        println!("{:?}", all_perm);
        let k = all_perm.len();
        for i in 0..k {
            if (all_perm[i] == n) & (i + 1 < k) {
                return all_perm[i + 1];
            }
        }
        -1
    }
}

mod solution {
    // https://www.codewars.com/kata/55983863da40caa2c900004e/solutions
    #[allow(dead_code)]
    pub fn next_bigger_number(n: i64) -> i64 {
        // n = 918651
        // represent number as vector of chars in Big Endian notation
        // 1 5 6 8 1 9
        let mut digits = n.to_string().chars().rev().collect::<Vec<char>>();

        // start from the left and find digit which less then previous one
        // 1 5 6 8 [1] 9
        let i = match (0..digits.len()-1).position(|i| digits[i] > digits[i+1]) {
            Some(i) => i+1,
            None => return -1,
        };

        // find index 'j' of first digit from the left
        // which grater than current digit
        // 1 |5| 6 8 [1] 9
        let j = (0..i).position(|j| digits[j] > digits[i] ).unwrap();

        // swap current digit with previous smallest greater
        // 1 |1| 6 8 [5] 9
        digits.swap(j,i);

        // reverse sort left part
        // 8 6 1 1 [5] 9
        digits[0..i].sort_by_key(|&i| std::cmp::Reverse(i));

        // make string from vector then parse string as i64
        digits.into_iter().rev().collect::<String>().parse().unwrap()

    }
}

use std::iter::FromIterator;

use codewars::vec_unique_elements;
use codewars::all_permutations;
#[allow(unused_imports)]
use codewars::exchange;
#[allow(unused_imports)]
use codewars::permutation;

#[allow(dead_code)]
fn next_bigger_number(_n: i64) -> i64 {
    let n_str = _n.to_string();
    if _n < 12 {
        -1
    } else {
        let chars = n_str.chars().collect::<Vec<char>>();
        let k = chars.len();
        for i in 2..=k {
            let i_last_digits_chars: Vec<char> = chars[k - i..k].to_vec();
            let num_last_i_digits: i64 = String::from_iter(i_last_digits_chars).parse::<i64>().unwrap();
            let mut permutations_of_last_digits: Vec<i64> = vec_unique_elements(all_permutations(num_last_i_digits));
            permutations_of_last_digits = permutations_of_last_digits.iter().filter(|v| v.to_string().len() == i).map(|v| *v).collect::<Vec<i64>>();
            permutations_of_last_digits.sort();
            let number_of_permutations = permutations_of_last_digits.len();
            if number_of_permutations < 2{
                continue;
            }
            for j in 0..number_of_permutations {
                if !(permutations_of_last_digits[j] == num_last_i_digits) {
                    continue;
                }
                if j + 1 >= number_of_permutations {
                    continue;
                }
                if k == number_of_permutations {
                        return permutations_of_last_digits[j + 1];
                } else {
                    return {
                        let mut str_res = String::from_iter(chars[0..k - i].iter());
                        str_res.push_str(&permutations_of_last_digits[j + 1].to_string());
                        str_res.parse::<i64>().unwrap_or(-1)
                    };
                }
            }
        }
        -1
    }
}

pub fn run() {
    let i: &[i64; 8] = &[12300, 54003, 12234, 513, 2017, 144, 513, 5275513];
    for j in i {
        println!("{}\t\t=>\t{}", j, solution::next_bigger_number(*j));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21,  next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071,next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
    }
}
