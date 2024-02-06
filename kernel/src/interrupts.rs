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

use crate::keyboard::ScanCode::Enter;
use crate::keyboard::KEYBOARD;
use crate::logger::LOGGER;
use crate::scheduler::SCHEDULER;
use crate::timer::TIMER;
use crate::vga::{Color, VGA};
use crate::warn;
use core::fmt::Write;
use pic8259::ChainedPics;
use spin::{Lazy, Mutex};
use x86_64::instructions;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[repr(u8)]
enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
    idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_handler);
    idt
});

static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    warn!("Breakpoint exception was thrown.");
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    let mut timer = TIMER.lock();
    let elapsed = timer.get_elapsed();
    timer.set_elapsed(elapsed + 1);

    SCHEDULER.lock().tick();

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    if KEYBOARD.lock().read() == Enter as u8 {
        let mut vga = VGA.lock();
        let x = vga.get_font_width();
        let y = vga.get_font_height() + x;
        vga.clear();
        vga.set_cursor(x, y, Color::Yellow as u32, Color::Black as u32);
        write!(vga, "Hello, world!").expect("Failed to write message.");
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}

pub fn initialize() {
    IDT.load();
    let mut pics = PICS.lock();
    unsafe {
        pics.initialize();
        pics.write_masks(0b1111_1100, 0b1111_1111);
    }
    instructions::interrupts::enable();
}
