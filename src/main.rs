mod _29_06_2021 {
    pub mod build_a_pile_of_cubes;
    pub mod find_the_divisor;
    pub mod iq_test;
    pub mod multiple_of_3_or_5;
    pub mod typed;
}

mod _30_06_2021 {
    pub mod rgb_to_hex_conversion;
    // pub mod which_are_in;
    pub mod n_queens_problem;
    pub mod tortoise_racing;
}

mod _01_07_2021 {
    pub mod algo;
    pub mod phone_directory;
}

fn main() {
    codewars_29_06_2021();
    codewars_30_06_2021();
    codewars_01_07_2021();
}

use std::collections::HashMap;

fn codewars_01_07_2021() {
    #[allow(unused_imports)]
    use _01_07_2021::algo::*;
    println!("{:?}", "ldsfkølsdkf");
}

fn codewars_30_06_2021() {
    // rgb_to_hex_conversion
    {
        type RGB = (i32, i32, i32);
        let mut answers: HashMap<RGB, String> = HashMap::new();
        answers.insert((255, 255, 255), "FFFFFF".to_string());
        answers.insert((255, 255, 300), "FFFFFF".to_string());
        answers.insert((0, 0, 0), "000000".to_string());
        answers.insert((148, 0, 211), "9400D3".to_string());
        answers.insert((0, 0, 0), "000000".to_string());
        answers.insert((1, 2, 3), "010203".to_string());
        answers.insert((255, 255, 255), "FFFFFF".to_string());
        answers.insert((254, 253, 252), "FEFDFC".to_string());
        answers.insert((-20, 275, 125), "00FF7D".to_string());
        for (k, v) in answers.iter() {
            let red = k.0;
            let green = k.1;
            let blue = k.2;
            let expected = v;
            let result = _30_06_2021::rgb_to_hex_conversion::my_rgb(red, green, blue);
            assert_eq!(*expected, result);
        }
    }
    // which_are_in
    {
        // let a1 = ["arp", "live", "strong"];
        // let a2 = ["lively", "alive", "harp", "sharp", "armstrong"];
        // let res: Vec<String> = _30_06_2021::which_are_in::in_array(&a1, &a2);
        // println!("{:?}", res);
    }
    // tortoise_racing
    {
        // println!("{:?}", _30_06_2021::tortoise_racing::race(80, 91, 37));
    }
    // n_queens_problem
    {
        use _30_06_2021::n_queens_problem::main;
        main();
    }
}

fn codewars_29_06_2021() {
    // // multiple_of_3_or_5
    {
        assert_eq!(
            23,
            _29_06_2021::multiple_of_3_or_5::sum_multiples_of_3_and_5(10)
        );
    }
    // // find_the_divisor
    {
        let res = _29_06_2021::find_the_divisor::divisors(10);
        assert_eq!(Ok(vec![2, 5]), res);
    }
    // // iq_test
    {
        // Third number is odd, while the rest of the numbers are even
        assert_eq!(3, _29_06_2021::iq_test::iq_test("2 4 7 8 10"));
        // Second number is even, while the rest of the numbers are odd
        assert_eq!(2, _29_06_2021::iq_test::iq_test("1 2 1 1"));
    }
    //build_a_pile_of_cubes
    {
        type PileOfCubesTest = (i64, i32);

        let test: [PileOfCubesTest; 5] = [
            (4183059834009, 2022),
            (24723578342962, -1),
            (135440716410000, 4824),
            (40539911473216, 3568),
            (26825883955641, 3218),
        ];
        for t in test.iter() {
            assert_eq!(t.1, _29_06_2021::build_a_pile_of_cubes::find_nb(t.0) as i32);
        }
    }
}
