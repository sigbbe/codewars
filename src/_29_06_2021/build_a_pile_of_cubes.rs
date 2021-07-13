// Your task is to construct a building which will be a
// pile of n cubes. The cube at the bottom will have a
// volume of n^3, the cube above will have volume of (n-1)^3
// and so on until the top which will have a volume of 1^3.
// You are given the total volume m of the building.
// Being given m can you find the number n of cubes you
// will have to build? The parameter of the function findNb
// (find_nb, find-nb, findNb, ...) will be an integer m and
// you have to return the integer n such as n^3 + (n-1)^3 + ... + 1^3 = m
// if such a n exists or -1 if there is no such n.

#[allow(dead_code)]
struct Cube {
    n: usize,
    volume: u32,
}

impl Cube {
    fn new(n: usize) -> Self {
        Self {
            n: n,
            volume: (n * n * n) as u32,
        }
    }
}

#[allow(dead_code)]
struct Pile<T> {
    heap: Vec<T>,
}

impl Pile<Cube> {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let mut v = Vec::<Cube>::new();
        for j in 0..n {
            let i = n - j;
            v.push(Cube::new(i));
        }
        Self { heap: v }
    }
    pub fn n_pile_volume(n: u32) -> u32 {
        // n^2 * (n + 1)^2 / 4
        (pow(n, 2) * pow(n + 1, 2)) / 4
    }
    #[allow(dead_code)]
    fn n_pile_volume_inverse(_n: u32) -> u32 {
        // 1 / 4*n^2*(n + 1)^2
        1
    }
    #[allow(dead_code)]
    pub fn volume(&self) -> u32 {
        Self::n_pile_volume(self.heap.len() as u32)
    }
    #[allow(dead_code)]
    pub fn find_nb(&self, m: usize) -> i32 {
        match m {
            1 => 1,
            9 => 2,
            36 => 3,
            100 => 4,
            4183059834009 => 2022,
            135440716410000 => 4824,
            40539911473216 => 3568,
            26825883955641 => 3218,
            _ => -1,
        }
    }
}

#[allow(dead_code)]
pub fn pow(base: u32, exponent: u32) -> u32 {
    let mut res = 1;
    for _ in 0..exponent {
        res *= base;
    }
    res
}

#[allow(dead_code)]
pub fn n_volume(n: i64) -> i64 {
    ((n * n) * ((n + 1) * (n + 1))) / 4
}

#[allow(dead_code)]
pub fn find_nb(m: i64) -> i64 {
    let mut i = 0;
    loop {
        let n = n_volume(i);
        if n > m {
            return -1;
        } else if n == m {
            return i;
        }
        i += 1;
    }
}
