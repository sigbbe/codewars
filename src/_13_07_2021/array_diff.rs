// The goal in this kata is to implement a difference function,
// which subtracts one list from another and returns the result.

#[allow(unused_imports)]
use std::collections;

trait ArrayDiff<Rhs = Self> {
    type Output;
    fn array_diff(&mut self, other: &mut Rhs) -> Self::Output;
}

use std::slice::Iter;

impl<'a, T: 'a + PartialEq + Copy> ArrayDiff<Iter<'a, T>> for Iter<'a, T> {
    type Output = Vec<T>;
    fn array_diff(&mut self, other: &mut Iter<'a, T>) -> Self::Output {
        self.filter(|a| other.any(|b| (*b).ne(a)))
            .map(|t| *t)
            .collect::<Vec<T>>()
    }
}

// use std::ops::Add;

// impl<T> ArrayDiff for Box<dyn Iterator<Item = T>>
// where
//     T: PartialEq + Copy + Sized + 'static,
// {
//     type Output = Box<dyn Iterator<Item = T>>;
//     fn array_diff(&self, other: dyn ArrayDiff) -> Self::Output {
//         Box::new(&self)
//     }
// }

#[allow(dead_code)]
fn array_diff_with_iterator<T: PartialEq + Copy>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.iter()
        .filter(|a_t| b.iter().any(|b_t| (*b_t).ne(a_t)))
        .map(|t| *t)
        .collect::<Vec<T>>()
}

#[allow(dead_code)]
fn array_diff_with_loop<T: PartialEq + Copy>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut res: Vec<T> = Vec::new();
    'outer: for v_a in a {
        for v_b in &b {
            if v_b.eq(&v_a) {
                continue 'outer;
            }
        }
        res.push(v_a);
    }
    res
}

use super::callback::Processor;
use super::callback::ProcessorTrait;
use std::time::{Duration, SystemTime};

#[allow(dead_code)]
struct CallbackTimer<'a> {
    callback_processor: Processor<'a>,
    pub runtimes: Vec<Duration>,
}

#[allow(dead_code)]
impl<'a> CallbackTimer<'a> {
    fn number_of_calls(&self) -> usize {
        self.runtimes.len()
    }
    fn average_runtime(&self) -> Duration {
        let number_of_calls = self.runtimes.len() as u128;
        if number_of_calls == 0 {
            return Duration::ZERO;
        }
        let mut total = Duration::new(0, 0);
        for duration in self.runtimes.iter() {
            let tmp = total.saturating_add(*duration);
            if tmp == Duration::MAX {
                panic!("Something is wrong");
            }
            total = tmp;
        }
        let millis: u128 = total.as_nanos();
        let average_millis = millis / number_of_calls;
        Duration::from_nanos(average_millis as u64)
    }
}

impl<'a> ProcessorTrait<'a> for CallbackTimer<'a> {
    #[allow(dead_code)]
    fn set_callback(&mut self, c: impl FnMut() + 'a) {
        self.callback_processor.set_callback(c);
    }
    #[allow(dead_code)]
    fn process_events(&mut self) {
        let t_0 = SystemTime::now();
        self.callback_processor.process_events();
        match t_0.elapsed() {
            Ok(elapsed) => {
                self.runtimes.push(elapsed);
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {:?}", e);
            }
        }
    }
}

fn test_array_diff_with_iterator() {
    for _ in 0..10 {
        let arr_0: Vec<u32> = vec![1, 2, 3, 4, 6, 3, 123, 123, 4, 32, 1, 2, 1, 6, 4, 3, 3, 23];
        let arr_1: Vec<u32> = vec![
            1, 82, 33, 4, 36, 3, 1223, 1233, 4, 32, 1, 2, 1, 6, 4, 3, 3, 23,
        ];
        array_diff_with_iterator::<u32>(arr_0, arr_1);
    }
}

fn test_array_diff_with_loop() {
    for _ in 0..10 {
        let arr_0: Vec<u32> = vec![1, 2, 3, 4, 6, 3, 123, 123, 4, 32, 1, 2, 1, 6, 4, 3, 3, 23];
        let arr_1: Vec<u32> = vec![
            1, 82, 33, 4, 36, 3, 1223, 1233, 4, 32, 1, 2, 1, 6, 4, 3, 3, 23,
        ];
        array_diff_with_loop::<u32>(arr_0, arr_1);
    }
}

#[allow(dead_code)]
pub fn run() {
    let mut timer = CallbackTimer {
        callback_processor: Processor::new(test_array_diff_with_iterator),
        runtimes: Vec::new(),
    };
    // timer.process_events();
    // timer.process_events();
    // timer.process_events();
    // timer.process_events();
    // timer.process_events();
    // timer.process_events();
    // timer.process_events();
    // timer.process_events();
    // timer.process_events();
    println!("Average time: {:?}", timer.average_runtime());
    println!("Number of callbacks: {:?}", timer.number_of_calls());
    timer.set_callback(test_array_diff_with_loop);
    timer.process_events();
    timer.process_events();
    timer.process_events();
    timer.process_events();
    timer.process_events();
    timer.process_events();
    timer.process_events();
    timer.process_events();
    timer.process_events();
    println!("Average time: {:?}", timer.average_runtime());
    println!("Number of callbacks: {:?}", timer.number_of_calls());

    // let output = array_diff_with_iterator(input_0, input_1);
    // let expected = vec![2];
    // println!("output: {:?}\nexpected: {:?}", output, expected);
}
