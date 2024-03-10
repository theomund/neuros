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

use crate::keyboard::KEYBOARD;
use crate::logger::LOGGER;
use crate::scheduler::SCHEDULER;
use crate::timer::TIMER;
use crate::{debug, error, warn};
use alloc::format;
use pic8259::ChainedPics;
use spin::{Lazy, Mutex};
use x86_64::instructions;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[repr(u8)]
enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.divide_error.set_handler_fn(divide_error_handler);
    idt.debug.set_handler_fn(debug_handler);
    idt.non_maskable_interrupt.set_handler_fn(nmi_handler);
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.overflow.set_handler_fn(overflow_handler);
    idt.bound_range_exceeded.set_handler_fn(bound_range_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt[InterruptIndex::Timer as u8].set_handler_fn(timer_handler);
    idt[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard_handler);
    idt
});

static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

extern "x86-interrupt" fn divide_error_handler(frame: InterruptStackFrame) {
    let log = format!("Division error was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn debug_handler(frame: InterruptStackFrame) {
    let log = format!("Debug exception was thrown: {frame:?}");
    debug!(log.as_str());
}

extern "x86-interrupt" fn nmi_handler(frame: InterruptStackFrame) {
    let log = format!("Non-Maskable Interrupt (NMI) was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    let log = format!("Breakpoint exception was thrown: {frame:?}");
    warn!(log.as_str());
}

extern "x86-interrupt" fn overflow_handler(frame: InterruptStackFrame) {
    let log = format!("Overflow exception was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn bound_range_handler(frame: InterruptStackFrame) {
    let log = format!("Bound range exceeded exception was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn page_fault_handler(frame: InterruptStackFrame, code: PageFaultErrorCode) {
    let log = format!(
        "Page fault was thrown (code 0x{:x}): {frame:?}",
        code.bits()
    );
    error!(log.as_str());
}

extern "x86-interrupt" fn timer_handler(_frame: InterruptStackFrame) {
    TIMER.increment();
    SCHEDULER.lock().tick();

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard_handler(_frame: InterruptStackFrame) {
    KEYBOARD.lock().interpret();

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
