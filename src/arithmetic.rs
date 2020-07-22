/// arithmetic_gcd computes the Greatest Common Divisor of two positive integers.
///
/// * `a` - first positive integer
/// * `b` - second positive integer
///
/// the Greatest Common Divisor of 0 and 0 is computed as 0
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

/// arithmetic_lcm computes the Least Common Multiple of two positive integers.
///
/// * `a` - first positive integer
/// * `b` - second positive integer
pub fn arithmetic_lcm(a: u32, b: u32) -> u32 {
    if a & b > 0 {
        (a * b) / arithmetic_gcd(a, b)
    } else {
        0
    }
}
