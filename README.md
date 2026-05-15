# typocalypse

A minimal terminal typing test with persistent stats.

## Features

- Practice typing with custom text or a word limit
- Tracks WPM, accuracy, correct, mistyped, and extra keystrokes
- Persists results to a local SQLite database
- View average stats across all sessions
- Keyboard-driven, no mouse required

## Installation

### From source

Requires [Rust](https://rustup.rs).

```bash
git clone https://github.com/AlejandroEVN/typocalypse
cd typocalypse
cargo install --path .
```

## Usage

Usage: typocalypse [OPTIONS]

Options:

-t <TEXT>    Raw text to practice with

-l <LIMIT>   Word limit (default: 30)

-p <PATH>    File path to read from

-r           Erase all historic test data

--help       Print this help message

### Examples

```bash
# Start with default text and 30 word limit
typocalypse

# Use custom text
typocalypse -t "the quick brown fox jumps over the lazy dog"

# Set a word limit
typocalypse -l 50

# Use a text from a file
typocalypse -p ~/projects/some/src/main.rs

# Reset all saved stats
typocalypse -r
```

## Key Bindings

| Key         | Action           |
|-------------|------------------|
| `⌥q`       | Quit             |
| `⌥r`       | Restart          |
| `⌥s`       | View stats       |
| `Esc`       | Back to home     |

## Stats

After completing a test, results are saved locally and you can view your averages at any time with `⌥s`. Stored stats include WPM, accuracy, typed, mistyped, extra keystrokes, and duration.

The database is stored at your platform's default data directory unless overridden with `-p`.

## License

MIT
