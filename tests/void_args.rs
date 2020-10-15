/// Non-obvious behaviour with void keyword
///
/// valid-old-style: MOCK_METHOD0(Foo, bool(void))
/// naive-new-style: MOCK_METHOD(bool, Foo, (void))  // Error
/// valid-new-style: MOCK_METHOD(bool, Foo, ())

mod void {

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

macro_rules! subtle_test {
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


subtle_test!(
test_old_style_void_args_is_empty_in_new_style
"MOCK_METHOD0(Foo, bool(void))"
->
"MOCK_METHOD(bool, Foo, ())"
);

subtle_test!(
test_old_style_void_args_is_empty_in_new_style_whitespace
"MOCK_METHOD0(Foo, bool(   void    ))"
->
"MOCK_METHOD(bool, Foo, ())"
);

subtle_test!(
ignore // Aware of scenario. Could capture n from MOCK_METHODn
test_old_style_void_args_is_empty_in_new_style_inline_comments
"MOCK_METHOD0(Foo, bool(/* Some */ void /* comments */))"
->
"MOCK_METHOD(bool, Foo, ())"
);

}
