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
    width: usize,
    height: usize,
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

pub struct Image {
    header: Header,
    data: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn new(path: &str) -> Image {
        let image = INITRD.get_data(path);
        let mut newlines = image
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == b'\n')
            .map(|(i, _)| i);
        let extension = path.split_terminator('.').last().unwrap();
        let index = match extension {
            "pbm" => newlines.nth(1).unwrap(),
            "pgm" | "ppm" => newlines.nth(2).unwrap(),
            _ => panic!("Unsupported image file type detected."),
        };
        let mut fields = str::from_utf8(&image[..index]).unwrap().split_whitespace();
        let header = Header {
            width: fields.nth(1).unwrap().parse().unwrap(),
            height: fields.next().unwrap().parse().unwrap(),
        };
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
        let pixels = &image[index + 1..];
        let masks = [128, 64, 32, 16, 8, 4, 2, 1];
        for y in 0..header.height {
            for x in 0..header.width {
                data[y][x] = match extension {
                    "pbm" => {
                        let pixel = pixels[(y * (header.width / 8)) + x / 8];
                        let mask = masks[x % 8];
                        let value = if pixel & mask == 0 { 0xFF } else { 0x00 };
                        Pixel {
                            red: value,
                            green: value,
                            blue: value,
                        }
                    }
                    "pgm" => {
                        let value = pixels[(y * header.width) + x];
                        Pixel {
                            red: value,
                            green: value,
                            blue: value,
                        }
                    }
                    "ppm" => Pixel {
                        red: pixels[(y * header.width * 3) + (3 * x)],
                        green: pixels[(y * header.width * 3) + (3 * x + 1)],
                        blue: pixels[(y * header.width * 3) + (3 * x + 2)],
                    },
                    _ => panic!("Unsupported image file type detected."),
                }
            }
        }
        Image { header, data }
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
