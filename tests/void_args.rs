/// Non-obvious behaviour with void keyword
///
/// valid-old-style: MOCK_METHOD0(Foo, bool(void))
/// naive-new-style: MOCK_METHOD(bool, Foo, (void))  // Error
/// valid-new-style: MOCK_METHOD(bool, Foo, ())

mod common;
use common::*;

mod void {
use super::*;

macro_rules! void_test {
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
    (ignore $name:tt $old:tt -> $new:tt) => {
        #[test]
        #[ignore]
        fn $name() {
            let path = file($old);

            binary().args(&["replace", path.to_str().unwrap()])
                    .assert()
                    .success();

            assert_eq!(read(&path), $new);
        }
    };
}


void_test!(
test_old_style_void_args_is_empty_in_new_style
"MOCK_METHOD0(Foo, bool(void))"
->
"MOCK_METHOD(bool, Foo, ())"
);

void_test!(
test_old_style_void_args_is_empty_in_new_style_whitespace
"MOCK_METHOD0(Foo, bool(   void    ))"
->
"MOCK_METHOD(bool, Foo, ())"
);

void_test!(
ignore // Aware of scenario. Could capture n from MOCK_METHODn
test_old_style_void_args_is_empty_in_new_style_inline_comments
"MOCK_METHOD0(Foo, bool(/* Some */ void /* comments */))"
->
"MOCK_METHOD(bool, Foo, ())"
);

}
