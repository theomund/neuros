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

#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod font;
mod gdt;
mod idt;
mod image;
mod intro;
mod logger;
mod memory;
mod serial;
mod vga;

use core::panic::PanicInfo;
use logger::LOGGER;
use x86_64::instructions;

#[no_mangle]
extern "C" fn _start() -> ! {
    idt::initialize();
    memory::initialize();
    intro::initialize();
    halt();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    LOGGER.lock().error("The kernel has panicked.");
    halt();
}

fn halt() -> ! {
    instructions::interrupts::disable();
    loop {
        instructions::hlt();
    }
}
