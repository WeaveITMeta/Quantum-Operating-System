# MACROHARD OS - Quick Start Guide

## Prerequisites

- **Linux/WSL2**: Required for building
- **Rust nightly**: `rustup default nightly`
- **QEMU**: For testing

## Build Commands

```bash
# Build MACROHARD Desktop
make all CONFIG_NAME=macrohard-desktop

# Build MACROHARD Server
make all CONFIG_NAME=macrohard-server

# Build MACROHARD Minimal
make all CONFIG_NAME=macrohard-minimal

# Run in QEMU
make qemu
```

## Windows Development

```powershell
# Setup WSL2 environment
.\scripts\macrohard-dev.ps1 setup

# Build
.\scripts\macrohard-dev.ps1 all

# Run in QEMU
.\scripts\macrohard-dev.ps1 qemu
```

## Linux/macOS Development

```bash
# Build
./scripts/macrohard-build.sh all

# Run in QEMU
./scripts/macrohard-build.sh qemu
```

## Ports

| Service | Port |
|---------|------|
| SSH (QEMU forward) | 2222 |
| HTTP (if enabled) | 8080 |

## Documentation

- [MACROHARD.md](MACROHARD.md) - Project overview
- [ROADMAP.md](ROADMAP.md) - Development roadmap
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md) - Detailed guide
- [docs/DRIVERS.md](docs/DRIVERS.md) - Driver development
- [docs/SECURITY.md](docs/SECURITY.md) - Security model
- [docs/HYPERVISOR.md](docs/HYPERVISOR.md) - Hypervisor design
