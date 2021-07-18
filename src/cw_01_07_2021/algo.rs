mod memo {
    #[allow(dead_code)]
    pub fn fib(n: u32) -> u32 {
        let mut memo: Vec<u32> = vec![0, 1];
        for i in 2..=n {
            let res = memo[(i - 1) as usize] + memo[(i - 2) as usize];
            memo.push(res);
        }
        return memo[memo.len() - 1];
    }
}

mod rec {
    #[allow(dead_code)]
    pub fn fib(n: u32) -> u32 {
        match n {
            n if n < 2 => 1,
            _ => fib(n - 1) + fib(n - 2),
        }
    }
}
