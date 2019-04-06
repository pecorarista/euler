pub fn fibonacci(i: u64) -> u64 {
    if i == 0 || i == 1 {
        1
    } else {
        fibonacci(i - 1) + fibonacci(i - 2)
    }
}

pub fn solve() -> u64 {
    let mut v: Vec<u64> = vec![];
    for i in 1.. {
        let f = fibonacci(i);
        v.push(f);
        if f > 4_000_000 {
            break;
        }
    }
    v.iter().filter(|&n| n % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_6() {
        assert_eq!(fibonacci(6), 13);
    }

    #[test]
    fn check() {
        assert_eq!(solve(), 4_613_732);
    }
}
