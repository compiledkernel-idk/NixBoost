# Changelog

## [1.2.0] - 2025-12-24
### Improved
- **Batch Operations**: Major performance boost by grouping package installations and removals into single transactions instead of sequential loops.
- **Architecture Support**: Removed hardcoded `x86_64-linux` dependency. `nixboost` now dynamically detects system architecture (e.g., `aarch64-darwin`), making it compatible with Apple Silicon and ARM devices.
- **Async I/O**: Refactored core package manager logic to use asynchronous process execution for better responsiveness.
- **User Interface**: Replaced basic text prompts with interactive `dialoguer` menus for cleaner confirmations.

## [1.1.0] - 2025-12-23
### Added
- **NixOS Support**: Complete pivot from Arch Linux to NixOS.
- **NUR Support**: Added search and installation support for the Nix User Repository.
- **NixManager**: New backend using `nix search` and `nix-env`.
- **NixOS News**: Added news reader to fetch from the NixOS blog.
- **Generation History**: Added history viewer to show Nix generations.
- **Nix Health Check**: Added health check for Nix store integrity.
- **Garbage Collection**: Added support for `nix-collect-garbage`.

## [1.0.0] - 2025-12-23
### Added
- Initial release of `nixboost`.
