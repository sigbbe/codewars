// The rgb function is incomplete. Complete it so that
// passing in RGB decimal values will result in a hexadecimal
// representation being returned. Valid decimal values for
// RGB are 0 - 255. Any values that fall out of that range
// must be rounded to the closest valid value.
// The answer should always be 6 characters long, the shorthand
// with 3 will not work here. The following are examples of expected output values:

// kata.rgb(255, 255, 255)  -- returns FFFFFF
// kata.rgb(255, 255, 300)  -- returns FFFFFF
// kata.rgb(0, 0, 0)        -- returns 000000
// kata.rgb(148, 0, 211)    -- returns 9400D3

#[allow(dead_code)]
pub fn rgb(r: i32, g: i32, b: i32) -> String {
    let data: Vec<u8> = [r, g, b]
        .iter()
        .map(|v| -> u8 {
            if *v > 255 {
                255
            } else if *v < 0 {
                0 as u8
            } else {
                *v as u8
            }
        })
        .collect::<Vec<u8>>();
    hex::encode(&data).to_ascii_uppercase()
}

pub fn my_rgb(r: i32, g: i32, b: i32) -> String {
    [r, g, b]
        .iter()
        .map(|v| -> u8 {
            if *v > 255 {
                255
            } else if *v < 0 {
                0 as u8
            } else {
                *v as u8
            }
        })
        .map(|v| hex_convert(v as u32))
        .collect::<Vec<String>>()
        .join("")
}

pub fn hex_convert(n: u32) -> String {
    const BASE: u32 = 16;
    let letters: [&str; 16] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];
    if n < BASE {
        let mut res = String::from("0");
        res.push_str(letters[n as usize]);
        return res;
    }
    use std::collections::VecDeque;
    let mut values: VecDeque<String> = VecDeque::new();
    let mut quotient = n / BASE;
    let mut remainder = n % BASE;
    loop {
        values.push_front(String::from(letters[remainder as usize]));
        if quotient == 0 {
            break;
        }
        remainder = quotient % BASE;
        quotient = quotient / BASE;
    }
    values.into_iter().collect::<Vec<String>>().join("")
}
