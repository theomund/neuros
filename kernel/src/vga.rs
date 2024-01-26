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

use crate::font::Font;
use crate::image::Image;
use limine::{Framebuffer, NonNullPtr};
use spin::Lazy;

static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new(0);
static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

pub static VGA: Lazy<Vga> = Lazy::new(|| {
    let font = Font::new();
    Vga::new(font)
});

#[derive(Clone, Copy)]
pub enum Color {
    Black = 0x0,
    Blue = 0x0097E6,
    Purple = 0x8C7AE6,
    Red = 0xE84118,
    White = 0xF5F6FA,
    Yellow = 0xFBC531,
}

pub struct Vga {
    font: Font,
    framebuffer: &'static NonNullPtr<Framebuffer>,
}

impl Vga {
    pub fn new(font: Font) -> Vga {
        assert!(BASE_REVISION.is_supported());
        if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
            if framebuffer_response.framebuffer_count < 1 {
                panic!("Failed to retrieve framebuffer.");
            }
            let framebuffer = &framebuffer_response.framebuffers()[0];
            Vga { font, framebuffer }
        } else {
            panic!("Failed to initialize VGA module.");
        }
    }

    pub fn draw_pixel(&self, x: usize, y: usize, color: Color) {
        let pixel_offset = y * self.framebuffer.pitch as usize + x * 4;
        let address = self
            .framebuffer
            .address
            .as_ptr()
            .unwrap()
            .wrapping_add(pixel_offset) as *mut u32;
        unsafe {
            *(address) = color as u32;
        }
    }

    pub fn draw_character(&self, character: char, x: usize, y: usize, fg: Color, bg: Color) {
        let masks = [128, 64, 32, 16, 8, 4, 2, 1];
        let position = character as usize * self.font.get_height();
        let glyphs = &self.font.get_data()[position..];

        for (cy, glyph) in glyphs.iter().enumerate().take(self.font.get_height()) {
            for (cx, mask) in masks.iter().enumerate().take(self.font.get_width()) {
                let color = if glyph & mask == 0 { bg } else { fg };
                self.draw_pixel(x + cx, y + cy - 12, color);
            }
        }
    }

    pub fn write(&self, message: &str, x: usize, y: usize, fg: Color, bg: Color) {
        for (position, character) in message.chars().enumerate() {
            self.draw_character(character, x + self.font.get_width() * position, y, fg, bg);
        }
    }

    pub fn draw_image(&self, image: Image, x: usize, y: usize) {
        let masks = [128, 64, 32, 16, 8, 4, 2, 1];
        let height = image.get_height();
        let byte_width = image.get_byte_width();
        let data = image.get_data();

        for (iy, row) in data.iter().enumerate().take(height) {
            for (ix, column) in row.iter().enumerate().take(byte_width) {
                for (mx, mask) in masks.iter().enumerate() {
                    let color = if column & mask == 0 {
                        Color::White
                    } else {
                        Color::Black
                    };
                    self.draw_pixel(x + (ix * masks.len()) + mx, y + iy, color);
                }
            }
        }
    }

    pub fn get_width(&self) -> usize {
        self.framebuffer.width as usize
    }

    pub fn get_height(&self) -> usize {
        self.framebuffer.height as usize
    }

    pub fn get_font_width(&self) -> usize {
        self.font.get_width()
    }

    pub fn get_font_height(&self) -> usize {
        self.font.get_height()
    }
}
