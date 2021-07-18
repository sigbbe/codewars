// Bob is preparing to pass IQ test. The most frequent task
// in this test is to find out which one of the given numbers
// differs from the others. Bob observed that one number usually
// differs from the others in evenness. Help Bob — to check his
// answers, he needs a program that among the given numbers
// finds one that is different in evenness, and return a position
// of this number.
// NB: the task is to help Bob solve a real IQ test, which means
// indexes of the elements start from 1 (not 0).

#[allow(unused_imports)]
use crate::cw_29_06_2021::typed::type_name::Typed;

#[allow(dead_code)]
pub fn iq_test(input: &str) -> u64 {
    // Convert the input string of the form "1 2 3 5 2 5 6" to a vector of integers
    let input_arr = input
        .trim()
        .split(" ")
        .map(|v| v.parse::<u64>().ok().unwrap())
        .collect::<Vec<u64>>();
    // [true: even, false: odd]
    let is_even_slice = input_arr.iter().map(|v| is_even(*v)).collect::<Vec<bool>>();
    let number_of_even_numbers = is_even_slice
        .iter()
        .fold(0, |accumulator, next_value| -> u32 {
            if *next_value {
                accumulator + 1
            } else {
                accumulator
            }
        });
    let number_of_odd_numbers = (is_even_slice.len() as u32) - number_of_even_numbers;
    // println!("{:?}", input_arr);
    // println!("{:?}", is_even_slice);
    // println!("{:?}", (number_of_even_numbers, number_of_odd_numbers));
    if number_of_odd_numbers == number_of_even_numbers {
        return 0;
    }
    for i in 0..is_even_slice.len() {
        if is_even_slice[i] == (number_of_odd_numbers > number_of_even_numbers) {
            return (i + 1) as u64;
        }
    }
    0
}

#[allow(dead_code)]
fn is_even(n: u64) -> bool {
    n % 2 == 0
}
