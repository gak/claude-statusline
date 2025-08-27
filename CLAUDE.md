# Claude Statusline Development Notes

## Quick Commands

### Development
```bash
# Run tests
cargo test

# Build and test locally
cargo run < sample_input.json

# Install updated binary
cargo install --path .
```

### Git/JJ Workflow
```bash
# Create feature branch
jj bookmark create feature-name

# Commit changes
jj describe -m "feat: description of changes"

# Push and create PR
jj git push --branch feature-name --allow-new
gh pr create --title "Title" --body "Description" --head feature-name --base main
```

## Architecture Notes

### Color Scheme (24-bit RGB)
- **Directory**: Teal `RGB(64, 224, 208)`
- **JJ Branch Names**: Bright Hot Pink `RGB(255, 20, 147)` - for prominence  
- **JJ Commit IDs**: Duller Hot Pink `RGB(200, 80, 140)` - for subtlety
- **JJ Conflicts**: Duller Hot Pink `RGB(200, 80, 140)`
- **Model**: Electric Orange `RGB(255, 140, 0)`
- **Output Style**: Neon Lime `RGB(50, 205, 50)`
- **Separators**: Dark Grey `RGB(96, 96, 96)`

### File Structure
- `src/input.rs` - JSON parsing from Claude Code
- `src/directory.rs` - Path formatting (~ expansion, truncation)
- `src/jj_status.rs` - JJ repository detection and status parsing
- `src/output.rs` - Colored terminal output with separate color handling
- `src/main.rs` - CLI entry point

### Key Design Decisions
- Uses `jj` command execution instead of `jj-lib` (API stability concerns)
- Individual color handling for jj components (change ID vs branch names)
- Test-driven development with 30 comprehensive tests
- Modular architecture for easy future enhancements

### Testing Strategy
- Unit tests for each module
- Integration tests with sample JSON inputs
- Color code verification in tests
- Test both with/without jj repositories

## Future Enhancements
- Consider `jj-lib` migration when API stabilizes
- Additional repository types (git fallback?)
- Configurable color themes
- Performance optimizations for large repositories