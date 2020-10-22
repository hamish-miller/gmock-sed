/// Extract substring from matching parentheses.

pub fn lextract(s: &str) -> &str {
    assert!(s.starts_with('('));
    let mut n = 0;

    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => n += 1,
            ')' => n -= 1,
            _ => {},
        }

        if n == 0 { return &s[1..i] }
    }

    panic!("Unmatched parenthesis")
}

pub fn rextract(s: &str) -> &str {
    assert!(s.ends_with(')'));
    let mut n = 0;

    for (i, c) in s.chars().rev().enumerate() {
        match c {
            ')' => n += 1,
            '(' => n -= 1,
            _ => {},
        }

        if n == 0 { return &s[(s.len() - i)..(s.len() - 1)] }
    }

    panic!("Unmatched parenthesis")
}


#[cfg(test)]
mod tests {
    use super::*;
    const DEMO: &str = "(abc(df))(xy)";

    #[test]
    fn test_lextract() {
        assert_eq!(lextract(DEMO), "abc(df)");
    }

    #[test]
    fn test_rextract() {
        assert_eq!(rextract(DEMO), "xy");
    }
}
