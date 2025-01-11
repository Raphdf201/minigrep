# A little copy of grep

![Rust build](https://github.com/Raphdf201/minigrep/actions/workflows/rust.yml/badge.svg)

i made this to train my rust skills

## Usage

This is a cli program, wich means you need to execute it using `.\searcher_txt.exe` on windows and `./searcher_txt` on linux and MacOS<br/>
The arguments needed are the query, filename and case sensitivity
Example for searching "potato" in file.txt with no case sensitivity on windows :

```
.\searcher_txt potato file.txt false
```

If you want your search to be case sensitive :

```
.\searcher_txt potato file.txt true
```

If you're on linux or MacOS, replace the \ by /

## Changelog

1.2.4 : Documentation

1.2.3 : Small optimizations

1.2.2 : Case sensitivity is now an argument

1.2.1 : Better error messages

1.2.0 : Added back case sensitivity option

1.1.0 : Fixed some errors

1.0.0 : Basic code (with errors)
