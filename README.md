# Claude Statusline

A vibrant Rust statusline for Claude Code that was **vibe coded** with Claude! 🎨✨

```
📂 ~/src/project ‧ ⚡ abc123 main* ‧ 🧠 Claude 3.5 Sonnet ‧ 🎭 Learning
```

**With colors:**
<p>
<span style="color: #40E0D0">📂 ~/src/project</span> <span style="color: #606060">‧</span> <span style="color: #FF1493">⚡ abc123 main*</span> <span style="color: #606060">‧</span> <span style="color: #FF8C00">🧠 Claude 3.5 Sonnet</span> <span style="color: #606060">‧</span> <span style="color: #32CD32">🎭 Learning</span>
</p>

This binary replaces complex bash commands with clean Rust code featuring dynamic emojis, 24-bit colors, and intelligent jj repository detection.

## Features

- **Smart jj integration**: Detects jj repositories and retrieves status information
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

Output: `📂 ~/src/grabby ‧ ⚡ abc123 main* ‧ 🧠 Claude 3.5 Sonnet`

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

This project was **vibe coded** with Claude using a test-driven development approach, going from concept to a fully-featured statusline with dynamic emojis and beautiful colors!

Run tests:
```bash
cargo test
```

Run with sample input:
```bash
cargo run < sample_input.json
```

### Vibe Coding Journey
1. 🏗️ Started with TDD approach and modular architecture
2. 🎨 Evolved colors from basic to vibrant 24-bit true colors  
3. ✨ Added dynamic emojis that change based on repository state
4. 🧹 Refined to clean layout with elegant section separators
5. 🚀 All developed through natural conversation and iteration!

### Implementation Notes
- Currently uses `jj` commands for repository detection (reliable and simple)
- Ready for future migration to `jj-lib` native integration when API stabilizes
- Designed with clean abstractions to support either approach

## Architecture

- `src/input.rs` - JSON parsing and data structures
- `src/directory.rs` - Path formatting (home abbreviation, truncation)
- `src/jj_status.rs` - jj repository status detection and parsing
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
- `🔀 abc123 main` - Clean repository with change ID and bookmarks
- `⚡ abc123 main*` - Dirty repository with uncommitted changes
- `⚡ abc123 main conflict*` - Repository with conflicts and changes
- No jj status section when not in a jj repository

### Color Scheme
- **Directory path**: Vibrant Teal `RGB(64, 224, 208)`
- **JJ status**: Hot Pink `RGB(255, 20, 147)` 
- **Model name**: Electric Orange `RGB(255, 140, 0)`
- **Output style**: Neon Lime `RGB(50, 205, 50)`

All colors use 24-bit true color for maximum vibrancy on modern terminals like Ghostty.

### Example Outputs
- Clean repo: `📂 ~/src/project ‧ 🔀 abc123 main ‧ 🧠 Claude 3.5 Sonnet`
- Dirty repo: `📂 ~/src/project ‧ ⚡ abc123 main* ‧ 🧠 Claude 3.5 Sonnet ‧ 🎭 Learning`
- No repo: `📂 ~/src/project ‧ 🧠 Claude 3.5 Sonnet`

### Layout Features
- **Clean spacing**: Space after each emoji for readability
- **Section separators**: Dark grey middle dots `‧` between sections `RGB(96, 96, 96)`
- **No brackets**: Removed parentheses and square brackets for cleaner look
- **Consistent flow**: Uniform spacing and visual hierarchy