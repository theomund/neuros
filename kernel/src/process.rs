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

#[derive(Copy, Clone)]
pub enum State {
    Running,
    Stopped,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Context {
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rbp: u64,
    rbx: u64,
    rsp: u64,
}

#[derive(Copy, Clone)]
pub struct Process {
    id: u32,
    context: Context,
    name: &'static str,
    state: State,
}

impl Process {
    pub fn new(id: u32, name: &str, state: State) -> Process {
        Process {
            id,
            context: Context {
                r12: 0,
                r13: 0,
                r14: 0,
                r15: 0,
                rbp: 0,
                rbx: 0,
                rsp: 0,
            },
            name,
            state,
        }
    }

    pub fn get_id(self) -> u32 {
        self.id
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
}
