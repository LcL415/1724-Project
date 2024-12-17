use ignore::{gitignore::GitignoreBuilder, Match};
use regex::Regex;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
// use strsim::levenshtein;

/// filter based on file size
#[allow(dead_code)]
pub fn filter_by_size(path: &Path, min_size: Option<u64>, max_size: Option<u64>) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let size = metadata.len();
        if let Some(min) = min_size {
            if size < min {
                return false;
            }
        }
        if let Some(max) = max_size {
            if size > max {
                return false;
            }
        }
    }
    true
}

/// filter based on date
#[allow(dead_code)]
pub fn filter_by_date(
    path: &Path,
    min_date: Option<SystemTime>,
    max_date: Option<SystemTime>,
) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified_time) = metadata.modified() {
            if let Some(min) = min_date {
                if modified_time < min {
                    return false;
                }
            }
            if let Some(max) = max_date {
                if modified_time > max {
                    return false;
                }
            }
        }
    }
    true
}

/// filter based in file type
#[allow(dead_code)]
pub fn filter_by_file_type(path: &Path, file_extension: Option<&str>) -> bool {
    if let Some(extension) = file_extension {
        if let Some(ext) = Path::new(path).extension() {
            if ext != extension {
                return false;
            }
        } else {
            return false; // file has no extension name
        }
    }
    true
}

/// filter based on regex file name
#[allow(dead_code)]
pub fn filter_by_name_regex(path: &Path, regex_pattern: Option<&str>) -> bool {
    if let Some(pattern) = regex_pattern {
        if let Some(file_name) = Path::new(path).file_name() {
            if let Some(name_str) = file_name.to_str() {
                if let Ok(regex) = Regex::new(pattern) {
                    if !regex.is_match(name_str) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

/// filte file based on .gitignore file
#[allow(dead_code)]
pub fn filter_by_gitignore(path: &Path, gitignore_path: Option<&str>) -> bool {
    let file_path = Path::new(path);

    // initialize GitignoreBuilder
    let mut builder = GitignoreBuilder::new(".");
    if let Some(ignore_file) = gitignore_path {
        if let Some(err) = builder.add(ignore_file) {
            eprintln!("Failed to load .gitignore file: {}", err);
            return true; // if load failed,
        }
    }

    // construct Gitignore instance
    match builder.build() {
        Ok(ignore) => {
            match ignore.matched(file_path, false) {
                Match::Ignore(_) => false, // ignored by .gitignore
                _ => true,                 // don't ignore
            }
        }
        Err(e) => {
            eprintln!("Failed to build Gitignore matcher: {}", e);
            true // don't ignore
        }
    }
}

pub fn fuzzy_match_with_distance(
    haystack: &str,
    needle: &str,
    max_distance: usize,
) -> bool {
    let haystack_len = haystack.len();
    let needle_len = needle.len();

    if needle_len > haystack_len {
        return false;
    }

    let mut needle_index = 0;
    let mut distance = 0;
    let mut last_match_pos = None;

    for (haystack_index, haystack_char) in haystack.chars().enumerate() {
        if needle_index < needle_len && haystack_char == needle.chars().nth(needle_index).unwrap() {
            if let Some(last_pos) = last_match_pos {
                if haystack_index > last_pos + max_distance {
                    distance += haystack_index - last_pos - max_distance;
                }
            }
            last_match_pos = Some(haystack_index);
            needle_index += 1;
        } else {
            distance += 1;
        }

        if distance > max_distance {
            return false;
        }
    }

    distance += needle_len - needle_index;

    distance <= max_distance
}

pub fn filter_by_fuzzy_match_with_distance(
    path: &Path,
    patterns: Option<&[&str]>,
    max_distance: usize,
) -> bool {
    if let Some(patterns) = patterns {
        if let Some(file_name) = std::path::Path::new(path).file_name().and_then(|f| f.to_str()) {
            let (name, extension) = split_file_name(file_name);

            return patterns.iter().any(|pattern| {
                let (pattern_name, pattern_extension) = split_file_name(pattern);

                let name_matches =
                    fuzzy_match_with_distance(name, pattern_name, max_distance);

                let extension_matches = if let Some(ext) = pattern_extension {
                    extension.map_or(false, |file_ext| file_ext == ext)
                } else {
                    true
                };

                name_matches && extension_matches
            });
        }
        false
    } else {
        true // 如果没有提供模式，默认匹配所有文件
    }
}

pub fn split_file_name(file_name: &str) -> (&str, Option<&str>) {
    if let Some(pos) = file_name.rfind('.') {
        (&file_name[..pos], Some(&file_name[pos + 1..]))
    } else {
        (file_name, None)
    }
}


#[allow(dead_code)]
pub fn filter_hidden(path: &Path, ignore_hidden: bool) -> bool {
    if !ignore_hidden {
        return true; // Do not filter anything if hidden files are allowed.
    }

    let is_hidden = |path: &Path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with('.'))
            .unwrap_or(false)
    };

    // Check if the file or any of its parent directories are hidden.
    let mut current_path = Some(path);
    while let Some(p) = current_path {
        if is_hidden(p) {
            return false; // Exclude hidden files or files in hidden directories.
        }
        current_path = p.parent();
    }

    true // Include the file if it or its parents are not hidden.
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use std::time::{Duration, SystemTime};

    const TEST_DIR: &str = "test_env";

    fn clean_test_env() {
        if Path::new(TEST_DIR).exists() {
            fs::remove_dir_all(TEST_DIR).unwrap();
        }
    }

    fn setup_test_env() {
        clean_test_env();
        fs::create_dir_all(format!("{}/example_dir", TEST_DIR)).unwrap();
        fs::create_dir_all(format!("{}/ignore_dir", TEST_DIR)).unwrap();

        let mut file = File::create(format!("{}/test_file.txt", TEST_DIR)).unwrap();
        writeln!(file, "This is a test file.").unwrap();

        let mut random_file =
            File::create(format!("{}/example_dir/random_file.txt", TEST_DIR)).unwrap();
        writeln!(random_file, "Random content for random file.").unwrap();

        let mut ignored_file =
            File::create(format!("{}/ignore_dir/ignored_file.txt", TEST_DIR)).unwrap();
        writeln!(ignored_file, "Ignored file content.").unwrap();

        // create .gitignore 文件
        let mut gitignore_file = File::create(format!("{}/.gitignore", TEST_DIR)).unwrap();
        writeln!(gitignore_file, "*.txt").unwrap();
    }

    #[test]
    #[serial]
    fn test_filter_by_size() {
        setup_test_env();
        let path_str = format!("{}/test_file.txt", TEST_DIR);
        let path: &Path = Path::new(&path_str);

        assert!(filter_by_size(&path, Some(10), Some(50))); // size = 20
        assert!(!filter_by_size(&path, Some(30), None)); // size < 30
        assert!(filter_by_size(&path, None, Some(50))); // size <= 50

        clean_test_env();
    }

    #[test]
    #[serial]
    fn test_filter_hidden() {
        // Test cases for visible files and directories.
        assert!(filter_hidden(Path::new("/path/to/file.txt"), true));
        assert!(filter_hidden(Path::new("/path/to/dir/file.txt"), true));

        // Test cases for hidden files.
        assert!(!filter_hidden(Path::new("/path/to/.hidden_file.txt"), true));

        // Test cases for files inside hidden directories.
        assert!(!filter_hidden(
            Path::new("/path/to/.hidden_dir/file.txt"),
            true
        ));

        // Test cases with ignore_hidden = false (include all files).
        assert!(filter_hidden(Path::new("/path/to/.hidden_file.txt"), false));
        assert!(filter_hidden(
            Path::new("/path/to/.hidden_dir/file.txt"),
            false
        ));
    }

    #[test]
    #[serial]
    fn test_filter_by_date() {
        setup_test_env();
        let path_str = format!("{}/test_file.txt", TEST_DIR);
        let path: &Path = Path::new(&path_str);

        let now = SystemTime::now();
        let past = now - Duration::from_secs(3600); // 1 hour ago
        let future = now + Duration::from_secs(3600); // 1 hour later

        assert!(filter_by_date(&path, Some(past), Some(future))); // modified_time within range
        assert!(!filter_by_date(&path, Some(future), None)); // modified_time before future
        assert!(filter_by_date(&path, None, None)); // No restriction

        clean_test_env();
    }

    #[test]
    #[serial]
    fn test_filter_by_file_type() {
        setup_test_env();
        let path_str = format!("{}/test_file.txt", TEST_DIR);
        let path: &Path = Path::new(&path_str);

        assert!(filter_by_file_type(&path, Some("txt"))); // matches extension
        assert!(!filter_by_file_type(&path, Some("doc"))); // doesn't match extension

        clean_test_env();
    }

    #[test]
    #[serial]
    fn test_filter_by_name_regex() {
        setup_test_env();
        let path_str = format!("{}/test_file.txt", TEST_DIR);
        let path: &Path = Path::new(&path_str);
        
        assert!(filter_by_name_regex(&path, Some(r"^test.*\.txt$"))); // regex matches
        assert!(!filter_by_name_regex(&path, Some(r"^example.*$"))); // regex doesn't match

        clean_test_env();
    }

    #[test]
    #[serial]
    fn test_filter_by_gitignore() {
        setup_test_env();
        let path_str = format!("{}/test_file.txt", TEST_DIR);
        let path: &Path = Path::new(&path_str);
        let ignore_file = format!("{}/.gitignore", TEST_DIR);

        assert!(!filter_by_gitignore(&path, Some(&ignore_file))); // matches .gitignore rule
        assert!(filter_by_gitignore(&path, None)); // without gitignore

        clean_test_env();
    }

    #[test]
    #[serial]
    fn test_filter_by_fuzzy_match_with_distance() {
        setup_test_env();
        let path_str = format!("{}/example_dir/random_file.txt", TEST_DIR);
        let path: &Path = Path::new(&path_str);

        assert!(filter_by_fuzzy_match_with_distance(
            &path,
            Some(&["rdm_file.txt"]),
            3
        ));
        assert!(filter_by_fuzzy_match_with_distance(
            &path,
            Some(&["ranom_file.txt"]),
            2
        ));
        assert!(!filter_by_fuzzy_match_with_distance(
            &path,
            Some(&["random_file.tx"]),
            2
        ));
        assert!(filter_by_fuzzy_match_with_distance(
            &path,
            Some(&["rando_file.txt"]),
            2
        ));
        assert!(filter_by_fuzzy_match_with_distance(
            &path,
            Some(&["random_file"]),
            2
        ));
        assert!(!filter_by_fuzzy_match_with_distance(
            &path,
            Some(&["not_matching"]),
            3
        ));

        clean_test_env();
    }
}
