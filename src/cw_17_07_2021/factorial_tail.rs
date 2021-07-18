
#[allow(dead_code)]
fn zeroes(base: i32, number: i32) -> i32 {
    println!("base: {}\nn: {}", base, number);
    let mut trailingzeroes: u64  = 0;
    let mut factorial: u64 = factorial(number as u64);
    println!("fac({})={}", number, factorial);
    let mut i = 0;
    while factorial % base as u64 == 0 {
        i += 1;
        factorial /= base as u64;
        trailingzeroes += 1;
        if i > 100 {
            break;
        }
    }
    trailingzeroes as i32
}

#[allow(dead_code)]
fn factorial(n: u64) -> u64 {
    let mut fac = 1;
    for i in 2..(n + 1) {
        fac *= i;
    }
    fac
}

mod math {
    #[allow(dead_code)]
    pub fn abs(n: i64) -> u64 {
        if n < 0 {
            (-1 * n) as u64
        } else {
            n as u64
        }
    }
    
    #[allow(dead_code)]
    pub fn karatsuba(_x: u64, _y: u64) {}
    
}

#[allow(dead_code)]
pub fn run() {
    let x = 1;
    let y = 2;
    let _z = math::karatsuba(x, y);
    // let z = zeroes(10, 10);
    println!("{}", x);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    
    #[allow(dead_code)]
    fn do_test() {

    }
}