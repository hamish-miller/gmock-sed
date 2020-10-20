/// Static regex literals. Compiled to Regex structs elsewhere.

macro_rules! _macro_regex {
    (*) => ( r"(MOCK_METHOD|MOCK_CONST_METHOD)" );
    (!) => ( r"MOCK_(CONST_)?METHOD(\d|10)(_T)?(_WITH_CALLTYPE)?" );
    (?) => ( r"MOCK_(?:CONST_)?METHOD(?:\d|10)(?:_T)?(?:_WITH_CALLTYPE)?" );
}

macro_rules! _parentheses {
    (s) => ( r"\((.*)\)" );
    (m) => ( r"\(((?s).*?)\)" );
}

macro_rules! _mock_method_regex {
    (s) => ( concat!(_mock_method_regex!(), _parentheses!(s)) );
    (m) => ( concat!(_mock_method_regex!(), _parentheses!(m), r"\s*", r";") );
    () => ( concat!(r"(", _macro_regex!(?), r")", r"\s*") );
}

macro_rules! _signature_regex {
    (s) => ( concat!(_signature_regex!(), _parentheses!(s)) );
    (m) => ( concat!(_signature_regex!(), _parentheses!(m)) );
    () => ( r"\s*([^,]+)\s*,\s*([^\(]+)\s*" );
}

pub const SEARCH_REGEX: &str = _macro_regex!(*);

pub const REPLACE_REGEX_S: &str = _mock_method_regex!(s);
pub const REPLACE_REGEX_M: &str = _mock_method_regex!(m);

pub const MACRO_REGEX: &str = _macro_regex!(!);

pub const SIG_REGEX: &str = _signature_regex!(m);

pub const ARG_REGEX: &str = r"(?s).*";

pub const CALLTYPE_REGEX: &str = r"[^,]+";

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use lazy_static::lazy_static;

    mod replace_regex {
        use super::*;

        fn regex() -> Regex {
            lazy_static! {
                static ref RE: Regex = Regex::new(REPLACE_REGEX_S).unwrap();
            }

            RE.clone()
        }

        #[test]
        fn test_match() {
            let cpp = "MOCK_METHOD0(Foo, bool())";

            assert!(regex().is_match(cpp));
        }

        #[test]
        fn test_groups() {
            let cpp = "MOCK_METHOD0(Foo, bool())";
            let c = regex().captures(cpp).unwrap();

            assert_eq!(c.get(1).map(|m| m.as_str()), Some("MOCK_METHOD0"));
            assert_eq!(c.get(2).map(|m| m.as_str()), Some("Foo, bool()"));
        }
    }

    mod macro_regex {
        use super::*;

        fn regex() -> Regex {
            lazy_static! {
                static ref RE: Regex = Regex::new(MACRO_REGEX).unwrap();
            }

            RE.clone()
        }

        #[test]
        fn test_match() {
            let cpp = "MOCK_CONST_METHOD0";

            assert!(regex().is_match(cpp));
        }

        #[test]
        fn test_groups() {
            let cpp = "MOCK_CONST_METHOD0_T_WITH_CALLTYPE";
            let c = regex().captures(cpp).unwrap();

            assert_eq!(c.get(1).map(|m| m.as_str()), Some("CONST_"));
            assert_eq!(c.get(2).map(|m| m.as_str()), Some("0"));
            assert_eq!(c.get(3).map(|m| m.as_str()), Some("_T"));
            assert_eq!(c.get(4).map(|m| m.as_str()), Some("_WITH_CALLTYPE"));
        }
    }

    mod signature_regex {
        use super::*;

        fn regex() -> Regex {
            lazy_static! {
                static ref RE: Regex = Regex::new(SIG_REGEX).unwrap();
            }

            RE.clone()
        }

        #[test]
        fn test_match() {
            let cpp = "Foo, int(bool, double)";

            assert!(regex().is_match(cpp));
        }

        #[test]
        fn test_groups() {
            let cpp = "Foo, int(bool, double)";
            let c = regex().captures(cpp).unwrap();

            assert_eq!(c.get(1).map(|m| m.as_str()), Some("Foo"));
            assert_eq!(c.get(2).map(|m| m.as_str()), Some("int"));
            assert_eq!(c.get(3).map(|m| m.as_str()), Some("bool, double"));
        }

        #[ignore]
        #[test]
        fn test_multiline() {
            let cpp = "Foo, int(bool,\ndouble)";
            let c = regex().captures(cpp).unwrap();

            assert_eq!(c.get(3).map(|m| m.as_str()), Some("bool,\ndouble"));
        }
    }

    mod arg_regex {
        use super::*;

        fn regex() -> Regex {
            lazy_static! {
                static ref RE: Regex = Regex::new(ARG_REGEX).unwrap();
            }

            RE.clone()
        }

        #[test]
        fn test_count() {
            let cpp = "bool, Foo, int";
            let m = regex().find(cpp).unwrap();

            assert_eq!(m.as_str(), "bool, Foo, int");
        }

        #[test]
        fn test_multiline() {
            let cpp = "bool,\nFoo,\nint";
            let m = regex().find(cpp).unwrap();

            assert_eq!(m.as_str(), "bool,\nFoo,\nint");
        }
    }

    mod calltype_regex {
        use super::*;

        fn regex() -> Regex {
            lazy_static! {
                static ref RE: Regex = Regex::new(CALLTYPE_REGEX).unwrap();
            }

            RE.clone()
        }

        #[test]
        fn test_find() {
            let cpp = "STDMETHODCALLTYPE, Foo, int(bool)";
            let m = regex().find(cpp).unwrap();

            assert_eq!(m.as_str(), "STDMETHODCALLTYPE");
        }
    }
}

