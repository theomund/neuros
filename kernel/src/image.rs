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

const WIDTH: usize = 1024;
const HEIGHT: usize = 254;
const BYTE_WIDTH: usize = WIDTH / 8;

struct Header {
    magic: u16,
    width: usize,
    height: usize,
}

pub struct Image {
    header: Header,
    data: [[u8; BYTE_WIDTH]; HEIGHT],
}

impl Image {
    pub fn new() -> Image {
        let pbm = include_bytes!("resource/logo.pbm");
        let magic = u16::from_be_bytes([pbm[0], pbm[1]]);
        let width = WIDTH;
        let height = HEIGHT;
        let header = Header {
            magic,
            width,
            height,
        };
        assert_eq!(header.magic, 0x5034);
        let mut data = [[0u8; BYTE_WIDTH]; HEIGHT];
        let pixels = &pbm[12..];
        for y in 0..HEIGHT {
            for x in 0..BYTE_WIDTH {
                data[y][x] = pixels[(y * BYTE_WIDTH) + x];
            }
        }
        Image { header, data }
    }

    pub fn get_data(&self) -> [[u8; BYTE_WIDTH]; HEIGHT] {
        self.data
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
