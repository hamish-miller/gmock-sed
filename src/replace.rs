use std::fmt;
use colored::*;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::regexes::{REPLACE_REGEX, MACRO_REGEX, SIG_REGEX, ARG_REGEX, CALLTYPE_REGEX};

pub fn replace(src: &str) -> ReplaceSummary {
    lazy_static! {
        static ref RE: Regex = Regex::new(REPLACE_REGEX).unwrap();
    }
    let mut err: Vec<String> = Vec::new();
    let mut counter = 0;

    let new = RE.replace_all(src, |caps: &Captures| {
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
    _args: Args,
}

#[derive(Debug, Clone)]
struct ParseSignatureError;

impl Signature {
    fn from_str(s: &str) -> Result<Self, ParseSignatureError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(SIG_REGEX).unwrap();
        }
        if let Some(c) = RE.captures(s) {
            Ok(Signature {
                _return: String::from(c.get(2).unwrap().as_str()),
                _name: String::from(c.get(1).unwrap().as_str()),
                _args: Args::from_str(c.get(3).unwrap().as_str()).unwrap(),
            })
        } else {
        println!("foo");
            Err(ParseSignatureError)
        }
    }

    fn to_string(&self) -> String {
        format!("{}, {}, ({})", self._return, self._name, self._args.to_string())
    }
}

struct Args(String);

#[derive(Debug, Clone)]
struct ParseArgError;

impl Args {
    fn from_str(s: &str) -> Result<Self, ParseArgError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(ARG_REGEX).unwrap();
        }

        Ok(Args(RE.find(s).unwrap().as_str().to_string()))
    }

    fn to_string(&self) -> String {
        match self.0.len() {
            0 => String::new(),
            _ if self.void() => String::new(),
            _ => self.0.clone(),
        }
    }

    fn void(&self) -> bool {
        self.0.trim() == "void"
    }
}

struct Qualifiers {
    _const: bool,
    _count: usize,
    _calltype: Option<String>,
}

impl Qualifiers {
    fn from_str(s: &str, p: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(MACRO_REGEX).unwrap();
        }
        let c = RE.captures(s).unwrap();
        Qualifiers {
            _const: c.get(1).is_some(),
            _count: c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            _calltype: c.get(4).map(|_| {
                lazy_static! {
                    static ref RE: Regex = Regex::new(CALLTYPE_REGEX).unwrap();
                }

                RE.find(p).unwrap().as_str().to_string()
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

