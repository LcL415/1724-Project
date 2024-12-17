use ansi_term::Colour;
use lscolors::{Color, LsColors, Style};
use std::fs;
use std::path::PathBuf;
use chrono::{Local, DateTime};
use std::time::SystemTime;

pub fn print_results(results: &[PathBuf], absolute: bool, pattern: Option<&str>) {
    let lscolors = LsColors::default();

    for result in results {
        // Resolve path
        let display_path = if absolute {
            fs::canonicalize(result).unwrap_or_else(|_| result.to_owned())
        } else {
            result.to_owned()
        };

        // Style and print path
        let styled_path = if let Some(style) = lscolors.style_for_path(&display_path) {
            apply_lscolors(&display_path, style, pattern)
        } else {
            highlight_pattern(&display_path, pattern)
        };

        // Collect and print additional information
        if let Ok(metadata) = fs::metadata(&display_path) {
            let size = metadata.len();
            let modified_time = metadata.modified()
                .map(to_local_time)
                .unwrap_or_else(|_| "Unknown".to_string());

            println!(
                "{} | Size: {} bytes | Modified: {}",
                styled_path, size, modified_time
            );
        } else {
            println!("{}", styled_path);
        }
    }
}

// Convert SystemTime to local time string
fn to_local_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

// Apply lscolors and highlight pattern
fn apply_lscolors(path: &PathBuf, style: &Style, pattern: Option<&str>) -> String {
    let path_str = path.to_string_lossy().to_string();
    let mut styled_path = path_str.clone();

    if let Some(fg) = &style.foreground {
        styled_path = match fg {
            Color::Fixed(color) => Colour::Fixed(*color).paint(&path_str).to_string(),
            Color::RGB(r, g, b) => Colour::RGB(*r, *g, *b).paint(&path_str).to_string(),
            _ => path_str,
        };
    }
    highlight_pattern_with_raw(&styled_path, pattern)
}

// Highlight a simple string pattern with red color
fn highlight_pattern(path: &PathBuf, pattern: Option<&str>) -> String {
    let path_str = path.to_string_lossy().to_string();
    highlight_pattern_with_raw(&path_str, pattern)
}

fn highlight_pattern_with_raw(path: &str, pattern: Option<&str>) -> String {
    if let Some(pat) = pattern {
        if !pat.is_empty() {
            return path.replace(pat, &Colour::Red.bold().paint(pat).to_string());
        }
    }
    path.to_string()
}

