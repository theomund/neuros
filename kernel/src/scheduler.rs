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
use crate::process::Process;
use crate::process::State;
use crate::{debug, trace};
use alloc::collections::VecDeque;
use alloc::format;
use spin::{Lazy, Mutex};

pub static SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| {
    let scheduler = Scheduler::new(100);
    Mutex::new(scheduler)
});

pub struct Scheduler {
    queue: VecDeque<Process>,
    quantum: u32,
    remaining: u32,
}

impl Scheduler {
    pub fn new(quantum: u32) -> Scheduler {
        let log = format!("Created process scheduler with a quantum of {quantum} ticks.");
        debug!(log.as_str());
        Scheduler {
            queue: VecDeque::new(),
            quantum,
            remaining: quantum,
        }
    }

    pub fn add(&mut self, process: Process) {
        self.queue.push_back(process);
    }

    pub fn tick(&mut self) {
        if self.remaining == 0 {
            let mut current = self.queue.pop_front().unwrap();
            current.set_state(State::Stopped);
            self.queue.push_back(current);
            let mut next = *self.queue.front().unwrap();
            next.set_state(State::Running);
            let log = format!(
                "Time slice for process #{} elapsed; switching to process #{}.",
                current.get_id(),
                next.get_id()
            );
            trace!(log.as_str());
            self.remaining = self.quantum;
        } else {
            self.remaining -= 1;
        }
    }
}

pub fn initialize() {
    let mut scheduler = SCHEDULER.lock();
    scheduler.add(Process::new(1, State::Running));
    scheduler.add(Process::new(2, State::Stopped));
}
