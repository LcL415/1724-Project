# 1724 Course Project,  Find -- Documents Searching Tool

By Changlin Liu, Junchen Zhu and Yidi Liu

## Group Member
Yidi Liu 1010763336
Junchen Zhu 1004900271


## Summary

Develop an efficient command-line tool for finding files, similar to the `find` command in the Unix system. In addition to core functions such as recursive search, regular expressions, file name matching, basic filtering, etc., optimize the command-line tool's UI appearance, highlight prompts, parameter naming, output format, etc. to conform to the development and usage habits of modern programmers. According to the subsequent time plan, selectively complete extended functions such as parallel processing, multi-threaded search, and .gitignore support.

## Motivation

Our motivation for this project stemmed from a combination of our desire to create a challenging and satisfying project that would be both fun to build and useful to the Rust ecosystem. We identified a gap in the current landscape for a powerful yet easy-to-use command-line tool for text searching that could seamlessly integrate modern features such as color-coded output, recursive directory scanning, case-insensitive and regex-based searches, and more advanced file handling capabilities. Although there are existing tools in the ecosystem, like `grep`, we found that there were certain user experience improvements and feature combinations that are either hard to find or scattered across multiple tools.

This motivated us to develop a command-line utility that aims to combine the speed and efficiency of Rust with an intuitive user interface and an improved feature set that allows for more flexibility and functionality. Our goal was not just to replicate existing tools, but to improve upon them in ways that would make them more accessible and user-friendly, without compromising on the power and performance that users expect. In particular, we wanted to bring enhanced color support, better regex capabilities, and more granular control over search results to make working with large codebases and file collections more efficient and pleasant.

Ultimately, this project was driven by our collective passion for Rust and for solving practical problems in a way that adds tangible value to developers. By identifying the gap and envisioning a unified solution that didn't previously exist in the Rust ecosystem, we felt a strong motivation to dedicate our time and energy to making this idea a reality.



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

We divided the project into tasks for each team member to make steady progress. Shawn, Jason, and Louis will each focus on specific parts of the tool. Our goal is to stay on track each week and finish everything by the due date.

---
#### Week 1-2 (Nov 5 – Nov 18): Setting Up and Core Features

- **Shawn**: Set up the project in Rust and create the GitHub repository. Begin working on **Pattern Matching** in **Advanced Search Options**. This will allow users to search for files by name, using regular expressions and wildcards.
  
- **Jason**: Start on **Enhanced Filtering Capabilities** with a focus on **Metadata Filters** (like file size, modification date, etc.). This feature will help users do more specific searches from the start.

- **Louis**: Work on **Recursive Directory Search** to enable searching through subfolders. Louis will also start adding multi-threading for faster searches on large file systems.

---

#### Week 3-4 (Nov 19 – Dec 2): Advanced Features and Indexing

- **Shawn**: Develop **Multi-Threaded Search** to make searches faster. This will use multiple threads to handle large directories more efficiently.

- **Jason**: Continue with **Enhanced Filtering Capabilities** by adding **File Type and Extension Filters** and start **File Content Search**. These features will allow users to narrow their search based on file type and file content.

- **Louis**: Begin building the **Database Indexing Mode**. Set up a local database to index files for faster repeat searches. Also, add options for users to update the index automatically or manually.

---

#### Week 5-6 (Dec 3 – Dec 16): Final Features, Testing, and Documentation

- **Shawn**: Implement **Real-Time Directory Monitoring**. This will track changes in specified folders, such as file additions, deletions, and modifications. Shawn will also handle **Automatic Re-indexing** to keep the index up-to-date when files change.

- **Jason**: Finalize **Customizable Output and Integration Options**. This includes adding JSON/CSV export options and allowing search results to be piped to other command-line tools. Jason will also work on sorting and grouping features for better-organized search results.

- **Louis**: Finish any remaining work on **Database Indexing Mode** and assist with testing. Louis will also make sure the tool works well on all platforms (Windows, macOS, and Linux) and that all features work together smoothly.

---

#### Final Week (Dec 10 – Dec 16): Testing, Demo

- **All Team Members**: Collaborate on thorough testing to fix any last bugs and make sure all features work as expected. Test on different platforms to confirm cross-platform compatibility.

- **Documentation and Demo Video**:
   - **Shawn**: Create the `README.md` file with setup instructions, usage examples, and a description of each feature.
   - **Jason**: Help with writing the user guide and contributing to the final `README.md`.
   - **Louis**: Record and edit a demo video, showing the tool’s main features and how to use it.
