use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;

use crate::regexes::SEARCH_REGEX;

pub fn search(source: &str, mode: SearchMode) -> SearchSummary {
    lazy_static! {
        static ref RE: Regex = Regex::new(SEARCH_REGEX).unwrap();
    }
    use SearchMode::*;
    match mode {
        Lazy => SearchSummary::from(RE.is_match(source)),
        Full => SearchSummary::from(RE.find_iter(source).count()),
    }
}

#[derive(Copy, Clone)]
pub enum SearchMode {
    Lazy,
    Full,
}

#[derive(Debug, PartialEq)]
pub struct SearchSummary {
    pub is_match: bool,
    pub count: Option<usize>,
}

impl From<bool> for SearchSummary {
    fn from(is_match: bool) -> Self {
        SearchSummary { is_match: is_match, count: None }
    }
}

impl From<usize> for SearchSummary {
    fn from(count: usize) -> Self {
        SearchSummary { is_match: count > 0, count: Some(count) }
    }
}

impl fmt::Display for SearchSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.count {
            Some(c) => write!(f, ":{}", c),
            None => write!(f, ""),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleline_macro() {
        let cpp = "MOCK_METHOD1(Foo, bool(int));";

        assert!(search(&cpp, SearchMode::Lazy).is_match);
    }

    #[test]
    fn test_multiline_macro() {
        let cpp = "MOCK_METHOD1\n(\nFoo,\nbool\n(int)\n);";

        assert!(search(&cpp, SearchMode::Lazy).is_match);
    }

    mod lazy {
        use super::*;

        #[test]
        fn test_zero_match() {
            let cpp = "";
            let expected = SearchSummary { is_match: false, count: None };

            assert_eq!(search(&cpp, SearchMode::Lazy), expected);
        }

        #[test]
        fn test_single_match() {
            let cpp = "MOCK_METHOD1(Foo, bool(int))";
            let expected = SearchSummary { is_match: true, count: None };

            assert_eq!(search(&cpp, SearchMode::Lazy), expected);
        }

        #[test]
        fn test_multi_match() {
            let cpp = "MOCK_METHOD1(Foo, bool(int))\nMOCK_METHOD1(Bar, bool(int))";
            let expected = SearchSummary { is_match: true, count: None };

            assert_eq!(search(&cpp, SearchMode::Lazy), expected);
        }
    }

    mod full {
        use super::*;

        #[test]
        fn test_zero_match() {
            let cpp = "";
            let expected = SearchSummary { is_match: false, count: Some(0) };

            assert_eq!(search(&cpp, SearchMode::Full), expected);
        }

        #[test]
        fn test_single_match() {
            let cpp = "MOCK_METHOD1(Foo, bool(int))";
            let expected = SearchSummary { is_match: true, count: Some(1) };

            assert_eq!(search(&cpp, SearchMode::Full), expected);
        }

        #[test]
        fn test_multi_match() {
            let cpp = "MOCK_METHOD1(Foo, bool(int))\nMOCK_METHOD1(Bar, bool(int))";
            let expected = SearchSummary { is_match: true, count: Some(2) };

            assert_eq!(search(&cpp, SearchMode::Full), expected);
        }
    }
}
