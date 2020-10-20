mod common;
use common::*;

mod multiline {
use super::*;

macro_rules! multiline_test {
    ($name:tt $old:tt -> $new:tt) => {
        #[test]
        fn $name() {
            let path = file($old);

            binary().args(&["replace", "--multi-line", path.to_str().unwrap()])
                    .assert()
                    .success();

            assert_eq!(read(&path), $new);
        }
    };
}

multiline_test!(
test_multiline_args
"MOCK_METHOD3(Foo, bool(
    int,
    double,
    void,
));"
->
"MOCK_METHOD(bool, Foo, (
    int,
    double,
    void,
));"
);

multiline_test!(
test_multiple_multiline_args
"MOCK_METHOD3(Foo, bool(
    int,
    double,
    void,
));
MOCK_METHOD3(Bar, bool(
    int,
    double,
    void,
));"
->
"MOCK_METHOD(bool, Foo, (
    int,
    double,
    void,
));
MOCK_METHOD(bool, Bar, (
    int,
    double,
    void,
));"
);

}
