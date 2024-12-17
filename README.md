# 1724-Project
## FIND COMMAND LINE TOOL

DEMO VIDEO is in git repo.

# **Group Member**

Yidi Liu  1010763336  id.liu@mail.utoronto.ca

Changlin Liu 1004971480  changlin.liu@mail.utoronto.ca

Junchen Zhu 1004900271    jasonjunchen.zhu@mail.utoronto.ca

# Motivation

Our motivation for this project stems from a desire to create a challenging yet rewarding tool that is enjoyable to build and valuable to the Rust ecosystem. While exploring the current landscape of file-searching utilities, we identified a notable gap: the lack of a powerful, user-friendly command-line tool that seamlessly integrates modern features like colour-coded output, recursive directory scanning, case-insensitive and regex-based searches, and advanced file handling capabilities—all in one cohesive solution.

Although tools like `find` exist, they often lack intuitive user interfaces or require users to piece together functionality from multiple utilities. This insight inspired us to develop a command-line utility that combines the speed and efficiency of Rust with an improved feature set and a more accessible user experience. Our focus was not merely to replicate existing tools but to enhance them by introducing innovations such as enhanced colour support, robust regex capabilities, and granular control over search results. These features aim to make working with large codebases and file collections more efficient and enjoyable.

At its core, this project is driven by our passion for Rust and our commitment to solving practical problems in a way that delivers tangible value to developers. By addressing the shortcomings of existing tools and envisioning a unified, innovative solution, we are motivated to contribute something meaningful to the Rust ecosystem—one that blends performance, usability, and modern capabilities into a single, intuitive tool.

# **Objectives**

The primary objective of this project is to develop a robust and user-friendly command-line file-searching utility that bridges the gap between existing tools and modern developer needs. By leveraging the power of Rust, the tool aims to deliver exceptional performance, enhanced usability, and an intuitive interface while providing advanced features that cater to a wide range of use cases.

**Seamless File Searching**: This feature enables users to efficiently search files and directories, supporting recursive searches, exclusion patterns, and regular expression matching for precise file targeting.

**Customizable Search Experience**: Introduce filtering options based on file properties such as size, date, and extensions, as well as a fuzzy search mechanism to simplify finding files with uncertain or partial names.

**Advanced Ignore Mechanisms**: Incorporate intelligent file and directory exclusion, including automatic `.gitignore` integration and hidden file handling, to minimize irrelevant search results and enhance user productivity.

**High Performance**: Implement a **high-performance file search** tool that supports complex filtering criteria and efficiently handles large directories. Utilize **multi-threading** and **optimized resource management** to accelerate file system traversal and matching operations.

**Enhanced User Interaction**: To ensure a pleasant and practical user experience, offer clean and versatile command-line interaction, with colour-coded output, customizable results display, and options for result redirection.

# **Features**

## Basic Functions

**Search files in a specified directory**:

- Recursively search files starting from the user-specified directory.

**Exclude directories**:

- Allows users to specify directories to exclude from the search.

**Regular expression matching**:

- Uses regular expressions for flexible file name matching, allowing precise targeting of desired files.

## Ignore Mechanism

**Ignore specified files/directories**:

- Provides options to exclude specific files or directories, such as .git or node_modules.

**Ignore System files or hidden files and read `.gitignore` automatically:**

- Provides options to exclude files or directories listed in the `.gitignore` file
- Ignoring hidden files (files that start with a .) is enabled by default.

## Multiple Filtering Mechanism

**File properties filtering:**

- Support filter files by their size, date, and extension name

**More user-friendly fuzzy search:**

- Split file name and fuzzy search to have a better experience when the file name is not certain

## Performance

- **Parallel File Traversal**: Uses `ignore::WalkBuilder` for multi-threaded directory traversal with configurable thread count.
- **Regex Filtering**: Compiles and shares regex patterns across threads using `Arc` for efficient name matching.
- **Exclusion Optimization**: Leverages `DashMap` to cache and efficiently filter excluded directories.
- **Cross-Thread Communication**: Employs `crossbeam_channel` for seamless and non-blocking result collection from threads.

## Command line interaction

**Search result output**:

- Outputs matching file paths to the terminal.

# **User’s Guide**

## **Installation**

### **Step 1: Download or Clone the Project**

Download the source code or clone the repository:

```bash
git clone git@github.com:LcL415/1724-Project.git
cd 1724-Project
```

---

### **Step 2: Build the Project**

Run the following command to build an optimized release version:

```bash
cargo build --release
```

This will generate the binary in the `target/release` folder.

---

### **Step 3: Run the Tool**

Navigate to the `target/release` directory:

```bash
cd target/release
```

Run the tool:

- On **Linux/Mac**:
    
    ```bash
    ./fd --help
    ```
    
- On **Windows**:
    
    ```bash
    fd.exe --help
    ```
    

## Basic Usage

### **Syntax**

```bash
fd [OPTIONS] [NAME]
```

- `[NAME]`: The file name to search (optional).
- `[OPTIONS]`: Additional filters or flags.

## **Command-line options**

This is the output of `fd --help`. To see the complete set of command-line options, use:

```bash
fd --help
```

### **Usage**

```bash
fd [OPTIONS] [NAME]
```

- `[NAME]`: The file name to search (optional).

### **Options**

| Option | Description |
| --- | --- |
| `--directory <DIRECTORY>` | The directory to search. **Default**: `.` (current directory). |
| `--pattern <PATTERN>` | The regex pattern to match in file names. |
| `-x, --exclude <EXCLUDE>` | Comma-separated list of directories to exclude. |
| `--absolute` | Output absolute file paths. |
| `--no-ignore-hidden` | Include hidden files and directories. (Negates `--ignore-hidden`). |
| `--threads <THREADS>` | Number of threads to use for the search. **Default**: 8. |
| `--min-size <MIN_SIZE>` | Minimum file size in bytes. |
| `--max-size <MAX_SIZE>` | Maximum file size in bytes. |
| `--max-depth <MAX_DEPTH>` | Set maximum directory traversal depth. |
| `--min-date <MIN_DATE>` | Minimum file modification date (RFC3339 format, e.g., `2024-01-01T00:00:00Z`). |
| `--max-date <MAX_DATE>` | Maximum file modification date (RFC3339 format). |
| `--file-extension <EXT>` | Filter by file extension (e.g., `txt`, `rs`). |
| `--gitignore-path <PATH>` | Path to a `.gitignore` file. Skips files matching the patterns in the specified `.gitignore`. |
| `--fuzzy-patterns <PATTERNS>` | Comma-separated list of fuzzy patterns to match (e.g., `file,example`). |
| `--max-fuzzy-distance <NUM>` | Set the maximum allowed distance for fuzzy matching. **Default**: 16. |
| `--skip-size-filter` | Skip size filtering. |
| `--skip-date-filter` | Skip date filtering. |
| `--skip-file-type-filter` | Skip file type filtering (e.g., file extension). |
| `--skip-gitignore-filter` | Skip filtering based on `.gitignore`. |
| `--skip-fuzzy-filter` | Skip fuzzy matching filtering. |
| `-h, --help` | Print the help message. |
| `-v, --version` | Print the version information. |

### **Examples(Windows Example)**

### 1. Basic Search

```bash
fd.exe --directory C:\Users\shado\Documents --pattern ".*test.*"
```

Search for files containing "test" in their names

### 2. Search Hidden Files

```bash
fd.exe --directory C:\Users\shado\Documents --file-extension "txt" --no-ignore-hidden
```

Include hidden files while searching for `.txt` files

### 3. Exclude Directories

```bash
fd.exe --directory C:\Users\shado\Documents --file-extension "txt" --exclude "temp,node_modules"
```

Search for `.txt` files but exclude `temp` and `node_modules` directories

### **4. Search by File Size**

Find files between 1 KB and 1 MB in size:

```bash
fd.exe --directory C:\Users\shado\Documents --min-size 1024 --max-size 1048576
```

### **5. Search by Modification Date**

Find files modified after January 1, 2024:

```bash
fd.exe --directory C:\Users\shado\Documents --min-date "2024-01-01T00:00:00Z"
```

### **6. Output Absolute Paths**

Display absolute paths of matching `.log` files:

```bash
fd.exe --directory C:\Users\shado\Documents --file-extension "log" --absolute
```

### **7. Fuzzy Search**

Find files with names similar to "example" (allow typos up to 2 characters):

```bash
fd.exe --directory C:\Users\shado\Documents --fuzzy-patterns "exmple" --max-fuzzy-distance 2
```

### **8. Skip Filters**

Search for files while skipping file size and date filters:

```bash
fd.exe --directory C:\Users\shado\Documents --pattern ".*\.txt" --skip-size-filter --skip-date-filter
```

## Troubleshooting

| **Issue** | **Solution** |
| --- | --- |
| `Permission Denied` error | Ensure you have permission to access the target directory. Run as administrator if needed. |
| Command not found | Ensure the binary is in your PATH or run it using its full path. |
| Incorrect search results | Check your filters (e.g., regex pattern, size, or date options). |
| Slow performance on large directories | Adjust the `--threads` option for better parallel performance. |

For additional help, use:

```bash
fd.exe --help
```

# **Reproducibility Guide**

This section provides detailed instructions to reproduce the functionality of project. Follow the steps below to set up, run, and test the tool.

### 1. Prerequisites

Before starting, ensure you have the following installed on your system:

- **Rust**: Install Rust using [rustup](https://rustup.rs/).
- **Git**: To clone the project repository.
- **Command Line Interface (CLI)**: Such as `Terminal` on macOS/Linux or `Command Prompt`/`PowerShell` on Windows.

Optional:

- A code editor like Visual Studio Code for modifying or reviewing the code.
- `cargo` and `rustc`, which are installed with Rust.

Operating System: macOS or Windows 10/11.

### 2. Clone the Repository

Download the project source code from the repository using Git:

```bash
git clone git@github.com:LcL415/1724-Project.git
cd 1724-Project
```

### 3. **Build the Project**

1. Navigate to the project root directory (where the `Cargo.toml` file is located):
    
    
2. Build the project in **release mode** for optimized performance:
    
    ```bash
    cargo build --release
    ```
    
3. The compiled binary will be generated in the `target/release` directory:
    - **Windows**: `fd.exe`
    - **macOS**: `fd`
4. Verify the binary:
    
    ```bash
    ./target/release/fd.exe --version
    ```
    

### 4. Run the Tool

**Basic Test**

Run the following command to display help:

```bash
./target/release/fd.exe --help
```

**Search Example**

Search for `.txt` files in the `Documents` directory:

```bash
./target/release/fd.exe --directory C:\Users\shado\Documents --pattern ".*\.txt"
```

**Performance Test Example**

Run a search using 8 threads:

```bash
./target/release/fd.exe --directory C:\Users\shado\Documents --threads 8 --pattern ".*\.log"
```

### 5. **Common Issues and Fixes**

| **Issue** | **Cause** | **Solution** |
| --- | --- | --- |
| `cargo: command not found` | Rust is not installed or not in PATH. | Install Rust using [Rustup](https://rustup.rs/). |
| `Permission Denied` | Binary lacks execution permissions. | Run with admin rights or use `chmod +x`. |
| `fd.exe is not recognized` | Binary is not in PATH. | Use full path: `./target/release/fd.exe`. |
| Output missing expected files | Directory path or pattern is incorrect. | Double-check the directory and pattern. |
| Slow performance on large directories | Default thread count is insufficient. | Use `--threads` to increase thread count. |

### 6. Results Verification

To verify correct execution, compare outputs for:

### **Command 1: Search for `.txt` files**

```bash
./target/release/fd.exe --directory C:\Users\shado\Documents --file-extension "txt"
```

**Expected Output**: List of `.txt` files with their paths.

---

### **Command 2: Exclude Directories**

```bash
./target/release/fd.exe --directory C:\Users\shado\Documents --pattern ".*" --exclude "temp,node_modules"
```

**Expected Output**: Files excluding those in `temp` and `node_modules`.

---

### **Command 3: Search Hidden Files**

```bash
./target/release/fd.exe --directory C:\Users\shado\Documents --no-ignore-hidden
```

**Expected Output**: Files including hidden ones.

---

### **Command 4: Fuzzy Search**

```bash
./target/release/fd.exe --directory C:\Users\shado\Documents --fuzzy-patterns "exmple" --max-fuzzy-distance 2
```

**Expected Output**: Files with names close to "example" (allowing typos).

# **Contributions**

### **Yidi Liu**

- **Key Roles**: Responsible for the development of search and filter modules
- **Specific Contributions**: Developed and implemented multiple filtering functions in the filter module and implemented a fuzzy query algorithm. Investigated and compared the high-performance file traversal methods in the search module, and used Arc, crossbeam_channel and other methods to optimize the performance bottlenecks of some search processes.

### **Changlin Liu**

- **Key Roles**: **Designed and implemented the core search functionality**
- **Specific Contributions**: Developed the main logic for file traversal and filtering using the walkdir library; Implemented parallel processing with the rayon library to improve performance; Added advanced filtering options, Implemented file filtering features based on regular expressions (regex) and file size (min-size and max-size), Added features like --threads for multithreading control and --ignore-hidden for hidden file filtering.

### **Junchen Zhu**

- **Key Roles**: Tester and Report Contributor
- **Specific Contributions**:  I conducted thorough testing of the tool to verify the functionality of all command-line options, including file searching, filtering by size, date, and extension, as well as fuzzy search and directory exclusions. I ensured the tool worked reliably across edge cases, such as large directories, hidden files, and special file paths. Additionally, I documented the testing process, results, and verification steps in the project report, contributing to the Testing section and the Reproducibility Guide to help users replicate the environment and test the tool successfully.

# **Lessons learned and concluding remarks**

### Lessons Learned

1. **Understanding the Power of Rust**:
    - **Ownership and Borrowing**: Initially, we struggled with Rust's ownership model, especially when working with multiple threads. Over time, we learned how to manage references and mutable data safely without sacrificing performance.
    - **Crates and Ecosystem**: Integrating crates like `Arc` for parallelism and `Ignore` for directory traversal taught us how to efficiently use the Rust ecosystem to solve complex problems without reinventing the wheel.
2. **Efficient Multithreading**:
    - **Thread Safety**: Implementing multithreading with `rayon` showed us how Rust enforces thread safety at compile time. For example, we faced challenges when sharing data across threads but resolved them by using thread-safe types like `Arc`.
    - **Performance Tuning**: We learned how to optimize thread usage by dynamically adjusting the thread pool size (`-threads` option), ensuring a balance between speed and resource consumption.
3. **Regex Optimization**:
    - We underestimated the complexity of regular expression matching on large directories. Experimenting with the `regex` crate helped us optimize performance by pre-compiling patterns and reducing unnecessary checks during file traversal.
4. **Edge Case Handling**:
    - Through rigorous testing, we encountered unexpected issues, such as handling hidden files, files without extensions, and symbolic links. We learned to anticipate and programmatically address these cases to ensure a seamless user experience.
5. **Working with `.gitignore` Files**:
    - Parsing `.gitignore` rules to exclude files taught us how to process and apply rules dynamically. We learned how to handle scenarios where `.gitignore` files contained invalid rules or were missing entirely.
6. **Fuzzy Matching Challenges**:
    - Implementing Levenshtein-based fuzzy matching required understanding the trade-offs between accuracy and performance. By experimenting with thresholds for match distances, we found a balance that worked well for most use cases.
7. **Effective Debugging in Complex Codebases**:
    - Debugging multi-threaded code was particularly challenging. We became proficient in using tools like Rust's logging macros (`println!` and `dbg!`) and debugging utilities such as `cargo test --nocapture`.
8. **Collaboration and Version Control**:
    - Managing merge conflicts in Git taught us how to organize our work branches and resolve issues without disrupting progress. We also learned to document changes clearly to maintain a shared understanding of the project.

Although not as complete and powerful as the find command, the command line tool we developed can achieve dozens of times the efficiency improvement in some search tasks. This surprised us very much, because find is a very mature and high-performance Unix tool. Our development project achieved a certain degree of complete functionality and good performance in a short development cycle.

This makes us have more expectations for the potential of Rust. Although there is no killer development application of Rust yet, and the ecosystem is mature but not widely promoted, we believe that Rust will be widely used in the future because of its rigorous compilation characteristics and excellent performance.

![https://p.ipic.vip/5ygeou.png](https://p.ipic.vip/5ygeou.png)

![https://p.ipic.vip/ua4m37.png](https://p.ipic.vip/ua4m37.png)

### **Concluding Remarks**

The project has been a valuable learning experience. It allowed us to build a complex software tool while solving practical challenges. From designing the search algorithm to implementing advanced features, we learned how to break down problems and tackle them step by step.

One of the main things we learned was how to optimize file search operations. We used multithreading to process large directories faster. By experimenting with the `rayon` crate, we saw how splitting tasks across multiple threads improved performance. This helped us handle large-scale directories efficiently. However, our final implementation did not use the `rayon` library, but instead used other more efficient methods of traversing files.

We have made underlying optimizations to the file search module, and the overall efficiency has been greatly improved. First, through parallel traversal, we use a file scanning mechanism that supports multi-threading to make directory search faster, and at the same time, we can flexibly adjust the number of threads as needed. Secondly, we have optimized the regular expressions, avoiding the performance overhead of repeated compilation by sharing compiled regular expressions, and only matching file names, reducing useless calculations. 

We also learned how to work with regular expressions and fuzzy matching. Implementing Levenshtein-based fuzzy matching required balancing speed and accuracy. We tested different configurations to ensure it worked well without slowing down the tool. This taught us the importance of tuning algorithms to match specific use cases.

Another challenge was handling real-world scenarios, like excluding hidden files and processing `.gitignore` rules. Initially, these edge cases caused unexpected results, but we resolved them by refining our code. This experience showed us how critical it is to test software in diverse environments.

Making the tool user-friendly was another key focus. Features like color-coded output, relative and absolute path options, and helpful error messages were added based on feedback. These improvements made the tool easier to use and more practical for different users.

Finally, collaboration played an important role in this project. We divided tasks among the team and used version control to keep track of changes. Regular communication and code reviews helped us maintain high-quality code and avoid misunderstandings. Planning and sticking to a weekly schedule also ensured that we stayed on track.
