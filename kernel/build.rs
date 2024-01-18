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

use std::process::Command;

fn main() {
    println!("cargo:rustc-link-arg=-Tkernel/linker.ld");
    println!("cargo:rerun-if-changed=kernel/linker.ld");
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=COMMIT_HASH={}", hash);
    println!("cargo:rerun-if-changed=.git/HEAD");
}
