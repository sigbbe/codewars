// Written by: https://www.codewars.com/users/AurevoirXavier

/**
 * Given two numbers n, k (integers > 2). The function
 * returns the number of terms u[i] >= k with 1 <= i <= n.
 * If we look above we can see that between u[1] and u[23]
 * we have four u[i] greater or equal to 12:
 * length_sup_u_k(23, 12) => 4      
 **/
#[allow(dead_code)]
fn length_sup_uk(n: i32, k: i32) -> i32 {
    u(n).into_iter().filter(|&x| x >= k).count() as i32
}

/**
 * Given n (integer > 2) write the function comp(n) returning
 * the number of times where a term of u is less than its predecessor
 * up to and including u[n].
 **/
#[allow(dead_code)]
fn comp(n: i32) -> i32 {
    let un = u(n);
    let mut count = 0;
    for i in 2..un.len() {
        if un[i] < un[i - 1] {
            count += 1;
        }
    }
    count
}

// An example: let us calculate u[13].
// At indexes 12 and 11 we have 8 and 6.
// Going backwards of 8 and 6 from 13 we
// get indexes 13 - 8 = 5 and 13 - 6 = 7.
// u[5] = 3 and u[7] = 5 so u[13] = u[5] + u[7] = 3 + 5 = 8

/**
 * Express u(n) as a function of n, u[n - 1], u[n - 2]
 **/
fn u(n: i32) -> Vec<i32> {
    let mut un = vec![1, 1];
    for i in 2..n as usize {
        let next = un[i - un[i - 1] as usize] + un[i - un[i - 2] as usize];
        un.push(next);
    }
    println!("u[{}] = {:?}", n, un);
    un
}

pub fn run() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest1(n: i32, k: i32, exp: i32) -> () {
        assert_eq!(length_sup_uk(n, k), exp)
    }
    fn dotest2(n: i32, exp: i32) -> () {
        assert_eq!(comp(n), exp)
    }

    #[test]
    fn test_length_sup_uk() {
        dotest1(50, 25, 2);
        dotest1(3332, 973, 1391);
        dotest1(2941, 862, 1246);
        dotest1(3177, 573, 2047);
    }
    #[test]
    fn test_comp() {
        dotest2(74626, 37128);
        dotest2(71749, 35692);
        dotest2(56890, 28281);
        dotest2(60441, 30054);
    }
}
