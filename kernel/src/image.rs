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

struct PbmHeader {
    magic: &'static str,
    width: usize,
    height: usize,
}

pub struct Pbm {
    header: PbmHeader,
    data: Vec<Vec<u8>>,
}

impl Pbm {
    pub fn new(path: &str) -> Pbm {
        let pbm = INITRD.get_data(path);
        let index = pbm
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == b'\n')
            .map(|(i, _)| i)
            .nth(1)
            .unwrap();
        let mut fields = str::from_utf8(&pbm[..index]).unwrap().split_whitespace();
        let header = PbmHeader {
            magic: fields.next().unwrap(),
            width: fields.next().unwrap().parse().unwrap(),
            height: fields.next().unwrap().parse().unwrap(),
        };
        assert_eq!(header.magic, "P4");
        let mut data = vec![vec![0u8; header.width / 8]; header.height];
        let pixels = &pbm[index + 1..];
        for y in 0..header.height {
            for x in 0..header.width / 8 {
                data[y][x] = pixels[(y * (header.width / 8)) + x];
            }
        }
        Pbm { header, data }
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

struct PgmHeader {
    magic: &'static str,
    width: usize,
    height: usize,
    max_value: u16,
}

pub struct Pgm {
    header: PgmHeader,
    data: Vec<Vec<u8>>,
}

impl Pgm {
    pub fn new(path: &str) -> Pgm {
        let pgm = INITRD.get_data(path);
        let index = pgm
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == b'\n')
            .map(|(i, _)| i)
            .nth(2)
            .unwrap();
        let mut fields = str::from_utf8(&pgm[..index]).unwrap().split_whitespace();
        let header = PgmHeader {
            magic: fields.next().unwrap(),
            width: fields.next().unwrap().parse().unwrap(),
            height: fields.next().unwrap().parse().unwrap(),
            max_value: fields.next().unwrap().parse().unwrap(),
        };
        assert_eq!(header.magic, "P5");
        assert_eq!(header.max_value, 255);
        let mut data = vec![vec![0u8; header.width]; header.height];
        let pixels = &pgm[index + 1..];
        for y in 0..header.height {
            for x in 0..header.width {
                data[y][x] = pixels[(y * header.width) + x];
            }
        }
        Pgm { header, data }
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
}

struct PpmHeader {
    magic: &'static str,
    width: usize,
    height: usize,
    max_value: u16,
}

#[derive(Clone)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    pub fn get_red(&self) -> u8 {
        self.red
    }

    pub fn get_green(&self) -> u8 {
        self.green
    }

    pub fn get_blue(&self) -> u8 {
        self.blue
    }
}

pub struct Ppm {
    header: PpmHeader,
    data: Vec<Vec<Pixel>>,
}

impl Ppm {
    pub fn new(path: &str) -> Ppm {
        let ppm = INITRD.get_data(path);
        let index = ppm
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == b'\n')
            .map(|(i, _)| i)
            .nth(2)
            .unwrap();
        let mut fields = str::from_utf8(&ppm[..index]).unwrap().split_whitespace();
        let header = PpmHeader {
            magic: fields.next().unwrap(),
            width: fields.next().unwrap().parse().unwrap(),
            height: fields.next().unwrap().parse().unwrap(),
            max_value: fields.next().unwrap().parse().unwrap(),
        };
        assert_eq!(header.magic, "P6");
        assert_eq!(header.max_value, 255);
        let mut data = vec![
            vec![
                Pixel {
                    red: 0,
                    green: 0,
                    blue: 0
                };
                header.width
            ];
            header.height
        ];
        let pixels = &ppm[index + 1..];
        for y in 0..header.height {
            for x in 0..header.width {
                let pixel = Pixel {
                    red: pixels[(y * header.width * 3) + (3 * x)],
                    green: pixels[(y * header.width * 3) + (3 * x + 1)],
                    blue: pixels[(y * header.width * 3) + (3 * x + 2)],
                };
                data[y][x] = pixel;
            }
        }
        Ppm { header, data }
    }

    pub fn get_data(&self) -> &Vec<Vec<Pixel>> {
        &self.data
    }

    pub fn get_width(&self) -> usize {
        self.header.width
    }

    pub fn get_height(&self) -> usize {
        self.header.height
    }
}
