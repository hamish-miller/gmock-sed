/// Extract substring from matching parentheses.

use crate::errors::UnmatchedParenthesisError;

type Result<T> = std::result::Result<T, UnmatchedParenthesisError>;

pub fn lextract(s: &str) -> Result<&str> {
    assert!(s.starts_with('('));
    let mut n = 0;

    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => n += 1,
            ')' => n -= 1,
            _ => {},
        }

        if n == 0 { return Ok(&s[1..i]) }
    }

    Err(UnmatchedParenthesisError)
}

pub fn rextract(s: &str) -> Result<&str> {
    assert!(s.ends_with(')'));
    let mut n = 0;

    for (i, c) in s.chars().rev().enumerate() {
        match c {
            ')' => n += 1,
            '(' => n -= 1,
            _ => {},
        }

        if n == 0 { return Ok(&s[(s.len() - i)..(s.len() - 1)]) }
    }

    Err(UnmatchedParenthesisError)
}


#[cfg(test)]
mod tests {
    use super::*;
    const DEMO: &str = "(abc(df))(xy)";

    #[test]
    fn test_lextract() {
        assert_eq!(lextract(DEMO), Ok("abc(df)"));
    }

    #[test]
    fn test_lextract_unmatched() {
        assert_eq!(lextract("(abc(df)"), Err(UnmatchedParenthesisError));
    }

    #[test]
    fn test_rextract() {
        assert_eq!(rextract(DEMO), Ok("xy"));
    }

    #[test]
    fn test_rextract_unmatched() {
        assert_eq!(rextract("xy)"), Err(UnmatchedParenthesisError));
    }
}
