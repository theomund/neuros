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
use crate::{debug, error, halt, warn};
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
    idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
    idt.device_not_available
        .set_handler_fn(device_not_available_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    idt.invalid_tss.set_handler_fn(invalid_tss_handler);
    idt.segment_not_present
        .set_handler_fn(segment_not_present_handler);
    idt.stack_segment_fault
        .set_handler_fn(stack_segment_fault_handler);
    idt.general_protection_fault
        .set_handler_fn(general_protection_fault_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt.x87_floating_point
        .set_handler_fn(x87_floating_point_handler);
    idt.alignment_check.set_handler_fn(alignment_check_handler);
    idt.machine_check.set_handler_fn(machine_check_handler);
    idt.simd_floating_point
        .set_handler_fn(simd_floating_point_handler);
    idt.virtualization.set_handler_fn(virtualization_handler);
    idt.cp_protection_exception
        .set_handler_fn(control_protection_handler);
    idt.hv_injection_exception
        .set_handler_fn(hypervisor_injection_handler);
    idt.vmm_communication_exception
        .set_handler_fn(vmm_communication_handler);
    idt.security_exception.set_handler_fn(security_handler);
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

extern "x86-interrupt" fn invalid_opcode_handler(frame: InterruptStackFrame) {
    let log = format!("Invalid opcode exception was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn device_not_available_handler(frame: InterruptStackFrame) {
    let log = format!("Device not available exception was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, code: u64) -> ! {
    let log = format!("Double fault was thrown (code 0x{code:x}): {frame:?}");
    error!(log.as_str());
    halt();
}

extern "x86-interrupt" fn invalid_tss_handler(frame: InterruptStackFrame, code: u64) {
    let log = format!("Invalid TSS exception was thrown (code 0x{code:x}): {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn segment_not_present_handler(frame: InterruptStackFrame, code: u64) {
    let log = format!("Segment not present exception was thrown (code 0x{code:x}): {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn stack_segment_fault_handler(frame: InterruptStackFrame, code: u64) {
    let log = format!("Stack segment fault was thrown (code 0x{code:x}): {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn general_protection_fault_handler(frame: InterruptStackFrame, code: u64) {
    let log = format!("General protection fault was thrown (code 0x{code:x}): {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn page_fault_handler(frame: InterruptStackFrame, code: PageFaultErrorCode) {
    let log = format!(
        "Page fault was thrown (code 0x{:x}): {frame:?}",
        code.bits()
    );
    error!(log.as_str());
}

extern "x86-interrupt" fn x87_floating_point_handler(frame: InterruptStackFrame) {
    let log = format!("x87 floating point exception was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn alignment_check_handler(frame: InterruptStackFrame, code: u64) {
    let log = format!("Alignment check exception was thrown (code 0x{code:x}): {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn machine_check_handler(frame: InterruptStackFrame) -> ! {
    let log = format!("Machine check exception was thrown: {frame:?}");
    error!(log.as_str());
    halt();
}

extern "x86-interrupt" fn simd_floating_point_handler(frame: InterruptStackFrame) {
    let log = format!("SIMD floating point exception was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn virtualization_handler(frame: InterruptStackFrame) {
    let log = format!("Virtualization exception was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn control_protection_handler(frame: InterruptStackFrame, code: u64) {
    let log = format!("Control protection exception was thrown (code 0x{code:x}): {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn hypervisor_injection_handler(frame: InterruptStackFrame) {
    let log = format!("Hypervisor injection exception was thrown: {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn vmm_communication_handler(frame: InterruptStackFrame, code: u64) {
    let log = format!("VMM communication exception was thrown (code 0x{code:x}): {frame:?}");
    error!(log.as_str());
}

extern "x86-interrupt" fn security_handler(frame: InterruptStackFrame, code: u64) {
    let log = format!("Security exception was thrown (code 0x{code:x}): {frame:?}");
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
