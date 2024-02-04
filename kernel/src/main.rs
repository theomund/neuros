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

mod font;
mod image;
mod initrd;
mod interrupts;
mod intro;
mod logger;
mod memory;
mod serial;
mod shell;
mod smp;
mod timer;
mod vga;

use crate::initrd::INITRD;
use crate::logger::LOGGER;
use crate::shell::Shell;
use alloc::format;
use core::panic::PanicInfo;
use x86_64::instructions;

#[no_mangle]
extern "C" fn _start() -> ! {
    memory::initialize();
    interrupts::initialize();
    smp::initialize();
    let log = format!(
        "Loaded {} files from the initial ramdisk.",
        INITRD.get_files().len()
    );
    debug!(log.as_str());
    intro::initialize().expect("Failed to initialize intro.");
    let mut shell = Shell::new();
    Shell::display().expect("Failed to display shell.");
    info!("The operating system has been successfully initialized.");
    instructions::interrupts::enable();
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
