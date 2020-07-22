#[cfg(test)]
mod test {
    use numeric::arithmetic::arithmetic_gcd;
    use numeric::arithmetic::arithmetic_lcm;
    
    #[test]
    fn test_gcd() {
        let a = 32;
        let b = 56;
        let c = arithmetic_gcd(a, b);

        assert_eq!(a, 32);
        assert_eq!(b, 56);
        assert_eq!(c, 8);

        assert_eq!(arithmetic_gcd(32, 56), 8);
        assert_eq!(arithmetic_gcd(101, 23), 1);
        assert_eq!(arithmetic_gcd(101, 0), 101);
        assert_eq!(arithmetic_gcd(0, 101), 101);
        assert_eq!(arithmetic_gcd(0, 0), 0);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(arithmetic_lcm(32, 56), 224);
        assert_eq!(arithmetic_lcm(101, 0), 0);
        assert_eq!(arithmetic_lcm(0, 101), 0);
    }
}
