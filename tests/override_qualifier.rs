mod common;
use common::*;

mod override_qualifier {
use super::*;

macro_rules! override_test {
    ($name:tt $old:tt -> $new:tt) => {
        #[test]
        fn $name() {
            let path = file($old);

            binary().args(&["replace", "--add-override", path.to_str().unwrap()])
                    .assert()
                    .success();

            assert_eq!(read(&path), $new);
        }
    }
}

override_test!(
test_basic
"MOCK_METHOD1(Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (override))"
);

override_test!(
test_const
"MOCK_CONST_METHOD1(Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (const, override))"
);

override_test!(
test_calltype
"MOCK_METHOD1_WITH_CALLTYPE(STDMETHODCALLTYPE, Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (override, Calltype(STDMETHODCALLTYPE)))"
);

override_test!(
test_const_and_calltype
"MOCK_CONST_METHOD1_WITH_CALLTYPE(STDMETHODCALLTYPE, Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (const, override, Calltype(STDMETHODCALLTYPE)))"
);

}
