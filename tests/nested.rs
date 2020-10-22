mod common;
use common::*;

mod nested {
use super::*;

macro_rules! nested_test {
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

nested_test!(
test_nested_arg
"MOCK_METHOD1(Foo, bool(SOME_MACRO(int)));"
->
"MOCK_METHOD(bool, Foo, (SOME_MACRO(int)));"
);

}
