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

use crate::trace;
use crate::LOGGER;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ptr::slice_from_raw_parts;
use core::str::{from_utf8, FromStr};
use limine::request::ModuleRequest;
use spin::Lazy;

static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

pub static INITRD: Lazy<Initrd> = Lazy::new(Initrd::new);

const BLOCK_SIZE: usize = 512;
const EOF_BLOCK: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];

#[derive(Debug)]
struct Header {
    name: String,
    mode: u32,
    uid: u32,
    gid: u32,
    size: u32,
    mtime: u32,
    checksum: String,
    flag: u32,
    linked: String,
    indicator: String,
    version: String,
    username: String,
    group: String,
    major: u32,
    minor: u32,
    prefix: String,
}

#[derive(Debug)]
pub struct File {
    header: Header,
    data: Vec<u8>,
}

fn parse_slice(address: *mut u8, length: usize) -> &'static str {
    let slice = slice_from_raw_parts(address, length);
    unsafe { from_utf8(&*slice).unwrap().trim_end_matches('\0') }
}

fn parse_string(address: *mut u8, length: usize) -> String {
    let slice = parse_slice(address, length);
    slice.to_string()
}

fn parse_integer(address: *mut u8, length: usize) -> u32 {
    let slice = parse_slice(address, length);
    u32::from_str(slice).unwrap_or(0)
}

fn parse_octal(address: *mut u8, length: usize) -> u32 {
    let slice = parse_slice(address, length);
    u32::from_str_radix(slice, 8).unwrap()
}

fn parse_block(address: *mut u8, length: usize) -> &'static [u8] {
    let slice = slice_from_raw_parts(address, length);
    unsafe { slice.as_ref().unwrap() }
}

fn parse_header(address: *mut u8) -> Header {
    let name = parse_string(address, 100);
    let mode = parse_integer(address.wrapping_add(100), 8);
    let uid = parse_octal(address.wrapping_add(108), 8);
    let gid = parse_octal(address.wrapping_add(116), 8);
    let size = parse_octal(address.wrapping_add(124), 12);
    let mtime = parse_octal(address.wrapping_add(136), 12);
    let checksum = parse_string(address.wrapping_add(148), 8);
    let flag = parse_integer(address.wrapping_add(156), 1);
    let linked = parse_string(address.wrapping_add(157), 100);
    let indicator = parse_string(address.wrapping_add(257), 6);
    let version = parse_string(address.wrapping_add(263), 2);
    let username = parse_string(address.wrapping_add(265), 32);
    let group = parse_string(address.wrapping_add(297), 32);
    let major = parse_integer(address.wrapping_add(329), 8);
    let minor = parse_integer(address.wrapping_add(337), 8);
    let prefix = parse_string(address.wrapping_add(345), 155);
    Header {
        name,
        mode,
        uid,
        gid,
        size,
        mtime,
        checksum,
        flag,
        linked,
        indicator,
        version,
        username,
        group,
        major,
        minor,
        prefix,
    }
}

fn parse_data(address: *mut u8, length: u32) -> Vec<u8> {
    let slice = slice_from_raw_parts(address, length as usize);
    unsafe { slice.as_ref().unwrap().to_vec() }
}

pub struct Initrd {
    files: Vec<File>,
}

impl Initrd {
    pub fn new() -> Initrd {
        let module = MODULE_REQUEST.get_response().unwrap().modules()[0];
        let mut address = module.addr();
        let mut files: Vec<File> = Vec::new();
        while parse_block(address, BLOCK_SIZE) != EOF_BLOCK {
            let header = parse_header(address);
            let mut data: Vec<u8> = Vec::new();
            if header.size != 0 {
                address = address.wrapping_add(BLOCK_SIZE);
                data = parse_data(address, header.size);
                address = address
                    .wrapping_add(BLOCK_SIZE * header.size.div_ceil(BLOCK_SIZE as u32) as usize);
            } else {
                address = address.wrapping_add(BLOCK_SIZE);
            }
            let file = File { header, data };
            let log = format!("{:?}", file);
            trace!(log.as_str());
            files.push(file);
        }
        Initrd { files }
    }

    pub fn get_files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn get_data(&self, path: &str) -> &Vec<u8> {
        &self
            .files
            .iter()
            .find(|x| x.header.name == path)
            .unwrap()
            .data
    }
}
