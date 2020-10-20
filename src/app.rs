use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "gmock-sed",
    about = "Simple CLI tool for updating gMock macros. (MOCK_METHODn -> MOCK_METHOD)",
)]
pub enum Opt {
    /// Locate files that contain at least 1 old-style MOCK_METHDODn macro.
    Search {
        /// List each result with a count of macros detected.
        #[structopt(short, long)]
        count: bool,

        /// Specify the maximum depth for directory traversal.
        #[structopt(long, default_value = "50")]
        max_depth: usize,

        /// Top-level directory in which search for old-style MOCK_METHODn macros.
        #[structopt(name = "DIR", parse(from_os_str))]
        dir: PathBuf,
    },

    /// Substitute old-style macros with equivalent new-style macros.
    Replace {
        /// Don't overwrite files.
        #[structopt(long)]
        dry_run: bool,

        /// Enable multi-line mode.
        #[structopt(long)]
        multi_line: bool,

        /// Paths to files that should be fixed.
        #[structopt(name = "PATHS", parse(from_os_str))]
        files: Vec<PathBuf>,
    },
}
