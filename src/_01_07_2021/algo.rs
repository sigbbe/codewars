mod memo {
    pub fib(n: u32) -> u32 {
        let memo: Vec<u32> = vec![0, 1];
        for i in 2..=n {
            let res = memo[i - 1] + memo[i - 2];
            memo.push(res);
        }
        return memo[memo.len() - 1];
    }
}

mod rec {
    pub fn fib(n: u32) -> u32 {
        match n {
            n if n < 2 => 1,
            _ => fib(n - 1) + fib(n - 2),
        }
    }
}
