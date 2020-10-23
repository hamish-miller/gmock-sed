mod common;
use common::*;

mod whitespace {
use super::*;

macro_rules! whitespace_test {
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

whitespace_test!(
test_outside
"
MOCK_METHOD3 (Foo, bool(One, Two, Three));
MOCK_METHOD3(Foo, bool (One, Two, Three)) ;
"
->
"
MOCK_METHOD(bool, Foo, (One, Two, Three));
MOCK_METHOD(bool, Foo, (One, Two, Three));
"
);

whitespace_test!(
test_signature
"
MOCK_METHOD3( Foo, bool(One, Two, Three));
MOCK_METHOD3(Foo , bool(One, Two, Three));
MOCK_METHOD3(Foo,bool(One, Two, Three));
MOCK_METHOD3(Foo, bool (One, Two, Three));
MOCK_METHOD3(Foo, bool(One, Two, Three) );
"
->
"
MOCK_METHOD(bool, Foo, (One, Two, Three));
MOCK_METHOD(bool, Foo, (One, Two, Three));
MOCK_METHOD(bool, Foo, (One, Two, Three));
MOCK_METHOD(bool, Foo, (One, Two, Three));
MOCK_METHOD(bool, Foo, (One, Two, Three));
"
);

// Allow arg whitespace to support multiline
whitespace_test!(
test_args
"
MOCK_METHOD3(Foo, bool( One, Two, Three));
MOCK_METHOD3(Foo, bool(One,Two, Three));
MOCK_METHOD3(Foo, bool(One, Two , Three));
MOCK_METHOD3(Foo, bool(One, Two,Three));
MOCK_METHOD3(Foo, bool(One, Two, Three ));
"
->
"
MOCK_METHOD(bool, Foo, ( One, Two, Three));
MOCK_METHOD(bool, Foo, (One,Two, Three));
MOCK_METHOD(bool, Foo, (One, Two , Three));
MOCK_METHOD(bool, Foo, (One, Two,Three));
MOCK_METHOD(bool, Foo, (One, Two, Three ));
"
);

}

