# 1724 Course Project, Real-Time Chat Application

By Shawn, Jason and Louis

## Group Member

## Summary

Develop an efficient command-line tool for finding files, similar to the `find` command in the Unix system. In addition to core functions such as recursive search, regular expressions, file name matching, basic filtering, etc., optimize the command-line tool's UI appearance, highlight prompts, parameter naming, output format, etc. to conform to the development and usage habits of modern programmers. According to the subsequent time plan, selectively complete extended functions such as parallel processing, multi-threaded search, and .gitignore support.

## Motivation


## Objective and key features


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

#### Final Week (Dec 10 – Dec 16): Testing, Polishing, and Demo

- **All Team Members**: Collaborate on thorough testing to fix any last bugs and make sure all features work as expected. Test on different platforms to confirm cross-platform compatibility.

- **Documentation and Demo Video**:
   - **Shawn**: Create the `README.md` file with setup instructions, usage examples, and a description of each feature.
   - **Jason**: Help with writing the user guide and contributing to the final `README.md`.
   - **Louis**: Record and edit a demo video, showing the tool’s main features and how to use it.
