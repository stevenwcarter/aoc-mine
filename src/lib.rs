mod coord;
mod grid;

pub use coord::*;
pub use grid::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn lcm_gcd_test() {
        let a = 12;
        let b = 15;
        assert_eq!(gcd(a, b), 3);
        assert_eq!(lcm(a, b), 60);
    }
}
