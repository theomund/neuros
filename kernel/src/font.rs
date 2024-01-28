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

pub struct Font {
    header: Header,
    data: &'static [u8],
}

struct Header {
    magic: u16,
    font_mode: u8,
    glyph_size: u8,
}

impl Font {
    pub fn new() -> Font {
        let psf = include_bytes!("resource/ter-i16n.psf");
        let header = Header {
            magic: u16::from_le_bytes([psf[0], psf[1]]),
            font_mode: psf[2],
            glyph_size: psf[3],
        };
        let data = &psf[4..4100];
        assert_eq!(header.magic, u16::from_le_bytes([0x36, 0x04]));
        assert_eq!(header.font_mode, 0x02);
        assert_eq!(header.glyph_size, 16);
        Font { header, data }
    }

    pub fn get_data(&self) -> &[u8] {
        self.data
    }

    pub fn get_width(&self) -> usize {
        (self.header.glyph_size / 2) as usize
    }

    pub fn get_height(&self) -> usize {
        self.header.glyph_size as usize
    }
}
