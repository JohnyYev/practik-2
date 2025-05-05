fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    let n = ((n % len) + len) % len;
    let n = n as usize;

    let (left, right) = s.split_at(s.len() - n);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_positive() {
        assert_eq!(rotate("abcdef".to_string(), 2), "efabcd");
        assert_eq!(rotate("abcdef".to_string(), 0), "abcdef");
        assert_eq!(rotate("abcdef".to_string(), 6), "abcdef");
        assert_eq!(rotate("abcdef".to_string(), 1), "fabcde");
    }

    #[test]
    fn test_rotate_negative() {
        assert_eq!(rotate("abcdef".to_string(), -1), "bcdefa");
        assert_eq!(rotate("abcdef".to_string(), -2), "cdefab");
        assert_eq!(rotate("abcdef".to_string(), -6), "abcdef");
        assert_eq!(rotate("abcdef".to_string(), -7), "bcdefa");
    }

    #[test]
    fn test_rotate_empty() {
        assert_eq!(rotate("".to_string(), 3), "");
    }
}
