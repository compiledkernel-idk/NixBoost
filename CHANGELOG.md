# Changelog

## [1.1.0] - 2025-12-23
### Added
- **NixOS Support**: Complete pivot from Arch Linux to NixOS.
- **NUR Support**: Added search and installation support for the Nix User Repository.
- **NixManager**: New backend using `nix search` and `nix-env`.
- **NixOS News**: Added news reader to fetch from the NixOS blog.
- **Generation History**: Added history viewer to show Nix generations.
- **Nix Health Check**: Added health check for Nix store integrity.
- **Garbage Collection**: Added support for `nix-collect-garbage`.

### Removed
- **Arch Linux Support**: Removed `libalpm` dependency and all pacman-specific logic.

## [1.0.0] - 2025-12-23
### Added
- Initial release of `nixboost`.
