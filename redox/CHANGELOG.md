# Changelog

All notable changes to MACROHARD OS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial fork from Redox OS 0.9.0
- MACROHARD branding and documentation
- New build configurations:
  - `macrohard-desktop` - Full desktop with hypervisor support
  - `macrohard-server` - Headless server variant
  - `macrohard-minimal` - Minimal testing configuration
- Enhanced CI/CD pipeline with GitHub Actions
- Windows development support via PowerShell scripts
- Comprehensive documentation:
  - `MACROHARD.md` - Project overview
  - `ROADMAP.md` - Development roadmap
  - `ARCHITECTURE.md` - System architecture
  - `docs/GETTING_STARTED.md` - Quick start guide
  - `docs/DRIVERS.md` - Driver development guide
  - `docs/SECURITY.md` - Security model
  - `docs/HYPERVISOR.md` - Hypervisor design
- Recipe stubs for future MACROHARD packages:
  - `macrohard-hypervisor` - Built-in hypervisor
  - `macrohard-security` - Security daemon
  - `macrohard-windows-bridge` - Windows integration
  - `macrohard-ai-tools` - AI-assisted development
  - `macrohard-core` - Core utilities

### Changed
- Updated os-release to reflect MACROHARD branding
- Default hostname changed to "macrohard"
- Increased default filesystem size for desktop (2GB)

### Planned (Phase 1)
- [ ] UEFI bootloader customization
- [ ] Cross-compilation for ARM64 and RISC-V
- [ ] Reproducible builds

---

## [0.9.0] - Redox OS Base

This release represents the Redox OS 0.9.0 base that MACROHARD is forked from.

See [Redox OS Changelog](https://www.redox-os.org/news/) for upstream changes.
