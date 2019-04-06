extern crate euler;

#[cfg(test)]
mod tests {

    #[test]
    fn test_q1() {
        assert_eq!(euler::q1(), 233_168);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(euler::fibonacci(6), 13);
    }

    #[test]
    fn test_q2() {
        assert_eq!(euler::q2(), 4_613_732);
    }

    #[test]
    fn test_is_palindromic_1() {
        assert!(euler::is_palindromic(2));
    }

    #[test]
    fn test_is_palindromic_3() {
        assert!(euler::is_palindromic(131));
    }

    #[test]
    fn test_is_palindromic_3_not() {
        assert!(!euler::is_palindromic(123));
    }

    #[test]
    fn test_is_palindromic_4() {
        assert!(euler::is_palindromic(2332));
    }

    #[test]
    fn test_is_palindromic_4_not() {
        assert!(!euler::is_palindromic(2323));
    }

    #[test]
    fn test_q4() {
        assert_eq!(euler::q4(), 906_609);
    }

}
