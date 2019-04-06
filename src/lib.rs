pub fn q1() -> u32 {
    (1..1000).filter(|x| (x % 3 == 0) || (x % 5 == 0)).sum()
}

pub fn fibonacci(i: u32) -> u32 {
    if (i == 0) || (i == 1) {
        1
    } else {
        fibonacci(i - 1) + fibonacci(i - 2)
    }
}

pub fn q2() -> u32 {
    (1u32..)
        .take_while(|&i| fibonacci(i) <= 4_000_000)
        .map(fibonacci)
        .filter(|n| n % 2 == 0)
        .sum()
}
