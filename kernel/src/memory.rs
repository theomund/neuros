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

use crate::debug;
use crate::logger::LOGGER;
use alloc::format;
use limine::memory_map::EntryType;
use limine::request::{MemoryMapRequest, StackSizeRequest};
use linked_list_allocator::LockedHeap;

static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(0x32000);
static MEMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn initialize() {
    STACK_SIZE_REQUEST.get_response();
    let entries = MEMMAP_REQUEST.get_response().unwrap().entries();
    let entry = entries
        .iter()
        .find(|x| x.entry_type == EntryType::USABLE)
        .unwrap();
    let start = entry.base;
    let size = entry.length as usize;
    unsafe {
        ALLOCATOR.lock().init(start as *mut u8, size);
    }
    let message = format!(
        "Detected usable memory region of size {} bytes at 0x{:x}.",
        size, start
    );
    debug!(message.as_str());
}
