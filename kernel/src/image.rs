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
use alloc::vec;
use alloc::vec::Vec;
use core::str;

struct Header {
    magic: &'static str,
    width: usize,
    height: usize,
}

pub struct Image {
    header: Header,
    data: Vec<Vec<u8>>,
}

impl Image {
    pub fn new(path: &str) -> Image {
        let pbm = INITRD.get_data(path);
        let end = pbm.iter().rposition(|x| *x == b'\n').unwrap();
        let mut fields = str::from_utf8(&pbm[..end]).unwrap().split_whitespace();
        let header = Header {
            magic: fields.next().unwrap(),
            width: fields.next().unwrap().parse().unwrap(),
            height: fields.next().unwrap().parse().unwrap(),
        };
        assert_eq!(header.magic, "P4");
        let mut data = vec![vec![0u8; header.width / 8]; header.height];
        let pixels = &pbm[end + 1..];
        for y in 0..header.height {
            for x in 0..header.width / 8 {
                data[y][x] = pixels[(y * (header.width / 8)) + x];
            }
        }
        Image { header, data }
    }

    pub fn get_data(&self) -> &Vec<Vec<u8>> {
        &self.data
    }

    pub fn get_width(&self) -> usize {
        self.header.width
    }

    pub fn get_height(&self) -> usize {
        self.header.height
    }

    pub fn get_byte_width(&self) -> usize {
        self.get_width() / 8
    }
}
