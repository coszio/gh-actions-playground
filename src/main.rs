fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn flaky_test() {
        assert!(rand::thread_rng().gen_bool(0.5))
    }

    #[test]
    fn failing_test() {
        assert!(false)
    }
}
