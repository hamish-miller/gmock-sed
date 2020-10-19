pub const MOCK_METHOD_SEARCH_REGEX: &'static str = r"(MOCK_METHOD|MOCK_CONST_METHOD)";

macro_rules! _macro_regex {
    (!) => ( r"MOCK_(CONST_)?METHOD(\d|10)(_T)?(_WITH_CALLTYPE)?" );
    (?) => ( r"MOCK_(?:CONST_)?METHOD(?:\d|10)(?:_T)?(?:_WITH_CALLTYPE)?" );
}

pub const MOCK_METHOD_REGEX: &str = concat!(
    r"(", _macro_regex!(?), r")",
    r"\s*", r"\((.*)\)"
);

pub const MACRO_REGEX: &str = _macro_regex!(!);

pub const PARAM_REGEX: &'static str = r"(?x)
    \s*
    ([^,]+)
    ,\s*
    ([^\(]+)
    \s*\(
    (.*)
    \)
";

pub const CALLTYPE_REGEX: &'static str = r"[^,]+";

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use lazy_static::lazy_static;

    mod mock_method {
        use super::*;

        fn regex() -> Regex {
            lazy_static! {
                static ref RE: Regex = Regex::new(MOCK_METHOD_REGEX).unwrap();
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

    mod param_regex {
        use super::*;

        fn regex() -> Regex {
            lazy_static! {
                static ref RE: Regex = Regex::new(PARAM_REGEX).unwrap();
            }

            RE.clone()
        }

        #[test]
        fn test_match() {
            let cpp = "Foo, int(bool)";

            assert!(regex().is_match(cpp));
        }

        #[test]
        fn test_groups() {
            let cpp = "Foo, int(bool)";
            let c = regex().captures(cpp).unwrap();

            assert_eq!(c.get(1).map(|m| m.as_str()), Some("Foo"));
            assert_eq!(c.get(2).map(|m| m.as_str()), Some("int"));
            assert_eq!(c.get(3).map(|m| m.as_str()), Some("bool"));
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

