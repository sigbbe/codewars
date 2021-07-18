mod _29_06_2021 {
    pub mod build_a_pile_of_cubes;
    pub mod find_the_divisor;
    pub mod iq_test;
    pub mod multiple_of_3_or_5;
    pub mod typed;
}

mod _30_06_2021 {
    pub mod n_queens_problem;
    pub mod rgb_to_hex_conversion;
    pub mod tortoise_racing;
}

mod _01_07_2021 {
    pub mod algo;
    pub mod phone_directory;
}

mod _13_07_2021 {
    pub mod anagram;
    pub mod array_diff;
    pub mod callback;
    pub mod fibo_akin;
    pub mod stop_gninnips_my_sdrow;
}

mod _14_07_2021 {
    pub mod if_you_can_read_this;
    pub mod molecule_to_atoms;
}

mod _17_07_2021 {
    pub mod snail;
    pub mod is_my_friend_cheating;
    pub mod factorial_tail;
    pub mod mix;
}

fn main() {
    // codewars_29_06_2021();
    // codewars_30_06_2021();
    // codewars_01_07_2021();
    // codewars_13_07_2021();
    // codewars_14_07_2021();
    codewars_17_07_2021();
}

#[allow(dead_code)]
fn codewars_01_07_2021() {
    #[allow(unused_imports)]
    use _01_07_2021::algo::*;
}

#[allow(dead_code)]
fn codewars_30_06_2021() {
    // rgb_to_hex_conversion
    {
        use std::collections::HashMap;
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
    // tortoise_racing
    {
        println!("{:?}", _30_06_2021::tortoise_racing::race(80, 91, 37));
    }
    // n_queens_problem
    {
        _30_06_2021::n_queens_problem::main();
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn codewars_13_07_2021() {
    _13_07_2021::anagram::tests::sample_tests();
    _13_07_2021::stop_gninnips_my_sdrow::run();
    _13_07_2021::fibo_akin::run();
    _13_07_2021::array_diff::run();
    _13_07_2021::callback::run();
}

#[allow(dead_code)]
fn codewars_14_07_2021() {
    _14_07_2021::if_you_can_read_this::run();
    _14_07_2021::molecule_to_atoms::run();
}

#[allow(dead_code)]
fn codewars_17_07_2021() {
    // _17_07_2021::snail::run();
    // _17_07_2021::is_my_friend_cheating::run();
    // _17_07_2021::factorial_tail::run();
    _17_07_2021::mix::run();
}
