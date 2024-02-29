# Agape Deus

This is a simple CLI tool to fetch and print today's daily readings in the catholic church, written in Rust

# Usage

```bash
$ agape --help

Usage: agape [OPTIONS]

Options:
  -d, --date <DATE> Get the readings for a specific date (expected as a unix timestamp)
  -s, --sunday      Get the readings for the upcoming sunday mass
  -h, --help        Print help information
  -V, --version     Print version information
```

# Source

The readings are retrieved from [Universalis.com](https://universalis.com/)