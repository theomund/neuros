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

use crate::logger::LOGGER;
use crate::timer::TIMER;
use crate::vga::{Color, VGA};
use crate::warn;
use core::fmt::Write;
use pic8259::ChainedPics;
use spin::{Lazy, Mutex};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[repr(u8)]
enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
    idt
});

static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn initialize() {
    IDT.load();
    unsafe {
        PICS.lock().initialize();
        PICS.lock().write_masks(0b11111110, 0b11111111);
    }
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    warn!("Breakpoint exception was thrown.");
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    let mut timer = TIMER.lock();
    let elapsed = timer.get_elapsed();
    let clamp = elapsed.clamp(0, 0xFF);
    timer.set_elapsed(elapsed + 1);

    let mut vga = VGA.lock();
    let width = vga.get_width();
    let height = vga.get_height();
    let font_width = vga.get_font_width();

    let instruction = "Press ENTER to continue.";
    vga.set_cursor(
        (width / 2) - (instruction.len() * font_width / 2),
        height - (height / 4),
        clamp << 16 | clamp << 8 | clamp,
        Color::Black as u32,
    );
    write!(vga, "{}", instruction).expect("Failed to write instruction.");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}
