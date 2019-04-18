pub fn solve() -> u64 {
    (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(solve(), 233_168);
    }
}
