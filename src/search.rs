use std::fmt;
use regex::Regex;

use crate::regexes::MOCK_METHOD_REGEX;

pub fn search(source: &str, mode: SearchMode) -> SearchSummary {
    let re = Regex::new(MOCK_METHOD_REGEX).unwrap();
    use SearchMode::*;
    match mode {
        Lazy => SearchSummary::from(re.is_match(source)),
        Full => SearchSummary::from(re.find_iter(source).count()),
    }
}

#[derive(Copy, Clone)]
pub enum SearchMode {
    Lazy,
    Full,
}

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

