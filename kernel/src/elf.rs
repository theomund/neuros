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
    header_size: u16,
    ph_size: u16,
    ph_number: u16,
    sh_size: u16,
    sh_number: u16,
    sh_index: u16,
}

pub struct Elf {
    header: Header,
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
            header_size: u16::from_le_bytes([data[52], data[53]]),
            ph_size: u16::from_le_bytes([data[54], data[55]]),
            ph_number: u16::from_le_bytes([data[56], data[57]]),
            sh_size: u16::from_le_bytes([data[58], data[59]]),
            sh_number: u16::from_le_bytes([data[60], data[61]]),
            sh_index: u16::from_le_bytes([data[62], data[63]]),
        };
        Elf { header }
    }
}
