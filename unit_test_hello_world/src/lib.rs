pub fn my_div(x: i64, y: i64) -> Result<i64, &'static str> {
    if y != 0 {
        Ok(x / y)
    } else {
        Err("divider must not 0")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_my_div() {
        assert_eq!(Ok(2), super::my_div(4, 2));
        assert_eq!(Ok(2), super::my_div(5, 2));
        assert_eq!(Ok(3), super::my_div(6, 2));
        assert_eq!(Err("divider must not 0"), super::my_div(6, 0));
    }
}
