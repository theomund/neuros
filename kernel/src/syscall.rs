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

use crate::scheduler::SCHEDULER;

pub fn close() {
    todo!("Implement system call.");
}

pub fn exec() {
    todo!("Implement system call.");
}

pub fn exit() {
    todo!("Implement system call.");
}

pub fn fork() -> u64 {
    SCHEDULER.lock().fork()
}

pub fn kill() {
    todo!("Implement system call.");
}

pub fn open() {
    todo!("Implement system call.");
}

pub fn read() {
    todo!("Implement system call.");
}

pub fn wait() {
    todo!("Implement system call.");
}

pub fn write() {
    todo!("Implement system call.");
}
