// NeurOS - Hobbyist operating system written in Rust.
// Copyright (C) 2024 Theomund
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::initrd::INITRD;
use core::fmt::{Display, Formatter};

struct Header {
    magic: u32,
    class: u8,
    data: u8,
    version: u8,
    os_abi: u8,
    abi_version: u8,
    file_type: u16,
    machine: u16,
    exec_version: u32,
    entrypoint: u64,
    ph_offset: u64,
    sh_offset: u64,
    flags: u32,
    eh_size: u16,
    ph_size: u16,
    ph_number: u16,
    sh_size: u16,
    sh_number: u16,
    sh_index: u16,
}

pub struct Elf {
    header: Header,
}

impl Display for Elf {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "ELF Header:")?;
        writeln!(f, "  Magic: 0x{:x}", self.header.magic)?;
        let class = match self.header.class {
            1 => "ELF32",
            2 => "ELF64",
            _ => "Unknown",
        };
        writeln!(f, "  Class: {class}")?;
        let data = match self.header.data {
            1 => "2's complement, little endian",
            2 => "2's complement, big endian",
            _ => "Unknown",
        };
        writeln!(f, "  Data: {data}")?;
        let version = match self.header.version {
            1 => "1 (current)",
            _ => "Unknown",
        };
        writeln!(f, "  Version: {version}")?;
        let os_abi = match self.header.os_abi {
            0 => "UNIX - System V",
            1 => "UNIX - HP-UX",
            2 => "UNIX - NetBSD",
            3 => "UNIX - Linux",
            4 => "UNIX - GNU Hurd",
            6 => "UNIX - Solaris",
            7 => "UNIX - AIX",
            8 => "UNIX - IRIX",
            9 => "UNIX - FreeBSD",
            10 => "UNIX - TRU64",
            11 => "Novell - Modesto",
            12 => "UNIX - OpenBSD",
            13 => "VMS - OpenVMS",
            14 => "HP - Non-Stop Kernel",
            15 => "AROS",
            16 => "FenixOS",
            17 => "Nuxi CloudABI",
            18 => "Stratus Technologies OpenVOS",
            _ => "Unknown",
        };
        writeln!(f, "  OS/ABI: {os_abi}")?;
        writeln!(f, "  ABI Version: {}", self.header.abi_version)?;
        let file_type = match self.header.file_type {
            0 => "NONE (None)",
            1 => "REL (Relocatable file)",
            2 => "EXEC (Executable file)",
            3 => "DYN (Position-Independent Executable file)",
            4 => "CORE (Core file)",
            _ => "Unknown",
        };
        writeln!(f, "  Type: {file_type}")?;
        let machine = match self.header.machine {
            0x3E => "Advanced Micro Devices X86-64",
            _ => "Unknown",
        };
        writeln!(f, "  Machine: {machine}")?;
        writeln!(f, "  Version: 0x{:x}", self.header.exec_version)?;
        writeln!(f, "  Entry point address: 0x{:x}", self.header.entrypoint)?;
        writeln!(
            f,
            "  Start of program headers: {} (bytes into file)",
            self.header.ph_offset
        )?;
        writeln!(
            f,
            "  Start of section headers: {} (bytes into file)",
            self.header.sh_offset
        )?;
        writeln!(f, "  Flags: 0x{:x}", self.header.flags)?;
        writeln!(f, "  Size of this header: {} (bytes)", self.header.eh_size)?;
        writeln!(
            f,
            "  Size of program headers: {} (bytes)",
            self.header.ph_size
        )?;
        writeln!(f, "  Number of program headers: {}", self.header.ph_number)?;
        writeln!(
            f,
            "  Size of section headers: {} (bytes)",
            self.header.sh_size
        )?;
        writeln!(f, "  Number of section headers: {}", self.header.sh_number)?;
        writeln!(
            f,
            "  Section header string table index: {}",
            self.header.sh_index
        )?;
        Ok(())
    }
}

impl Elf {
    pub fn new(path: &str) -> Elf {
        let data = INITRD.get_data(path);
        let header = Header {
            magic: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
            class: data[4],
            data: data[5],
            version: data[6],
            os_abi: data[7],
            abi_version: data[8],
            file_type: u16::from_le_bytes([data[16], data[17]]),
            machine: u16::from_le_bytes([data[18], data[19]]),
            exec_version: u32::from_le_bytes([data[20], data[21], data[22], data[23]]),
            entrypoint: u64::from_le_bytes([
                data[24], data[25], data[26], data[27], data[28], data[29], data[30], data[31],
            ]),
            ph_offset: u64::from_le_bytes([
                data[32], data[33], data[34], data[35], data[36], data[37], data[38], data[39],
            ]),
            sh_offset: u64::from_le_bytes([
                data[40], data[41], data[42], data[43], data[44], data[45], data[46], data[47],
            ]),
            flags: u32::from_le_bytes([data[48], data[49], data[50], data[51]]),
            eh_size: u16::from_le_bytes([data[52], data[53]]),
            ph_size: u16::from_le_bytes([data[54], data[55]]),
            ph_number: u16::from_le_bytes([data[56], data[57]]),
            sh_size: u16::from_le_bytes([data[58], data[59]]),
            sh_number: u16::from_le_bytes([data[60], data[61]]),
            sh_index: u16::from_le_bytes([data[62], data[63]]),
        };
        Elf { header }
    }
}
