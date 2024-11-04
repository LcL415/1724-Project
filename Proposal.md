# 1724 Course Project,  Find -- Documents Searching Tool

By Changlin Liu(Shawn), Junchen Zhu(Jason) and Yidi Liu(Louis)

## Group Member
Yidi Liu 1010763336

Changlin Liu 1004971480

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

We divided the project into tasks for each team member to make steady progress. Shawn, Jason, and Louis will each focus on specific parts of the tool. Our goal is to stay on track each week and complete the project by the due date.

---

#### Week 1-2 (Nov 5 – Nov 18): Setting Up and Basic Features

- **Shawn**: Set up the project in Rust and create the GitHub repository. Start working on **File Name Matching** and **Regular Expressions** in the **Basic Function Implementation** section. This will allow users to search for files by name or partial name, using regular expressions for more flexible matching.

- **Jason**: Begin implementing **Time Filtering** and **Size Filtering** in **Basic Function Implementation**. These features will allow users to filter search results based on file creation, modification, or access time, as well as file size.

- **Louis**: Work on **Recursive Search** functionality to enable searching through all files and folders within a specified directory. Also, start laying the groundwork for **Parallel Processing Support** to improve search speed on large directories.

---

#### Week 3-4 (Nov 19 – Dec 2): User Experience Enhancements

- **Shawn**: Design and implement the **Concise Command Format** and **Intuitive Parameter Naming** in the **User Experience Optimization** section. This will make the tool’s command-line interface more readable and easier to use.

- **Jason**: Add **Color Highlighting** to the output to help users quickly identify files and folders. Work on **Smart Default Value Filtering**, such as ignoring hidden files by default, to make searches more efficient.

- **Louis**: Start **Output Format Optimization** to ensure search results are displayed in a clear and organized way. Continue working on **Parallel Processing Support** to improve search performance in large directories.

---

#### Week 5-6 (Dec 3 – Dec 16): Advanced Features and Performance Optimization

- **Shawn**: Implement **Large-Scale Directory Performance Optimization** to reduce search time in large and deeply nested directories. Also, begin work on the **Cache Mechanism** to store recent search results, allowing for faster repeat searches.

- **Jason**: Begin developing **Type Filtering** to allow users to filter by file type (such as files, directories, or links). Also, start working on **Content Search Function** to support text searches within files, similar to `ripgrep`.

- **Louis**: Implement **.gitignore Support** so that the tool automatically reads `.gitignore` files and skips unnecessary files and folders. Finalize the **Parallel Processing Support** to ensure it works well across large file systems.

---

#### Final Week (Dec 10 – Dec 16): Final Testing, Documentation, and Demo

- **All Team Members**: Collaborate on comprehensive testing to catch any last bugs and confirm that all features work as expected. Perform cross-platform testing on Windows, macOS, and Linux to ensure compatibility.

- **Documentation and Demo Video**:
   - **Shawn**: Write the `README.md` with setup instructions, usage examples, and detailed descriptions of each feature.
   - **Jason**: Contribute to the user guide and help finalize the `README.md`.
   - **Louis**: Record and edit the demo video, showcasing the tool’s main features, how to use it, and how each part works.

---
