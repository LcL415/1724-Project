mod filter;
mod output;
mod search;

use clap::{ArgAction, Parser};
use chrono::DateTime;
use std::time::SystemTime;

#[derive(Parser)]
#[command(
    name = "fd",
    about = "A command-line tool to search files with various filters.",
    author = "LYD, LCL, ZJC",
    version = "1.0"
)]
struct Cli {
    #[arg(long, default_value = ".", help = "The directory to search.")]
    directory: String,

    #[arg(help = "The file name to search.")]
    name: Option<String>,

    #[arg(long, help = "The pattern to search for in file names (interpreted as regex).")]
    pattern: Option<String>,

    #[arg(short = 'x', long, help = "Comma-separated list of directories to exclude.")]
    exclude: Option<String>,

    #[arg(long, action = ArgAction::SetTrue, help = "Output absolute file paths.")]
    absolute: bool,

    #[arg(
        long,
        default_value_t = true,
        action = ArgAction::SetTrue,
        help = "Ignore hidden files and directories. Use --no-ignore-hidden to include hidden files."
    )]
    #[arg(
        long = "no-ignore-hidden",
        action = ArgAction::SetFalse,
        help = "Include hidden files and directories (negates --ignore-hidden)."
    )]
    ignore_hidden: bool,

    #[arg(long, default_value_t = 16, help = "Number of threads to use.")]
    threads: usize,

    #[arg(long, help = "Minimum file size in bytes.")]
    min_size: Option<u64>,

    #[arg(long, help = "Maximum file size in bytes.")]
    max_size: Option<u64>,

    #[arg(long, help = "Maximum directory traversal depth.")]
    max_depth: Option<usize>,

    #[arg(long, help = "Minimum file modification date (RFC3339 format).")]
    min_date: Option<String>,

    #[arg(long, help = "Maximum file modification date (RFC3339 format).")]
    max_date: Option<String>,

    #[arg(long, help = "Filter by file extension.")]
    file_extension: Option<String>,

    #[arg(long, help = "Path to a .gitignore file.")]
    gitignore_path: Option<String>,

    #[arg(long, help = "Comma-separated list of fuzzy patterns to match.")]
    fuzzy_patterns: Option<String>,

    #[arg(long, default_value_t = 2, help = "Maximum fuzzy match distance.")]
    max_fuzzy_distance: usize,

    #[arg(long, action = ArgAction::SetTrue, help = "Skip size filter.")]
    skip_size_filter: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Skip date filter.")]
    skip_date_filter: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Skip file type filter.")]
    skip_file_type_filter: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Skip gitignore filter.")]
    skip_gitignore_filter: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Skip fuzzy filter.")]
    skip_fuzzy_filter: bool,
}

fn parse_system_time(input: Option<&String>) -> Option<SystemTime> {
    input.and_then(|s| {
        DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| SystemTime::from(dt))
    })
}

fn main() {
    let cli = Cli::parse();

    let fuzzy_patterns = cli
        .fuzzy_patterns
        .as_deref()
        .map(|p| p.split(',').collect::<Vec<&str>>());

    let min_date = parse_system_time(cli.min_date.as_ref());
    let max_date = parse_system_time(cli.max_date.as_ref());

    let results = search::search_files(
        &cli.directory,
        cli.pattern.as_deref().unwrap_or(".*"), // Use regex pattern if provided, or match everything
        cli.name.as_deref(),
        cli.exclude.as_deref(),
        cli.ignore_hidden,
        cli.threads,
        cli.min_size,
        cli.max_size,
        cli.max_depth,
        min_date,
        max_date,
        cli.file_extension.as_deref(),
        cli.gitignore_path.as_deref(),
        fuzzy_patterns.as_deref(),
        cli.max_fuzzy_distance,
        cli.skip_size_filter || cli.min_size.is_none() && cli.max_size.is_none(),
        cli.skip_date_filter || min_date.is_none() && max_date.is_none(),
        cli.skip_file_type_filter || cli.file_extension.is_none(),
        cli.skip_gitignore_filter || cli.gitignore_path.is_none(),
        cli.skip_fuzzy_filter || cli.fuzzy_patterns.is_none(),
    );
    output::print_results(&results, cli.absolute, cli.name.as_deref());
}
