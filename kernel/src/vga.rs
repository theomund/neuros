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
use limine::{Framebuffer, NonNullPtr};

static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new(0);
static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

#[derive(Clone, Copy)]
pub enum Color {
    Black = 0x0,
    Yellow = 0xFFFF00,
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
        let mask = [128, 64, 32, 16, 8, 4, 2, 1];
        let position = character as usize * 16;
        let glyph = &self.font.get_data()[position..];

        for cy in 0..16 {
            for cx in 0..8 {
                let color = if glyph[cy] & mask[cx] == 0 { bg } else { fg };
                self.draw_pixel(x + cx, y + cy - 12, color);
            }
        }
    }

    pub fn write(&self, message: &str, x: usize, y: usize) {
        for (position, character) in message.chars().enumerate() {
            self.draw_character(character, x + 8 * position, y, Color::Yellow, Color::Black);
        }
    }

    pub fn get_height(&self) -> usize {
        self.framebuffer.height as usize
    }

    pub fn get_width(&self) -> usize {
        self.framebuffer.width as usize
    }
}
