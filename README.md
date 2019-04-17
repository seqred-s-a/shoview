# Shoview

Shoview is a tool that queries mongodb database built by shosearch, and generates HTML reports.

# Basic usage

Make sure mongod is running. (To install on Mac: `brew install mongodb`, then `mongod` to start daemon).

Make sure `cargo` command is working. To install cargo, go to https://rustup.rs/ and follow the instructions.

Make sure database contains data (shosearch is the tool for that).

You can run `cargo run -- -h` to see help.

Usage examples:
* `cargo run -- -o out.html` will get data from default collection and save report in out.html file
* `cargo run -- -o out.html -c cshodan -q '{"vulns" : {"$exists" : true}}'` will create report with all vulnerable hosts from collection cshodan

# Building

Shoview is written in Rust. Can be build using *cargo*, like any other Rust program.

To install cargo, go to https://rustup.rs/ and follow the instructions.

To build shoview, run
```
cargo build --release
```
in repo's main directory.

The binary will be in target/release folder.

Binary requires *templates* folder to be in the same directory, make sure to copy it when releasing.
