pub mod intcode;

#[must_use]
pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

#[must_use]
pub fn gcd_i64(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd_i64(b, a % b) }
}

#[must_use]
pub fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 && b == 0 {
        return 0;
    }
    let gcd = gcd(a, b);
    a.abs() * (b.abs() / gcd)
}

#[must_use]
pub fn lcm_i64(a: i64, b: i64) -> i64 {
    if a == 0 && b == 0 {
        return 0;
    }
    let gcd = gcd_i64(a, b);
    a.abs() * (b.abs() / gcd)
}
