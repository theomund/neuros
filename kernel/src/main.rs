// NeurOS - Hobbyist operating system written in Rust.
// Copyright (C) 2023 Theomund
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

#![no_std]
#![no_main]

use core::arch::asm;

static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new(0);

#[no_mangle]
extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            hcf();
        }

        let framebuffer = &framebuffer_response.framebuffers()[0];

        for i in 0..100_usize {
            let pixel_offset = i * framebuffer.pitch as usize + i * 4;
            let address = framebuffer
                .address
                .as_ptr()
                .unwrap()
                .wrapping_add(pixel_offset) as *mut u32;
            unsafe {
                *(address) = 0xFFFFFFFF;
            }
        }
    }

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
