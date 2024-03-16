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
use crate::{debug, trace, warn};
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
            if self.queue.len() >= 2 {
                self.schedule();
            } else {
                warn!("Queue doesn't have enough processes to swap.");
            }
            self.remaining = self.quantum;
        } else {
            self.remaining -= 1;
        }
    }

    fn schedule(&mut self) {
        let current = self.queue.pop_front().unwrap();
        self.queue.push_back(current);
        if let Some(back) = self.queue.back_mut() {
            back.set_state(State::Stopped);
            let log = format!("Stopped process #{} ({}).", back.get_id(), back.get_name());
            trace!(log.as_str());
        }
        if let Some(front) = self.queue.front_mut() {
            front.set_state(State::Running);
            let log = format!(
                "Started process #{} ({}).",
                front.get_id(),
                front.get_name()
            );
            trace!(log.as_str());
        }
    }

    pub fn fork(&mut self) -> u64 {
        let parent = self.queue.front().unwrap();
        let mut child = parent.clone();
        child.set_id((self.queue.len() + 1) as u64);
        let pid = child.get_id();
        self.add(child);
        pid
    }
}

pub fn initialize() {
    SCHEDULER
        .lock()
        .add(Process::new(1, "kernel", State::Running));
}
