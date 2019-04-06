pub fn is_palindromic(n: u64) -> bool {
    let s = n.to_string();
    let m = s.len() / 2;
    let xs = s.bytes().take(m);
    let ys = s.bytes().rev().take(m);
    xs.zip(ys).all(|(x, y)| x == y)
}

pub fn solve() -> u64 {
    let mut m = 0;
    for i in 100..999 {
        for j in (100..999).rev() {
            if i < j {
                continue;
            }
            let p = i * j;
            if is_palindromic(p) {
                if m < p {
                    m = p;
                }
                break;
            }
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindromic_1() {
        assert!(is_palindromic(2));
    }

    #[test]
    fn is_palindromic_3() {
        assert!(is_palindromic(131));
    }

    #[test]
    fn is_not_palindromic_3() {
        assert!(!is_palindromic(123));
    }

    #[test]
    fn is_palindromic_4() {
        assert!(is_palindromic(2332));
    }

    #[test]
    fn is_not_palindromic_4() {
        assert!(!is_palindromic(2323));
    }

    #[test]
    fn check() {
        assert_eq!(solve(), 906_609);
    }
}
