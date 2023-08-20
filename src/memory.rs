// tmp import
use crate::println;
use crate::print;

pub fn read_addr_u8(addr: usize) -> u8 {
    let p = addr as *const u8;
    unsafe {return *p}
}
pub fn read_addr_u16(addr: usize) -> u16 {
    let p = addr as *const u16;
    unsafe {return *p}
}
pub fn read_addr_u32(addr: usize) -> u32 {
    let p = addr as *const u32;
    unsafe {return *p}
}
pub fn read_addr_u64(addr: usize) -> u64 {
    let p = addr as *const u64;
    unsafe {return *p}
}

pub fn write_addr_u8(addr: usize, data: u8) {
    let p = addr as *mut u8;
    unsafe {*p = data as u8}
}
pub fn write_addr_u16(addr: usize, data: u16) {
    let p = addr as *mut u16;
    unsafe {*p = data as u16}
}
pub fn write_addr_u32(addr: usize, data: u32) {
    let p = addr as *mut u32;
    unsafe {*p = data as u32}
}
pub fn write_addr_u64(addr: usize, data: u64) {
    let p = addr as *mut u64;
    unsafe {*p = data as u64}
}

#[derive(Debug)]
pub struct Block {
    pub id: u64,
    pub next: u16,
}
impl Block {
    fn new(addr: u64, size: u16) -> Block {
        println!("Creating block {}.", addr);
        write_addr_u64((addr*256) as usize, 0xFu64);
        Block{ id:addr, next:size }
    }
}

pub fn block_free(id: u64) -> bool {
    print!("{} ", id);
    for i in (0..256).step_by(8) {
        if read_addr_u64((id*256+i) as usize) != 0 {
            return false;
        }
    }
    return true;
}

pub fn alloc_block() -> Block {
    for i in 0x100..0x200 {
        if block_free(i) {
            return Block::new(i, 0);
        };
    }
    return Block{id:0,next:0}
}

// pub fn block_free(addr: usize, size: u16) -> bool {
//     for i in (0..size).step_by(8) {
//         if read_addr_u64(addr+(i as usize)) != 0 {
//             return false;
//         }
//     }
//     return true;
// }

// pub fn alloc_block(size: u16) -> Block {
//     let mut i: usize = 0x1000;

//     loop {
//         if block_free(i, size) {
//             write_addr_u64
//             return Block::new(i, size);
//         }
//         else {
//             i += size as usize;
//         };
//     }
// }

#[test_case]
fn test_mem_read_many() {
    for i in 0..256 {
        read_addr_u8(0x1000+i);
    }
}
#[test_case]
fn test_mem_read_var_size() {
    read_addr_u8(0x1000);
    read_addr_u16(0x1000);
    read_addr_u32(0x1000);
    read_addr_u64(0x1000);
}

// #[macro_export]
// macro_rules! read_mem_addr {
//     ($addr:expr,1) => { $crate::memory::read_addr_u8($addr) };
//     ($addr:expr,2) => { $crate::memory::read_addr_u16($addr) };
//     ($addr:expr,4) => { $crate::memory::read_addr_u32($addr) };
//     ($addr:expr,8) => { $crate::memory::read_addr_u64($addr) };
// }