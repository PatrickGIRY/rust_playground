fn fibo(n: u32) -> u32 {
    if n == 1 || n == 2 { return 1; }
    fibo(n - 1) + fibo(n - 2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for1() {
        assert_eq!(fibo(1), 1);
    }

    #[test]
    fn for2() {
        assert_eq!(fibo(2), 1);
    }

    #[test]
    fn for3() {
        assert_eq!(fibo(3), 2);
    }

    #[test]
    fn for4() {
        assert_eq!(fibo(4), 3);
    }

    #[test]
    fn for5() {
        assert_eq!(fibo(5), 5);
    }

    #[test]
    fn for6() {
        assert_eq!(fibo(6), 8);
    }
}
