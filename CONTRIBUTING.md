# Contributing to Ratel

Thank you for your interest in contributing to Ratel! 🎉

## 🚀 Quick Start

### Setup Development Environment
```bash
# Clone and setup
git clone https://github.com/biyard/ratel.git
cd ratel
source scripts/setup-local-env.sh

# Install prerequisites: Rust, Dioxus CLI, PostgreSQL, LLVM
# See docs/LOCAL_SETUP.md for details
```

### Project Structure
- `packages/main-ui/` - Dioxus frontend (React-like for Rust)
- `packages/main-api/` - Axum backend API
- `packages/mobile/` - Tauri mobile app
- `scripts/` - Development scripts

## 🐛 Issues

- **Search existing issues** before creating new ones
- Include: OS, Rust version, steps to reproduce, error logs
- Use labels: `bug`, `feature`, `documentation`, `good first issue`

## 🔄 Pull Requests

### Process
1. **Fork and branch**: `git checkout -b feature/your-feature`
2. **Follow code style**: `cargo fmt` and `cargo clippy`
3. **Test**: `cargo test`
4. **Commit**: Use conventional commits (`feat:`, `fix:`, `docs:`)
5. **Submit PR** with clear description

### PR Checklist
- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No merge conflicts

## 🎯 Good First Issues

Look for issues labeled:
- `good first issue` - Perfect for beginners
- `documentation` - Improve docs
- `translation` - Korean/English translations

### Easy Contributions
1. Fix typos or improve documentation
2. Add Korean/English translations
3. Improve error messages
4. Add unit tests

## 🤝 Getting Help

- Check [Local Setup Guide](docs/LOCAL_SETUP.md)
- Search existing issues
- Ask questions in GitHub Discussions
- Tag maintainers for urgent issues

---

**Happy Contributing! 🎉**
