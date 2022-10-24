# Advent Of Code input-importer

Tired of manually importing the puzzle data for  [Advent of Code](https://adventofcode.com)? You're in luck!

## Usage

```bash
cargo run -- [session] [year]
```

| Parameter | Description |
| :---:     |       :---: |
| session   | Advent of Code session stored in cookie     |
| year      | Optional. Year to import|

## Resulting File Structure

```
[home directory]
└── AdventOfCode
    └ ...
    └ 2020
    └ 2021
      └ day01
          input.txt
      └ day02
          input.txt
      └ ...
```
