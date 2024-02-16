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
}

pub struct Elf {
    header: Header,
}

impl Elf {
    pub fn new(path: &str) -> Elf {
        let data = INITRD.get_data(path);
        let header = Header {
            magic: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
        };
        Elf { header }
    }
}
