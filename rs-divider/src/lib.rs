/// returns the result of `a` divided by `b`
pub fn div_numbers(a: f64, b: f64) -> f64 {
    a / b
}

#[cfg(test)]
mod tests {
    use super::div_numbers;

    #[test]
    fn test_from_file() {
        assert_eq!(div_numbers(4f64, 2f64), 2f64)
    }
}
