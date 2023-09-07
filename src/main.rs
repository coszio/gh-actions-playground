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
    #[cfg(target_os = "linux")]
    fn flaky_test() {
        assert!(rand::thread_rng().gen_bool(0.4))
    }

    #[test]
    #[ignore]
    fn failing_test() {
        assert!(false)
    }
}
