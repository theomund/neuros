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

extern crate alloc;

mod font;
mod image;
mod interrupts;
mod intro;
mod logger;
mod memory;
mod serial;
mod shell;
mod smp;
mod vga;

use crate::logger::LOGGER;
use crate::shell::Shell;
use core::panic::PanicInfo;
use x86_64::instructions;

#[no_mangle]
extern "C" fn _start() -> ! {
    interrupts::initialize();
    memory::initialize();
    smp::initialize();
    intro::initialize().expect("Failed to initialize intro.");
    let mut shell = Shell::new();
    shell.display().expect("Failed to display shell.");
    info!("The operating system has been successfully initialized.");
    shell.interpret().expect("Failed to interpret shell input.");
    halt();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    error!("The kernel has panicked.");
    halt();
}

fn halt() -> ! {
    instructions::interrupts::disable();
    loop {
        instructions::hlt();
    }
}
