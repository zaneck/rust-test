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
