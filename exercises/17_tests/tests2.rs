// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.



fn increment(a: i32) -> i32 {
    a + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        let a = 1;
        let target = 2;
        let result = increment(a);
        assert_eq!(target, result);
    }
}

