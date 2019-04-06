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

}
