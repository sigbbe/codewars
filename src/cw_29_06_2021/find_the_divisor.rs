// Create a function named divisors/Divisors that takes an
// integer n > 1 and returns an array with all of the integer's
// divisors(except for 1 and the number itself), from smallest to
// largest. If the number is prime return the string '(integer) is
// prime' (null in C#) (use Either String a in Haskell and
// Result<Vec<u32>, String> in Rust).

#[allow(dead_code)]
pub fn divisors(num: u32) -> Result<Vec<u32>, String> {
    if num <= 1 {
        return Err(format!("{} must be > 1", "num"));
    }
    if is_prime(num) {
        return Err(format!("{} is prime", num));
    }
    let between_1_and_num: Vec<u32> = (2..num).collect();
    Ok(between_1_and_num
        .iter()
        .filter(|v| num % **v == 0)
        .map(|v| *v)
        .collect::<Vec<u32>>())
}

#[allow(dead_code)]
fn is_prime(n: u32) -> bool {
    // Primality test using 6k+-1 optimization.
    if n <= 3 {
        return n > 1;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
