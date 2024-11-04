# 1724 Course Project, Real-Time Chat Application

By Shawn, Jason and Louis

## Group Member

Yidi Liu 1010763336

## Summary

Develop an efficient command-line tool for finding files, similar to the `find` command in the Unix system. In addition to core functions such as recursive search, regular expressions, file name matching, basic filtering, etc., optimize the command-line tool's UI appearance, highlight prompts, parameter naming, output format, etc. to conform to the development and usage habits of modern programmers. According to the subsequent time plan, selectively complete extended functions such as parallel processing, multi-threaded search, and .gitignore support.

## Motivation



## Objective and key features

### Basic function implementation

- Recursive search: supports recursive traversal of all files and folders from the specified directory.
- File name matching: supports matching files based on names or partial names.
- Regular expressions: allows users to use regular expressions to find files.
- Time filtering: supports filtering by file creation time, modification time, or access time.
- Size filtering: allows filtering based on file size. 
- Type filtering: supports filtering by file type (such as files, directories, links, etc.).

### User experience optimization

- Concise command format: design a more readable and intuitive parameter format.
- Intuitive parameter naming: use parameter names that conform to modern development habits to simplify option settings.
- Colour highlighting: add colour highlights to the output so that users can quickly identify files and folders.
- Smart default value filtering: Provide default values for smart filtering (e.g., ignore hidden files or system files by default).
- Output format optimization: optimize the output layout and display clear file information.

### Advanced functions and performance optimization(Optional)

- Parallel processing support: implement multi-threading or parallel processing to improve search efficiency.
- Large-scale directory performance optimization: Optimize for large folders and deeply nested directories to reduce search time.
- Cache mechanism: Support result caching to improve performance when searching repeatedly.
- .gitignore support: Automatically read .gitignore files and skip unnecessary files and folders.
- Content search function: Optional support for file content search function, similar to `ripgrep`, for searching for specific text content in files.

## Tentative plan