fn factorial(num: u64) -> u64 {
    (2..=num).reduce(|acc, n| acc * n).unwrap_or(1)
}

fn main() {
    let num = 0u64;
    println!("{:?}", (2..=num).collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
