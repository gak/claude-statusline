# Claude Statusline

A Rust binary that provides a clean statusline for Claude Code, replacing the complex bash command with native Rust and `jj-lib` integration.

## Features

- **Native jj integration**: Uses `jj-lib` directly instead of shelling out to commands
- **Vibrant 24-bit colors**: Teal directory, hot pink jj status, electric orange model name, neon lime output style
- **Dynamic emojis**: Visual indicators that change based on repository state
- **Smart path formatting**: Home directory abbreviation and path truncation
- **JSON input parsing**: Reads Claude Code's JSON data from stdin
- **Test-driven development**: Comprehensive test coverage

## Installation

```bash
cargo install --path .
```

This installs `claude-statusline` to `~/.cargo/bin/`.

**Status**: ✅ **COMPLETED** - Binary built, tested, and installed successfully!

## Usage

The binary reads JSON from stdin (provided by Claude Code) and outputs a formatted statusline:

```bash
echo '{"workspace":{"current_dir":"/Users/gak/src/grabby"},"model":{"display_name":"Claude 3.5 Sonnet"},"output_style":{"name":"default"}}' | claude-statusline
```

Output: `📂~/src/grabby (⚡abc123 main*) 🧠Claude 3.5 Sonnet [🎭Learning]`

## Configuration

Update your Claude Code settings (`~/.claude/settings.json`):

```json
{
  "statusLine": {
    "type": "command",
    "command": "claude-statusline"
  }
}
```

**Status**: ✅ **CONFIGURED** - Claude Code now uses the Rust binary!

## Development

Run tests:
```bash
cargo test
```

Run with sample input:
```bash
cargo run < sample_input.json
```

## Architecture

- `src/input.rs` - JSON parsing and data structures
- `src/directory.rs` - Path formatting (home abbreviation, truncation)
- `src/jj_status.rs` - Native jj repository status using `jj-lib`
- `src/output.rs` - Colored terminal output formatting
- `src/main.rs` - CLI entry point

## Visual Elements

### Emojis
- **📂** Directory path indicator
- **🔀** Clean jj repository (no uncommitted changes)  
- **⚡** Dirty jj repository (uncommitted changes present)
- **🧠** Model name indicator
- **🎭** Output style indicator (when not default)

### Status Indicators
- `(🔀abc123 main)` - Clean repository with change ID and bookmarks
- `(⚡abc123 main*)` - Dirty repository with uncommitted changes
- `(⚡abc123 main conflict*)` - Repository with conflicts and changes
- No parentheses when not in a jj repository

### Color Scheme
- **Directory path**: Vibrant Teal `RGB(64, 224, 208)`
- **JJ status**: Hot Pink `RGB(255, 20, 147)` 
- **Model name**: Electric Orange `RGB(255, 140, 0)`
- **Output style**: Neon Lime `RGB(50, 205, 50)`

All colors use 24-bit true color for maximum vibrancy on modern terminals like Ghostty.

### Example Outputs
- Clean repo: `📂~/src/project (🔀abc123 main) 🧠Claude 3.5 Sonnet`
- Dirty repo: `📂~/src/project (⚡abc123 main*) 🧠Claude 3.5 Sonnet [🎭Learning]`
- No repo: `📂~/src/project 🧠Claude 3.5 Sonnet`