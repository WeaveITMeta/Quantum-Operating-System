# MACROHARD OS

<p align="center">
  <strong>🔷 MACROHARD 🔷</strong><br/>
  <em>A Next-Generation Operating System for the People</em>
</p>

---

## Overview

**MACROHARD** is a next-generation operating system forked from [Redox OS](https://www.redox-os.org)—a mature, Rust-based microkernel OS. MACROHARD aims to combine:

- **Security & Efficiency** of Redox (memory-safe Rust, microkernel architecture)
- **User-Friendliness** of Windows (seamless GUI, app integration)
- **Versatility** of Linux (modular, open-source, broad hardware support)

### Key Differentiators

1. **Built-in Hypervisor** - Run Windows/Linux guests natively with seamless integration
2. **AI-Assisted Development** - Embedded tooling for code scaffolding, fuzzing, and analysis
3. **Security-First Design** - Capability-based security, sandboxed processes, kernel ASLR
4. **Bloat-Free Computing** - Minimal, efficient, accessible for everyday users

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      MACROHARD OS Stack                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │  Desktop    │  │   Server    │  │   Hosted (VM)           │  │
│  │  Variant    │  │   Variant   │  │   Variant               │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│                    Userspace Services                           │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────────┐   │
│  │ Orbital  │ │ Ion Shell│ │ Drivers  │ │ Hypervisor (WIP) │   │
│  │ (GUI)    │ │          │ │ (User)   │ │ VT-x/AMD-V       │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                    Core Libraries                               │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────────┐   │
│  │ relibc   │ │ RedoxFS  │ │ TCP/IP   │ │ Capability Mgr   │   │
│  │ (POSIX)  │ │          │ │ Stack    │ │                  │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                    Rust Microkernel (~30K LoC)                  │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────────┐   │
│  │ IPC      │ │ Memory   │ │ Process  │ │ Syscall          │   │
│  │ (Msg)    │ │ Mgmt     │ │ Sched    │ │ Interface        │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                    Hardware Abstraction                         │
│  UEFI Boot │ ACPI │ PCIe │ NVMe │ USB │ Graphics │ Network     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Development Phases

### Phase 1: Fork & Setup (Months 1-3) ✅ IN PROGRESS
- [x] Fork Redox repository
- [ ] Rebrand to MACROHARD
- [ ] Set up enhanced CI/CD pipeline
- [ ] Configure cross-compilation (x86_64, ARM64, RISC-V)
- [ ] UEFI bootloader customization

### Phase 2: Core OS Architecture (Months 3-6)
- [ ] Enhance microkernel with syscall filtering
- [ ] Optimize process scheduler for SMP
- [ ] Implement kernel ASLR
- [ ] Extend filesystem abstraction layer
- [ ] Add GPT partitioning support

### Phase 3: Hardware & Drivers (Months 6-9)
- [ ] PCIe enumeration improvements
- [ ] NVMe/AHCI driver extensions
- [ ] USB HID stack enhancements
- [ ] Intel/AMD IOMMU support
- [ ] Power management (S3/S4)

### Phase 4: Hypervisor (Months 9-12)
- [ ] VT-x/VMX implementation
- [ ] AMD-V/SVM support
- [ ] EPT/NPT nested paging
- [ ] VM-exit handlers
- [ ] VirtIO device emulation

### Phase 5: Windows Integration (Months 12-15)
- [ ] Windows guest VM support
- [ ] GPU passthrough (VFIO)
- [ ] Shared folder protocol (9P/VirtIO-fs)
- [ ] Seamless window mode
- [ ] Guest agent service

### Phase 6+: Ongoing Development
- [ ] Networking stack improvements
- [ ] Security hardening
- [ ] AI-assisted tooling
- [ ] Community building

---

## Quick Start

### Prerequisites

- **Linux host** (recommended) or WSL2
- **Rust nightly** (via rustup)
- **QEMU** for testing
- **Podman** or Docker (optional, for containerized builds)

### Build

```bash
# Clone the repository
git clone https://github.com/macrohard-os/macrohard.git
cd macrohard

# Build the OS image
make all

# Run in QEMU
make qemu
```

### Configuration

Edit `config/desktop.toml` or create a custom configuration:

```toml
include = ["server.toml"]

[general]
filesystem_size = 650

[packages]
# Add your packages here
```

---

## Project Structure

```
macrohard/
├── config/           # Build configurations (desktop, server, minimal)
├── mk/               # Makefile includes (qemu, podman, disk, etc.)
├── recipes/          # Package recipes (2800+ packages)
├── scripts/          # Build and utility scripts
├── src/              # Cookbook build system source
├── podman/           # Container build files
├── rust/             # Rust toolchain (submodule)
└── bin/              # Cross-compilation helpers
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Key Areas for Contribution

1. **Hypervisor Development** - VT-x/AMD-V, VM management
2. **Driver Development** - Hardware support expansion
3. **Security Auditing** - Unsafe block review, fuzzing
4. **Documentation** - Tutorials, API docs
5. **Testing** - QEMU automation, hardware validation

---

## License

MACROHARD is licensed under the [MIT License](LICENSE), same as Redox OS.

---

## Acknowledgments

MACROHARD is built on the excellent foundation of [Redox OS](https://www.redox-os.org) and its community of 97+ contributors. Special thanks to Jeremy Soller (@jackpot51) and all Redox maintainers.

---

<p align="center">
  <strong>MACROHARD</strong> — Secure. Efficient. For the People.
</p>
