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

pub struct VGA {
    font: Font,
    framebuffer: &'static NonNullPtr<Framebuffer>,
}

impl VGA {
    pub fn new() -> VGA {
        assert!(BASE_REVISION.is_supported());
        if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
            if framebuffer_response.framebuffer_count < 1 {
                panic!("Failed to retrieve framebuffer.");
            }
            let font = Font::new();
            let framebuffer = &framebuffer_response.framebuffers()[0];
            VGA { font, framebuffer }
        } else {
            panic!("Failed to initialize VGA module.");
        }
    }

    pub fn draw(&self) {
        for i in 0..100_usize {
            let pixel_offset = i * self.framebuffer.pitch as usize + i * 4;
            let address = self
                .framebuffer
                .address
                .as_ptr()
                .unwrap()
                .wrapping_add(pixel_offset) as *mut u32;
            unsafe {
                *(address) = 0xFFFFFFFF;
            }
        }
    }
}
