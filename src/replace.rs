use std::fmt;
use colored::*;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::errors::GmockSedError;
use crate::extract::{lextract, rextract};
use crate::regexes::{REPLACE_REGEX, MACRO_REGEX, SIG_REGEX, CALLTYPE_REGEX};

pub fn replace(src: &str, add_override: bool) -> ReplaceSummary {
    lazy_static! {
        static ref RE: Regex = Regex::new(REPLACE_REGEX).unwrap();
    }

    let mut err: Vec<String> = Vec::new();
    let mut counter = 0;

    let new = RE.replace_all(src, |caps: &Captures| {
        counter += 1;
        let original = &caps[0];

        let parameters = match lextract(&caps[2].trim()) {
            Ok(s) => s,
            Err(e) => {
                err.push(format!("  {}:\t{}", e, original));
                return String::from(original)
            },
        };

        let q = Qualifiers::new(&caps[1], add_override).calltype(parameters);

        let s = match Signature::new(q.strip_self(parameters).trim(), q.argc) {
            Ok(s) => s,
            Err(e) => {
                err.push(format!("  {}:\t{}", e, original));
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

    pub fn error_summary(&self) -> String {
        self.errors.join("\n")
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
    fn new(s: &str, argc: usize) -> Result<Self, GmockSedError> {
        let args = Args::new(s, argc)?;
        let rest = args.strip_self(s);

        lazy_static! {
            static ref RE: Regex = Regex::new(SIG_REGEX).unwrap();
        }

        if let Some(c) = RE.captures(rest.trim()) {
            if let (Some(n), Some(r)) = (c.get(1), c.get(2)) {
                return Ok(Signature {
                    _return: protect(r.as_str().trim()),
                    _name: n.as_str().trim().to_owned(),
                    _args: args,
                })
            }
        }

        Err(GmockSedError::ParseSignatureError)
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, n, a) = (&self._return, &self._name, self._args.to_string());
        write!(f, "{}, {}, ({})", r, n, a)
    }
}

struct Args {
    args: String,
    argc: usize,
}

impl Args {
    fn new(s: &str, argc: usize) -> Result<Self, GmockSedError> {
        Ok(Args { args: rextract(s)?.to_owned(), argc: argc })
    }

    fn strip_self<'a>(&self, s: &'a str) -> &'a str {
        &s[..(s.len() - (self.args.len() + 2))]
    }

    fn empty(&self) -> bool {
        self.args.is_empty()
    }

    fn void(&self) -> bool {
        self.args.trim() == "void"
    }

    fn protected(&self) -> String {
        let mut p = String::new();
        let mut n = 0;
        let mut a = 0;

        for (i, c) in self.args.chars().enumerate() {
            match c {
                ',' if n == 0 => {
                    p.push_str(&protect(&self.args[a..i]));
                    p.push(',');
                    a = i + 1;
                },
                '<' => n += 1,
                '>' => n -= 1,
                _ => {},
            }
        }

        p.push_str(&protect(&self.args[a..]));
        p
    }

    fn contains_unprotected_comma(&self) -> bool {
        let cc = self.comma_count();
        let tc = self.trailing_comma();

        self.argc != cc + (!tc as usize)
    }

    fn comma_count(&self) -> usize {
        self.args.chars().filter(|&c| c == ',').count()
    }

    fn trailing_comma(&self) -> bool {
        self.args.trim_end().ends_with(',')
    }

}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.empty() || self.void() {
            write!(f, "")
        } else if self.contains_unprotected_comma() {
            write!(f, "{}", self.protected())
        } else {
            write!(f, "{}", &self.args)
        }
    }
}

#[derive(Debug)]
struct Qualifiers {
    _const: bool,
    _override: bool,
    pub argc: usize,
    _calltype: Option<String>,
}

impl Qualifiers {
    fn new(_macro: &str, add_override: bool) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(MACRO_REGEX).unwrap();
        }

        let c = RE.captures(_macro).unwrap();
        Qualifiers {
            _const: c.get(1).is_some(),
            _override: add_override,
            argc: c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
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
        match (self._const, self._override, self._calltype.as_ref()) {
            (false, false, None)     => write!(f, ""),
            (true,  false, None)     => write!(f, ", (const)"),
            (false, true,  None)     => write!(f, ", (override)"),
            (false, false, Some(ct)) => write!(f, ", (Calltype({}))", &ct),
            (true,  true,  None)     => write!(f, ", (const, override)"),
            (true,  false, Some(ct)) => write!(f, ", (const, Calltype({}))", &ct),
            (false, true,  Some(ct)) => write!(f, ", (override, Calltype({}))", &ct),
            (true,  true,  Some(ct)) => write!(f, ", (const, override, Calltype({}))", &ct),
        }
    }
}


fn protect(s: &str) -> String {
    if !s.contains(',') { return s.to_owned() }

    let (lead, arg, trail) = trimmings(s);
    format!("{}({}){}", lead, arg, trail)
}

fn trimmings(s: &str) -> (&str, &str, &str) {
    let not_whitespace = |c| { !char::is_whitespace(c) };
    let l = s.find(not_whitespace).unwrap();
    let t = s.rfind(not_whitespace).unwrap() + 1;

    (&s[..l], &s[l..t], &s[t..])
}
