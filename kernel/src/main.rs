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

#![warn(clippy::pedantic)]
#![feature(abi_x86_interrupt)]
#![feature(int_roundings)]
#![no_std]
#![no_main]

extern crate alloc;

mod ansi;
mod elf;
mod font;
mod gdt;
mod image;
mod initrd;
mod interrupts;
mod intro;
mod keyboard;
mod logger;
mod memory;
mod process;
mod scheduler;
mod serial;
mod shell;
mod smp;
mod syscall;
mod timer;
mod vga;

use crate::logger::Logger;
use alloc::format;
use core::panic::PanicInfo;
use x86_64::instructions;

#[no_mangle]
extern "C" fn _start() -> ! {
    gdt::initialize();
    memory::initialize();
    interrupts::initialize();
    smp::initialize();
    initrd::initialize();
    scheduler::initialize();
    intro::initialize().expect("Failed to initialize intro.");
    shell::initialize();
    tick();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let log = format!("The kernel has panicked.\n\n{info}");
    fatal!(log.as_str());
    halt();
}

fn tick() -> ! {
    instructions::interrupts::enable();
    loop {
        instructions::hlt();
    }
}

fn halt() -> ! {
    instructions::interrupts::disable();
    loop {
        instructions::hlt();
    }
}
