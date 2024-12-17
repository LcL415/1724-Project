use ignore::WalkBuilder;
use regex::Regex;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use dashmap::DashMap;
use crossbeam_channel;

use crate::filter::*;

pub fn search_files(
    directory: &str,
    pattern: &str,
    name: Option<&str>,
    exclude: Option<&str>,
    ignore_hidden: bool,
    threads: usize,
    min_size: Option<u64>,
    max_size: Option<u64>,
    max_depth: Option<usize>,
    min_date: Option<SystemTime>,
    max_date: Option<SystemTime>,
    file_extension: Option<&str>,
    gitignore_path: Option<&str>,
    fuzzy_patterns: Option<&[&str]>,
    max_fuzzy_distance: usize,
    skip_size_filter: bool,
    skip_date_filter: bool,
    skip_file_type_filter: bool,
    skip_gitignore_filter: bool,
    skip_fuzzy_filter: bool,
) -> Vec<PathBuf> {
    // Compile regex pattern once
    let regex = Arc::new(Regex::new(pattern).expect("Invalid regex pattern"));

    // Parse excluded directories and cache canonical paths using DashMap
    let excluded_dirs: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
    if let Some(exclude) = exclude {
        for ex in exclude.split(',').filter(|ex| !ex.is_empty()) {
            let canonical = std::fs::canonicalize(ex).unwrap_or_else(|_| ex.into());
            excluded_dirs.insert(ex.into(), canonical.to_string_lossy().into());
        }
    }

    // Set up WalkBuilder with ignore library
    let walker = WalkBuilder::new(directory)
        .max_depth(max_depth)
        .follow_links(true)
        .hidden(ignore_hidden)
        .threads(threads)
        .build_parallel();

    // Use crossbeam channel for results collection
    let (tx, rx) = crossbeam_channel::unbounded();

    walker.run(|| {
        let regex = Arc::clone(&regex);
        let excluded_dirs = Arc::clone(&excluded_dirs);
        let tx = tx.clone();

        Box::new(move |entry_res| {
            if let Ok(entry) = entry_res {
                let path = entry.path();

                // Filter excluded directories
                if let Some(parent) = path.parent() {
                    if let Ok(parent_abs) = std::fs::canonicalize(parent) {
                        if excluded_dirs.iter().any(|ex| parent_abs.starts_with(ex.value())) {
                            return ignore::WalkState::Continue;
                        }
                    }
                }

                // Filter files only (skip directories)
                if !entry.file_type().map_or(false, |ft| ft.is_file()) {
                    return ignore::WalkState::Continue;
                }

                // Filter by file name if name is provided
                if let Some(name_pattern) = name {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if !file_name.contains(name_pattern) {
                            return ignore::WalkState::Continue;
                        }
                    }
                }

                // Filter by file extension
                if !skip_file_type_filter && !filter_by_file_type(path, file_extension) {
                    return ignore::WalkState::Continue;
                }

                // Filter by file size
                if !skip_size_filter && !filter_by_size(path, min_size, max_size) {
                    return ignore::WalkState::Continue;
                }

                // Filter by modification date
                if !skip_date_filter && !filter_by_date(path, min_date, max_date) {
                    return ignore::WalkState::Continue;
                }

                // Filter by .gitignore
                if !skip_gitignore_filter && !filter_by_gitignore(path, gitignore_path) {
                    return ignore::WalkState::Continue;
                }

                // Filter by fuzzy match
                if !skip_fuzzy_filter
                    && !filter_by_fuzzy_match_with_distance(path, fuzzy_patterns, max_fuzzy_distance)
                {
                    return ignore::WalkState::Continue;
                }

                // Filter by regex pattern on file name
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if !regex.is_match(file_name) {
                        return ignore::WalkState::Continue;
                    }
                }

                // Send matching path to channel
                tx.send(path.to_path_buf()).unwrap();
            }

            ignore::WalkState::Continue
        })
    });

    drop(tx); // Close the sending side of the channel

    // Collect results from the channel
    rx.into_iter().collect()
}
