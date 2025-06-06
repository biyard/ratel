# Changelog

## 2025-05-29

### Added
- **Streamlined development environment**
  - Integrated Makefile with comprehensive development workflow
  - Single-command setup: `make start` launches all services
  - Real-time service monitoring with `make status`
  - Automatic tool installation and dependency management
  - Development log aggregation in `logs/` directory

- **Local development authentication**
  - Environment-aware authentication (local vs production)
  - Automatic Ed25519 keypair generation for local development
  - Local wallet storage in browser LocalStorage
  - Firebase emulator integration for testing

- **Build optimizations**
  - Fast development mode with `make run-fast` (incremental compilation)
  - Apple Silicon WASM compilation fixes
  - Cargo workspace profile optimization
  - Hot reload support for frontend development

- **Enhanced documentation**
  - Comprehensive `LOCAL_DEVELOPMENT.md` guide
  - Updated `README.md` with accurate development workflow
  - Simplified `CONTRIBUTING.md` setup instructions

### Fixed
- **Critical authentication bug**: Removed call to non-existent `request_local_wallet()` method
- **Local development workflow**: Eliminated script-based setup in favor of Makefile
- **WASM compilation issues**: Fixed Apple Silicon build problems with proper LLVM configuration
- **Duplicate user handling**: Graceful fallback to login when signup encounters existing user
- **Development environment inconsistencies**: Unified all tooling under single Makefile interface

### Changed
- **Development workflow**: Migrated from `scripts/setup-local-env.sh` to Makefile-based commands
- **Project structure**: Removed redundant setup script, consolidated into Makefile
- **Authentication flow**: Added environment detection for local vs production auth methods
- **Build system**: Optimized Cargo profiles and removed package-level profile conflicts

### Technical Details
- Enhanced `user_service.rs` with dual authentication paths (local/production)
- Removed duplicate Cargo profile definitions causing workspace warnings
- Added WASM-specific environment variables for cross-compilation
- Implemented Ed25519 key generation using `ring` crate for local development
- Firebase emulator configuration for ports 9099 (auth), 8081 (firestore), 4000 (UI)

### Developer Experience Improvements
- Setup time reduced from multi-step manual process to single `make start`
- Development iteration speed improved with incremental compilation
- Service debugging simplified with centralized log monitoring
- Clean development reset available with `make clean-dev`
- Tool dependency issues resolved with automatic installation

