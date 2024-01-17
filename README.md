<!--
  NeurOS - Hobbyist operating system written in Rust.
  Copyright (C) 2024 Theomund

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program. If not, see <https://www.gnu.org/licenses/>.
-->

# ![Project Logo](https://github.com/Theomund/NeurOS/assets/34360334/7b2e911a-429b-4d44-a4bb-131a8037cb97)

![GitHub License](https://img.shields.io/github/license/Theomund/NeurOS?style=for-the-badge)
![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FTheomund%2FNeurOS%2Fmain%2Fkernel%2FCargo.toml&query=package.version&style=for-the-badge&label=version)
![Channel](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FTheomund%2FNeurOS%2Fmain%2Frust-toolchain.toml&query=toolchain.channel&style=for-the-badge&logo=rust&label=toolchain)

> [!WARNING]
> This project is currently in its early development stage. As a result, you
> might find bugs, incomplete features, and unexpected behavior. Be aware that
> the system is not yet stable for production use. Use it at your own
> discretion, and prepare for potential issues.

# Overview

This project aims to create a lightweight, modular, and extensible operating
system based on the microkernel architecture. The microkernel design minimizes
the kernel's complexity by delegating most tasks to userspace services,
resulting in improved reliability, security, and maintainability.

# Architecture

```mermaid
graph BT
    classDef node font-size:24px,padding:64px,color:#f5f6fa,stroke-width:0px
    style Application fill:#0097e6
    style Drivers fill:#8c7ae6
    style Hardware fill:#e1b12c
    style Kernel fill:#c23616
    style Servers fill:#44bd32
    style Userspace color:#f5f6fa,fill:#2f3640
    Hardware <==> Kernel
    Kernel <==> Drivers
    Kernel <==> Servers
    subgraph Userspace
        Drivers <==> Application
        Servers <==> Application
    end
```

# Building

> [!NOTE]
> Building on Windows or macOS is currently not supported at this time.

To build and compile the operating system from source, follow the outlined
steps. Make sure you have the necessary tools and dependencies installed on
your system.

## Prerequisites

Before building the operating system, ensure you have the following prerequisites installed:

* `edk2-ovmf`
* `gcc`
* `make`
* `qemu`
* `rustup`
* `xorriso`

## Cloning

Clone the repository to your local machine by using the following command:

```bash
git clone https://github.com/Theomund/NeurOS.git
```

## Compiling

Run the following command to generate an ISO image:

```bash
make all
```

This command will start the build process based on the configured settings
and source code.

## Testing

To test the compiled operating system in a virtual environment, run this
command:

```bash
make run
```

This command will start QEMU and run the operating system in BIOS mode. To
switch the mode to UEFI, run the following command:

```bash
make run-uefi
```

## Cleaning

To clean the build artifacts and start fresh, use the following command:

```bash
make clean 
```

This command removes compiled binaries and temporary files.

# License

![GPL Logo](https://www.gnu.org/graphics/gplv3-with-text-136x68.png)

This project uses the [GNU General Public License v3.0](COPYING) (or later).

You can find the detailed terms of the license in the [COPYING](COPYING) file.
