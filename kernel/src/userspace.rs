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

use crate::gdt;
use x86_64::registers::control::{Efer, EferFlags};
use x86_64::registers::model_specific::Star;

pub fn initialize() {
    Star::write(
        gdt::get_user_code(),
        gdt::get_user_data(),
        gdt::get_kernel_code(),
        gdt::get_kernel_data(),
    )
    .expect("Failed to write to STAR register");

    unsafe {
        Efer::write(
            EferFlags::SYSTEM_CALL_EXTENSIONS
                | EferFlags::LONG_MODE_ENABLE
                | EferFlags::LONG_MODE_ACTIVE
                | EferFlags::NO_EXECUTE_ENABLE,
        );
    }
}
