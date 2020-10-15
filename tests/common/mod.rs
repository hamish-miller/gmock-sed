/// Helper functions for testing.

use std::io::prelude::*;
use assert_cmd::Command;
use tempfile::{NamedTempFile, TempPath};

pub fn binary() -> Command {
    Command::cargo_bin("gmock-sed").unwrap()
}

pub fn file(contents: &str) -> TempPath {
    let mut file = NamedTempFile::new().unwrap();
    file.write(contents.as_bytes()).unwrap();

    file.into_temp_path()
}

pub fn read(path: &TempPath) -> String {
    std::fs::read_to_string(path).unwrap()
}
