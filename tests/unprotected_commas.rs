/// MOCK_METHOD needs brackets to replace prior knowledge from n-suffix
///
/// https://github.com/google/googletest/blob/master/googlemock/docs/cook_book.md#dealing-with-unprotected-commas

mod common;
use common::*;

mod unprotected_commas {
use super::*;

macro_rules! unprotected_commas_test {
    ($name:tt $old:tt -> $new:tt) => {
        #[test]
        fn $name() {
            let path = file($old);

            binary().args(&["replace", path.to_str().unwrap()])
                    .assert()
                    .success();

            assert_eq!(read(&path), $new);
        }
    };
}

unprotected_commas_test!(
test_return_type
"MOCK_METHOD0(GetPair, std::pair<bool, int>());"
->
"MOCK_METHOD((std::pair<bool, int>), GetPair, ());"
);

unprotected_commas_test!(
test_arg_type
"MOCK_METHOD2(CheckMap, bool(std::map<int, double>, bool));"
->
"MOCK_METHOD(bool, CheckMap, ((std::map<int, double>), bool));"
);

unprotected_commas_test!(
test_arg_type_trailing_comma
"
MOCK_METHOD2(CheckMap, bool(
    std::map<int, double>,
    bool,
));
"
->
"
MOCK_METHOD(bool, CheckMap, (
    (std::map<int, double>),
    bool,
));
"
);

}
