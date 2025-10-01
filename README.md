# A little copy of grep

![Rust build](https://github.com/Raphdf201/minigrep/actions/workflows/rust.yml/badge.svg)
![Crates.io Version](https://img.shields.io/crates/v/searcher_txt)
![docs.rs](https://img.shields.io/docsrs/searcher_txt)
![Crates.io Size](https://img.shields.io/crates/size/searcher_txt)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/searcher_txt)
![Crates.io Total Downloads](https://img.shields.io/crates/d/searcher_txt)
![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/searcher_txt)

## Usage

This is a cli program, which means you need to execute it using `.\searcher_txt.exe` on windows and `./searcher_txt` on
linux and macOS

The arguments needed are the query and filename. You can also add optional arguments like case sensitivity or verbose
output. Example for searching "potato" in file.txt with no case sensitivity on windows :

```
.\searcher_txt potato file.txt
```

If you want your search to be case-sensitive :

```
.\searcher_txt potato file.txt --case
```

If you want to search recursively, print the whole file, be case-sensitive and verbose, 

```
.\searcher_txt potato folder/ -cvwr
```

Available arguments :
`--case/-c`
`--verbose/-v`
`--whole/-w`
`--recurse/-r`
`--help/-h`
`--version/-V`

If you're on linux or macOS, replace the \ by /

[Info](https://docs.raphdf201.net/minigrep/)

## Changelog

1.2.8 : Add recursive argument

1.2.7 : Update dependencies

1.2.6 : Print whole file function

1.2.5 : Verbose argument

1.2.4 : New argument usage

1.2.3 : Small optimizations

1.2.2 : Case sensitivity is now an argument

1.2.1 : Better error messages

1.2.0 : Added back case sensitivity option

1.1.0 : Fixed some errors

1.0.0 : Basic code (with errors)

## Testing

`cargo test`

## Code coverage
```
cargo install cargo-llvm-cov
cargo llvm-cov --open
```


