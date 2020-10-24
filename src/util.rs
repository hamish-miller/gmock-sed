use std::path::Path;
use walkdir::DirEntry;
use gmock_sed::ReplaceSummary;

pub fn read(path: &Path) -> String {
    // Assume small file
    std::fs::read_to_string(path).unwrap_or_else(|_e| {
        #[cfg(debug_assertions)]
        eprintln!("{}: {}", _e, path.display());

        // Pretend the file is empty
        String::new()
    })
}

pub fn write(path: &Path, contents: &ReplaceSummary) {
    if let Some(code) = contents.suggestion.as_ref() {
        std::fs::write(path, code.as_bytes()).unwrap()
    }
}

pub fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

const CPP_SOURCE_EXT: [&'static str; 5] = ["cpp", ".cc", ".C", ".cxx", ".c++"];
const CPP_HEADER_EXT: [&'static str; 6] = ["h", ".hh", ".H", ".hxx", ".hpp", ".h++"];

pub fn is_cpp(entry: &DirEntry) -> bool {
    if let Some(ext) = entry.path().extension().and_then(|os_str| os_str.to_str()) {
        return CPP_SOURCE_EXT.contains(&ext) || CPP_HEADER_EXT.contains(&ext)
    }

    // Better safe than sorry
    false
}

#[macro_export]
macro_rules! fast_stdout {
    ( $name:ident ) => {
        use std::io::Write;
        let out = std::io::stdout();
        let mut $name = std::io::BufWriter::new(out.lock());
    };
}
