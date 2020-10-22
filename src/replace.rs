use std::fmt;
use colored::*;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::errors::{ParseArgError, ParseSignatureError};
use crate::extract::{lextract, rextract};
use crate::regexes::{REPLACE_REGEX, MACRO_REGEX, SIG_REGEX, CALLTYPE_REGEX};

pub fn replace(src: &str) -> ReplaceSummary {
    lazy_static! {
        static ref RE: Regex = Regex::new(REPLACE_REGEX).unwrap();
    }

    let mut err: Vec<String> = Vec::new();
    let mut counter = 0;

    let new = RE.replace_all(src, |caps: &Captures| {
        counter += 1;
        let original = &caps[0];

        let parameters = lextract(&caps[2]);

        let q = Qualifiers::new(&caps[1]).calltype(parameters);

        let s = match Signature::new(q.strip_self(parameters)) {
            Ok(s) => s,
            Err(_e) => {
                err.push(format!("ParseSignatureError:\t{}", original));
                return String::from(original)
            },
        };

        MockMethod::new(s, q, caps.get(3).map(|m| m.as_str())).to_string()
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
    _semicolon: bool,
}

impl MockMethod {
    fn new(s: Signature, q: Qualifiers, sc: Option<&str>) -> Self {
        MockMethod {
            _signature: s,
            _qualifiers: q,
            _semicolon: sc == Some(";")
        }
    }

    fn semicolon(&self) -> &'static str {
        if self._semicolon { ";" } else { "" }
    }
}

impl fmt::Display for MockMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (s, q) = (self._signature.to_string(), self._qualifiers.to_string());

        write!(f, "MOCK_METHOD({}{}){}", s, q, self.semicolon())
    }
}

struct Signature {
    _return: String,
    _name: String,
    _args: Args,
}

impl Signature {
    fn new(s: &str) -> Result<Self, ParseSignatureError> {
        let args = Args::new(s)?;
        let rest = args.strip_self(s);

        lazy_static! {
            static ref RE: Regex = Regex::new(SIG_REGEX).unwrap();
        }

        if let Some(c) = RE.captures(rest) {
            if let (Some(n), Some(r)) = (c.get(1), c.get(2)) {
                return Ok(Signature {
                    _return: r.as_str().to_string(),
                    _name: n.as_str().to_string(),
                    _args: args,
                })
            }
        }

        Err(ParseSignatureError)
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, n, a) = (&self._return, &self._name, self._args.to_string());
        write!(f, "{}, {}, ({})", r, n, a)
    }
}

struct Args(String);

impl Args {
    fn new(s: &str) -> Result<Self, ParseArgError> {
        Ok(Args(rextract(s).to_owned()))
    }

    fn strip_self<'a>(&self, s: &'a str) -> &'a str {
        &s[..(s.len() - (self.0.len() + 2))]
    }

    fn empty(&self) -> bool {
        self.0.is_empty()
    }

    fn void(&self) -> bool {
        self.0.trim() == "void"
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.empty() || self.void() {
            write!(f, "")
        } else {
            write!(f, "{}", &self.0)
        }
    }
}

#[derive(Debug)]
struct Qualifiers {
    _const: bool,
    _count: usize,
    _calltype: Option<String>,
}

impl Qualifiers {
    fn new(_macro: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(MACRO_REGEX).unwrap();
        }

        let c = RE.captures(_macro).unwrap();
        Qualifiers {
            _const: c.get(1).is_some(),
            _count: c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            _calltype: c.get(4).map(|_| String::new()),
        }
    }

    fn calltype(mut self, params: &str) -> Self {
        if self._calltype.is_none() { return self }

        lazy_static! {
            static ref RE: Regex = Regex::new(CALLTYPE_REGEX).unwrap();
        }

        self._calltype = RE.find(params).map(|m| m.as_str().to_owned());
        self
    }

    fn strip_self<'a>(&self, s: &'a str) -> &'a str {
        &s[self.len()..]
    }

    fn len(&self) -> usize {
        self._calltype.as_ref().map_or(0, |ct| ct.len())
    }
}

impl fmt::Display for Qualifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self._const, self._calltype.as_ref()) {
            (false, None)     => write!(f, ""),
            (true,  None)     => write!(f, ", (const)"),
            (false, Some(ct)) => write!(f, ", (Calltype({}))", &ct),
            (true,  Some(ct)) => write!(f, ", (const, Calltype({}))", &ct),
        }
    }
}

