pub fn check_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::check_prime;

    #[test]
    fn test_9() {
        assert_eq!(check_prime(9), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(check_prime(3), true);
    }

    #[test]
    fn test_9293() {
        assert_eq!(check_prime(9293), true);
    }

    #[test]
    fn test_18446744073709551615() {
        assert_eq!(check_prime(18446744073709551615), false);
    }
}
