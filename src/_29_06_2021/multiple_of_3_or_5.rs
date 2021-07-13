// If we list all the natural numbers below 10 that are multiples
// of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Finish the solution so that it returns the sum of all the multiples
// of 3 or 5 below the number passed in.
// Note: If the number is a multiple of both 3 and 5, only count it once.
//Also, if a number is negative, return 0(for languages that do have them)

#[allow(dead_code)]
fn mutliple_of_3_or_5(num: i32) -> bool {
    (num % 3 == 0) || (num % 5 == 0)
}

#[allow(dead_code)]
pub fn sum_multiples_of_3_and_5(num: i32) -> i32 {
    let mut ans = 0;
    for i in 1..num {
        if mutliple_of_3_or_5(i) {
            ans += i;
        }
    }
    ans
}
