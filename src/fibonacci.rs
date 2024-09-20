pub const MAX_N: usize = 46;

pub fn fibonacci(first: u32, second: u32, n: usize) -> u64 {
    assert!(n > 1 && n <= MAX_N);

    let mut prev1 = first as u64;
    let mut prev2 = second as u64;
    let mut current = 0;

    for _ in 2..=n {
        current = prev1 + prev2;

        prev1 = prev2;
        prev2 = current;
    }

    current
}
