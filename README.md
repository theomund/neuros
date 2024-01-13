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
![Channel](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FTheomund%2FNeurOS%2Fmain%2Fkernel%2Frust-toolchain.toml&query=toolchain.channel&style=for-the-badge&logo=rust&label=toolchain)

> [!WARNING]
> This project is currently in its early development stage. As a result, you
> might find bugs, incomplete features, and unexpected behavior. We appreciate
> your interest and encourage you to offer feedback, but be aware that the
> system is not yet stable for production use. Use it at your own discretion,
> and prepare for potential issues.

## Overview

This project aims to create a lightweight, modular, and extensible operating
system based on the microkernel architecture. The microkernel design minimizes
the kernel's complexity by delegating most tasks to userspace services,
resulting in improved reliability, security, and maintainability.

## Architecture

```mermaid
graph BT
    subgraph Userspace
        style Userspace fill: #2980b9
        subgraph Servers
            direction BT
            style Servers fill: #2980b9
            app_ipc(Application IPC)
            device(Device Driver)
            file(File Server)
            unix(UNIX Server)
        end
        Application
    end
    subgraph Kernel
        direction BT
        style Kernel fill: #c0392b
        basic_ipc(Basic IPC)
        memory(Virtual Memory)
        scheduler(Scheduling)
    end
    Servers <--> Application
    Kernel <--> Servers
```

## License

![GPL Logo](https://www.gnu.org/graphics/gplv3-with-text-136x68.png)

This project uses the [GNU General Public License v3.0](COPYING) (or later).

You can find the detailed terms of the license in the [COPYING](COPYING) file.
