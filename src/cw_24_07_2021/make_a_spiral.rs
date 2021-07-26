mod solution {
    // Example solution from https://www.codewars.com/kata/534e01fbbb17187c7e0000c6/solutions/rust
    #[allow(dead_code)]
    fn spiralize(size: usize) -> Vec<Vec<i8>> {
        let mut spiral = vec![vec![0; size]; size];
        let mut value = 1;
        
        for j in 0..(size + 1) / 2 {
            for i in j..(size - j) {
                spiral[i][j] = value;
                spiral[j][i] = value;

                spiral[i][size - 1 - j] = value;
                spiral[size - 1 - j][i] = value;
            }

            value = (value + 1) % 2;
            
            if j < (size - 1) / 2 || spiral[j][j - 1] == 1 {
                spiral[j + 1][j] = value;
            }
        }

        spiral
    }
}

// This is my soltion
#[allow(dead_code)]
fn spiralize(size: usize) -> Vec<Vec<i8>> {
    if size < 5 {
        panic!("Because of the edge-cases for tiny spirals, the size will be at least 5.");
    };
    let mut spiral: Vec<Vec<i8>> = vec![vec![1; size]; size];
    let mut x = 0;
    let mut y = 1;
    let mut w = 1;
    let mut h = 1;
    while (h <= size - 1) & (w < size) {
        // Right
        // Down
        // Left
        // Up
        for i in x..size - w {
            spiral[y][i] = 0;
        }
        w += 1;
        x = size - w;
        for i in y..size - h {
            spiral[i][x] = 0;
        }
        h += 1;
        y = size - h;
        for i in w - 1..x {
            spiral[y][i] = 0;
        }
        h += 1;
        x = w - 1;
        for i in h..y {
            spiral[i][x] = 0;
        }
        x += 1;
        w += 1;
        y = size - y + 1;
    }
    spiral
}

fn print_spiral(size: usize) {
    let spiral = spiralize(size);
    for s in spiral {
        let line = s.iter().map(|s| {
           match s {
               0 => ".", 
               _ => "0"
           } 
        }).collect::<String>();
        println!("{}", line);
    }
}

fn print_spiral_from_vec(v: Vec<Vec<u8>>) {
    for s in v {
        let line = s.iter().map(|s| {
            match s {
                0 => ".", 
                _ => "0"
            }
        }).collect::<String>();
        println!("{}", line);
    }
}

#[allow(dead_code)]
pub fn run() {
    let n: usize = 100;
    let t = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 0, 1, 0, 1],
        vec![1, 0, 1, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ];
    print_spiral(n);
    // println!("\n");
    // print_spiral_from_vec(t);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            spiralize(8),
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}
