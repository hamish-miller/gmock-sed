mod regexes;

use std::fmt;
use colored::*;
use regex::{Captures, Regex};

use regexes::{MOCK_METHOD_REGEX, MACRO_REGEX, PARAM_REGEX, CALLTYPE_REGEX};

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


pub fn replace(src: &str) -> ReplaceSummary {
    let re = Regex::new(MOCK_METHOD_REGEX).unwrap();
    let mut err: Vec<String> = Vec::new();
    let mut counter = 0;

    let new = re.replace_all(src, |caps: &Captures| {
        counter += 1;
        let original = &caps[0];

        let q = Qualifiers::from_str(&caps[1], &caps[2]);

        match Signature::from_str(&caps[2][q.len()..]) {
            Ok(s) => { MockMethod { _signature: s, _qualifiers: q }.to_string() },
            Err(_e) => {
                err.push(format!("ParseSignatureError:\t{}", original));
                String::from(original)
            },
        }

    });

    let s = match new != src { true => Some(new.to_string()), false => None };

    ReplaceSummary { suggestion: s, total: counter, errors: err }
}

pub struct ReplaceSummary {
    pub suggestion: Option<String>,
    total: usize,
    errors: Vec<String>,
}

impl ReplaceSummary {
    pub fn error_free(&self) -> bool {
        self.errors.is_empty() && self.suggestion.is_some()
    }
}

impl fmt::Display for ReplaceSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.suggestion.is_none() {
            return write!(f, "{}",  "(0/0)".yellow())
        }

        let ratio = format!("({}/{})", self.total - self.errors.len(), self.total);

        if self.errors.is_empty() {
            write!(f, "{}", ratio.green())
        } else {
            write!(f, "{}", ratio.red())
        }
    }
}

struct MockMethod {
    _signature: Signature,
    _qualifiers: Qualifiers,
}

impl MockMethod {
    fn to_string(&self) -> String {
        let mut signature = self._signature.to_string();

        if self._qualifiers.to_bool() {
            signature = format!("{}{}", signature, self._qualifiers.to_string());
        }

        format!("MOCK_METHOD({})", signature)
    }
}

struct Signature {
    _return: String,
    _name: String,
    _args: String,
}

#[derive(Debug, Clone)]
struct ParseSignatureError;

impl Signature {
    fn from_str(s: &str) -> Result<Self, ParseSignatureError> {
        let re = Regex::new(PARAM_REGEX).unwrap();
        if let Some(c) = re.captures(s) {
            Ok(Signature {
                _return: String::from(c.get(2).unwrap().as_str()),
                _name: String::from(c.get(1).unwrap().as_str()),
                _args: Self::fix_void(c.get(3).unwrap().as_str()),
            })
        } else {
            Err(ParseSignatureError)
        }
    }

    fn fix_void(s: &str) -> String {
        if s.trim() == "void" { return String::new() }
        String::from(s)
    }

    fn to_string(&self) -> String {
        format!("{}, {}, ({})", self._return, self._name, self._args)
    }
}

struct Qualifiers {
    _const: bool,
    _calltype: Option<String>,
}

impl Qualifiers {
    fn from_str(s: &str, p: &str) -> Self {
        let re = Regex::new(MACRO_REGEX).unwrap();
        let c = re.captures(s).unwrap();
        Qualifiers {
            _const: c.get(1).is_some(),
            _calltype: c.get(2).map(|_| {
                let re = Regex::new(CALLTYPE_REGEX).unwrap();
                let m = re.find(p).unwrap();
                String::from(m.as_str())
            }),
        }
    }

    fn to_string(&self) -> String {
        match (self._const, self._calltype.as_ref()) {
            (false, Some(ct)) => format!(", (Calltype({}))", &ct),
            (true, Some(ct)) => format!(", (const, Calltype({}))", &ct),
            (false, None) => String::new(),
            (true, None) => String::from(", (const)"),
        }
    }

    fn to_bool(&self) -> bool {
        self._const || self._calltype.is_some()
    }

    fn len(&self) -> usize {
        match self._calltype.as_ref() {
            Some(ct) => ct.len(),
            None => 0,
        }
    }
}
