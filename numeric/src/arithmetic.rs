pub fn arithmetic_gcd(a: u32, b: u32) -> u32 {
    let mut al = a;
    let mut be = b;
    while be > 0 {
        let tmp = be;
        be = al % be;
        al = tmp;
    }
    al
}

pub fn arithmetic_lcm(a: u32, b: u32) -> u32 {
    if a & b > 0 {
        (a * b) / arithmetic_gcd(a, b)
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::arithmetic_gcd;
    use super::arithmetic_lcm;

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
