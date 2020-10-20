/// gmock-sed: Simple CLI tool for updating gMock macros. (MOCK_METHODn -> MOCK_METHOD)

mod app;
mod util;

use std::path::PathBuf;
use rayon::prelude::*;
use walkdir::WalkDir;

use gmock_sed::{ReplaceSummary, ReplaceMode, SearchSummary, SearchMode};

fn main() {
    use structopt::StructOpt;
    use app::Opt::*;
    match app::Opt::from_args() {
        Search { count, max_depth, dir } => {
            let walker = WalkDir::new(dir).max_depth(max_depth);

            let files: Vec<PathBuf> =
                walker.into_iter()
                      .filter_map(Result::ok)
                      .filter(util::is_file)
                      .filter(util::is_cpp)
                      .map(|de| de.into_path())
                      .collect();

            let mode = SearchMode::from(count);

            let results: Vec<SearchSummary> =
                files.par_iter()
                     .map(|pb| util::read(&pb))
                     .map(|cpp| gmock_sed::search(&cpp, mode))
                     .collect();

            fast_stdout!(stdout);

            #[allow(unused)]
            for (f, r) in files.iter().zip(results.iter()) {
                if r.is_match {
                    writeln!(stdout, "{}{}", &f.to_string_lossy(), r);
                }
            }
        },

        Replace { dry_run, multi_line, files } => {
            let mode = ReplaceMode::from(multi_line);

            let results: Vec<ReplaceSummary> =
                files.par_iter()
                     .map(|p| util::read(&p))
                     .filter(|s| !s.is_empty())
                     .map(|cpp| gmock_sed::replace(&cpp, mode))
                     .collect();

            for (file, result) in files.iter().zip(results.iter()) {
                println!("{}: {}", file.display(), result);
                if !dry_run && result.error_free() {
                    util::write(file, result);
                }
            }
        }
    }
}

