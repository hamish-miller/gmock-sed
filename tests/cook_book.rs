/// Examples specified in the GMock cookbook.
///
/// https://github.com/google/googletest/blob/master/googlemock/docs/cook_book.md#old-style-mock_methodn-macros

mod cookbook {

use std::io::prelude::*;
use assert_cmd::Command;
use tempfile::{NamedTempFile, TempPath};

fn binary() -> Command {
    Command::cargo_bin("gmock-sed").unwrap()
}

fn file(contents: &str) -> TempPath {
    let mut file = NamedTempFile::new().unwrap();
    file.write(contents.as_bytes()).unwrap();

    file.into_temp_path()
}

fn read(path: &TempPath) -> String {
    std::fs::read_to_string(path).unwrap()

}

macro_rules! cookbook_test {
    ($name:tt $old:tt -> $new:tt) => {
        #[test]
        fn $name() {
            let path = file($old);

            binary().args(&["replace", path.to_str().unwrap()])
                    .assert()
                    .success();

            assert_eq!(read(&path), $new);
        }
    }
}

cookbook_test!(
test_simple
"MOCK_METHOD1(Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int))"
);

cookbook_test!(
test_const_method
"MOCK_CONST_METHOD1(Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (const))"
);

cookbook_test!(
test_method_in_a_class_template
"MOCK_METHOD1_T(Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int))"
);

cookbook_test!(
test_const_method_in_a_class_template
"MOCK_CONST_METHOD1_T(Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (const))"
);

cookbook_test!(
test_method_with_call_type
"MOCK_METHOD1_WITH_CALLTYPE(STDMETHODCALLTYPE, Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (Calltype(STDMETHODCALLTYPE)))"
);

cookbook_test!(
test_const_method_with_call_type
"MOCK_CONST_METHOD1_WITH_CALLTYPE(STDMETHODCALLTYPE, Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (const, Calltype(STDMETHODCALLTYPE)))"
);

cookbook_test!(
test_method_with_call_type_in_a_class_template
"MOCK_METHOD1_T_WITH_CALLTYPE(STDMETHODCALLTYPE, Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (Calltype(STDMETHODCALLTYPE)))"
);

cookbook_test!(
test_const_method_with_call_type_in_a_class_template
"MOCK_CONST_METHOD1_T_WITH_CALLTYPE(STDMETHODCALLTYPE, Foo, bool(int))"
->
"MOCK_METHOD(bool, Foo, (int), (const, Calltype(STDMETHODCALLTYPE)))"
);

}

